use std::ops::Deref;
use crate::types::structs::{RefValue, Reference};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_data::ImportantData;

impl Reference {
    pub fn write(&self, bw: &mut BinaryWriter, type_id: u16, important_data: &mut ImportantData) {
        let local_type;
        if type_id == 0 {
            bw.write_uint16(self.ref_type);
            local_type = self.ref_type;
        }
        else {
            local_type = type_id
        }

        let local_type_name = important_data.get_type_name(local_type);

        let mut new_id = important_data.current_max_id + 1;
        if important_data.old_new_id_dict.contains_key(&self.id) {
            new_id = important_data.old_new_id_dict[&self.id]
        }
        else {
            important_data.old_new_id_dict.insert(self.id, new_id);
            important_data.current_max_id += 1;
        }

        bw.write_uint32(new_id);

        if !important_data.should_write_data(new_id) {
            return;
        }

        match &*local_type_name {
            "EzStateMap" => if let RefValue::Map(map) = &self.value.as_ref().unwrap().deref() {
                map.write(bw, important_data)
            },
            "EzStateMapState" => if let RefValue::State(state) = &self.value.as_ref().unwrap().deref() {
                state.write(bw, important_data)
            },
            "EzStateExternalEvent" => if let RefValue::Event(event) = &self.value.as_ref().unwrap().deref() {
                event.write(bw, important_data)
            },
            "EzStateTransition" => if let RefValue::Transition(transition) = &self.value.as_ref().unwrap().deref() {
                transition.write(bw, important_data)
            }
            "EzStateExternalEventT<ES_EVENT_PARAM_NUM_6>" => if let RefValue::Event(event) = &self.value.as_ref().unwrap().deref() {
                event.write(bw, important_data)
            },
            "EzState::detail::EzStateCondition" => if let RefValue::Condition(condition) = &self.value.as_ref().unwrap().deref() {
                condition.write(bw, important_data)
            },
            _ => panic!("Invalid type string found while writing references: {}", local_type_name)
        }
    }
}