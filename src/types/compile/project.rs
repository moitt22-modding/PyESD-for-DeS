use std::sync::Arc;
use crate::types::structs::{DLVectorReference, Map, Project, RefValue, Reference};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;
use crate::utils::important_data::ImportantData;
use crate::utils::text_reader::TextReader;

impl Project {
    pub fn compile(tr: &mut TextReader, important_comp_data: &mut ImportantCompData) -> Project {
        let mut map_vec: Vec<Reference> = Vec::new();


        loop {
            let line = tr.read_line();
            if line.starts_with("defMap_") {
                let map_idx = line
                    .replace("defMap_", "")
                    .replace("():", "")
                    .parse::<u32>()
                    .expect("Error parsing str to int");

                let map = Map::compile(tr, map_idx, important_comp_data);
                
                let map_ref = Reference {
                    ref_type: 3,
                    id: important_comp_data.get_next_id(),
                    value: Some(Arc::new(RefValue::Map(map)))
                };
                
                map_vec.push(map_ref);
            }

            if tr.line_idx == tr.lines.len() {
                break;
            }
        }
        

        let maps = DLVectorReference {
            struct_type: 2,
            ref_count: map_vec.len() as u32,
            item_type: None,
            references: map_vec
        };

        Project {
            struct_type: 1,
            version: 3,
            maps
        }
    }

    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let struct_type = important_data.get_type_by_name("EzStateProject");
        bw.write_uint16(struct_type);
        
        bw.write_uint32(2);
        self.maps.write(bw, important_data);
    }
}