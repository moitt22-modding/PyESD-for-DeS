use std::ops::Deref;
use crate::types::structs::{Buffer, Condition, DLVectorReference, RefValue, Reference, Transition};
use crate::utils::binary_reader::{assert_values, BinaryReader};
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl Transition {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Transition {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "EzStateTransition");

        let version = br.read_uint32();
        assert_values(&version, vec![1, 3]);

        let mut cond_ref = None;
        let mut target_state = None;
        let mut cond_buffer = None;

        if version == 1 {
            cond_ref = Some(Reference::read(br, 0, important_data));
            target_state = Some(Reference::read(br, 0, important_data));
        }
        else {
            cond_buffer = Some(Buffer::read(br, important_data));
        }
        
        let pass_events = DLVectorReference::read(br, important_data);
        
        Transition {
            struct_type,
            version,
            cond_ref,
            target_state,
            cond_buffer,
            pass_events
        }
    }

    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData, transition_ref_id: u32) {
        let target_state_index: u32;

        if self.target_state.is_some() {
            let target_state_val = important_data.ref_id_ref_data_map[&self.target_state.as_ref().unwrap().id].deref();
            if let RefValue::State(state) = target_state_val {
                target_state_index = state.state_index;
            }
            else {
                panic!("Invalid transition target state reference found")
            }
        }
        else {
            target_state_index = important_data.trans_id_state_idx_dict[&transition_ref_id];
        }

        if self.cond_buffer.is_some() {
            Condition::decompile_condition_buffer(&self.cond_buffer.as_ref().unwrap(), tw, important_data);
        }
        else {
            self.cond_ref.as_ref().unwrap().decompile(tw, important_data);
        }

        tw.write_line(format!("            State_{}()", target_state_index));
        
        if self.pass_events.ref_count != 0 {
            tw.write_line("        #PassEvents:".to_string());
            self.pass_events.decompile(tw, important_data);
        }
    }
}