use crate::types::structs::{Header, SharedString};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;

impl Header {
    pub fn compile(important_comp_data: &mut ImportantCompData) -> Header {
        Header {
            magic: "DLSE".to_string(),
            unk04: 2,
            version: 3,
            unk08: Some(26140),
            transition_count: Some(important_comp_data.transition_ids.len() as u32),
            highest_ref_id_plus_one: Some(important_comp_data.get_next_id()),
            string_count: 7,
            strings: vec![
                SharedString {
                    length: 14,
                    str: "EzStateProject".to_string()
                },
                SharedString {
                    length: 8,
                    str: "DLVector".to_string()
                },
                SharedString {
                    length: 10,
                    str: "EzStateMap".to_string()
                },
                SharedString {
                    length: 15,
                    str: "EzStateMapState".to_string()
                },
                SharedString {
                    length: 17,
                    str: "EzStateTransition".to_string()
                },
                SharedString {
                    length: 6,
                    str: "buffer".to_string()
                },
                SharedString {
                    length: 43,
                    str: "EzStateExternalEventT<ES_EVENT_PARAM_NUM_6>".to_string()
                }
            ]
        }
    }

    pub fn write(&self, bw: &mut BinaryWriter) {
        let string_count = self.strings.len() as u16;

        bw.write_string(self.magic.clone());
        bw.write_uint16(self.unk04);
        bw.write_uint16(self.version);

        if self.version == 3 {
            bw.write_uint32(self.unk08.unwrap());
            bw.write_uint32(self.transition_count.unwrap());
            bw.write_uint32(self.highest_ref_id_plus_one.unwrap());
        }

        bw.write_uint16(string_count);

        for shared_str in &self.strings {
            shared_str.write(bw);
        }
    }
}