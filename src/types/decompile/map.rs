use crate::types::structs::{DLVectorReference, Map, Reference};
use crate::utils::binary_reader::{assert_values, BinaryReader};
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl Map {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Map {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "EzStateMap");

        let version = br.read_uint32();
        assert_values::<u32>(&version, vec![1, 2]);

        let map_index = br.read_uint32();
        let initial_state = Reference::read(br, 0, important_data);

        let states = DLVectorReference::read(br, important_data);

        let mut transitions = None;

        if version == 2 {
            transitions = Some(DLVectorReference::read(br, important_data));
            
            for (idx, transition_ref) in transitions.as_ref().unwrap().references.iter().enumerate() {
                important_data.trans_id_idx_dict.insert(transition_ref.id, idx);
            }
        }

        Map {
            struct_type,
            version,
            map_index,
            initial_state,
            states,
            transitions
        }
    }
    
    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        let map_def_str = format!("def Map_{}():", self.map_index);
        tw.write_line(map_def_str);
        
        self.states.decompile(tw, important_data);
    }
}