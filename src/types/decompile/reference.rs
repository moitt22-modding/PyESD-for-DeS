use std::ops::Deref;
use std::sync::Arc;
use crate::types::structs::{Condition, Event, Map, RefValue, Reference, State, Transition};
use crate::utils::binary_reader::{BinaryReader};
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl Reference {
    pub fn read(br: &mut BinaryReader, type_id: u16, important_data: &mut ImportantData) -> Reference {
        let ref_type;

        if type_id == 0 {
            ref_type = br.read_uint16();
        }
        else {
            ref_type = type_id;
        }

        let id = br.read_uint32();

        if !important_data.should_read_data(id) {
            return Reference {
                ref_type,
                id,
                value: None
            }
        }

        let type_string = important_data.get_type_name(ref_type);

        let value: RefValue;

        match &*type_string {
            "EzStateMap" => value = RefValue::Map(Map::read(br, important_data)),
            "EzStateMapState" => value = RefValue::State(State::read(br, important_data)),
            "EzStateExternalEvent" => value = RefValue::Event(Event::read(br, important_data)),
            "EzStateExternalEventT<ES_EVENT_PARAM_NUM_6>" => value = RefValue::Event(Event::read(br, important_data)),
            "EzStateTransition" => value = RefValue::Transition(Transition::read(br, important_data)),
            "EzState::detail::EzStateCondition" => value = RefValue::Condition(Condition::read(br, important_data)),
            invalid_str => panic!("Invalid type string found while reading references: {}", invalid_str)
        }

        let map_val = Arc::new(value);
        let ret_val = map_val.clone();
        
        important_data.ref_id_ref_data_map.insert(id, map_val);

        Reference {
            ref_type,
            id,
            value: Some(ret_val)
        }
    }

    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        let value;

        if let Some(val) = &self.value {
            value = val.deref()
        }
        else {
            value = important_data.ref_id_ref_data_map[&self.id].deref();
        }

        match &*important_data.get_type_name(self.ref_type) {
            "EzStateMap" => if let RefValue::Map(map) = value {
                map.decompile(tw, important_data)
            },
            "EzStateMapState" => if let RefValue::State(state) = value {
                state.decompile(tw, important_data)
            },
            "EzStateExternalEvent" => if let RefValue::Event(event) = value {
                event.decompile(tw, important_data)
            },
            "EzStateExternalEventT<ES_EVENT_PARAM_NUM_6>" => if let RefValue::Event(event) = value {
                event.decompile(tw, important_data)
            },
            "EzStateTransition" => if let RefValue::Transition(transition) = value {
                transition.decompile(tw, important_data, self.id);
            }
            "EzState::detail::EzStateCondition" => if let RefValue::Condition(condition) = value {
                condition.decompile(tw, important_data)
            },
            invalid_str => panic!("Invalid type string found while decompiling references: {}", invalid_str)
        }
    }
}