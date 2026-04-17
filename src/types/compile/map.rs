use std::sync::Arc;
use crate::types::structs::{DLVectorReference, Map, RefValue, Reference, State};
use crate::utils::binary_reader::{assert_values};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;
use crate::utils::important_data::ImportantData;
use crate::utils::text_reader::TextReader;

impl Map {
    pub fn compile(tr: &mut TextReader, map_idx: u32, important_comp_data: &mut ImportantCompData) -> Map {
        let mut initial_state_ref: Reference = Reference {ref_type: 0, id: 0, value: None};
        let mut state_vec: Vec<Reference> = Vec::new();

        loop {
            if tr.line_idx == tr.lines.len() {
                break;
            }
            let line = tr.read_line();

            if line.starts_with("defState_") {
                let state_idx = line
                    .replace("defState_", "")
                    .replace("():", "")
                    .parse::<u32>()
                    .expect("Error parsing str to int");

                if state_idx == 0 {
                    let initial_state = State::compile(tr, state_idx, important_comp_data);
                    let id = important_comp_data.get_next_id();

                    important_comp_data.state_idx_state_id_dict.insert(state_idx, id);

                    initial_state_ref = Reference {
                        ref_type: 4,
                        id,
                        value: Some(Arc::new(RefValue::State(initial_state)))
                    };
                }
                else {
                    let state = State::compile(tr, state_idx, important_comp_data);
                    let id = important_comp_data.get_next_id();

                    let state_ref = Reference {
                        ref_type: 4,
                        id,
                        value: Some(Arc::new(RefValue::State(state)))
                    };

                    important_comp_data.state_idx_state_id_dict.insert(state_idx, id);

                    state_vec.push(state_ref);
                }
            }

            if line.starts_with("defMap_") {
                tr.line_idx -= 1;
                break;
            }
            if tr.line_idx == tr.lines.len() {
                break;
            }
        }

        state_vec.insert(
            0,
            Reference {
                ref_type: 4,
                id: initial_state_ref.id,
                value: None
            }
        );

        let states = DLVectorReference {
            struct_type: 2,
            ref_count: state_vec.len() as u32,
            item_type: None,
            references: state_vec
        };

        let mut transition_refs: Vec<Reference> = Vec::new();

        for id in &important_comp_data.transition_ids {
            transition_refs.push(
                Reference {
                    ref_type: 5,
                    id: *id,
                    value: None
                }
            )
        }

        let transitions = Some(DLVectorReference {
            struct_type: 2,
            ref_count: transition_refs.len() as u32,
            item_type: None,
            references: transition_refs
        });

        Map {
            struct_type: 3,
            version: 2,
            map_index: map_idx,
            initial_state: initial_state_ref,
            states,
            transitions
        }
    }

    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let struct_type = important_data.get_type_by_name("EzStateMap");
        assert_values(&self.version, vec![1, 2]);

        bw.write_uint16(struct_type);
        bw.write_uint32(self.version);

        let map_index = important_data.current_map_index + 1;
        important_data.current_map_index += 1;
        bw.write_uint32(map_index);
        
        self.initial_state.write(bw, 0, important_data);
        self.states.write(bw, important_data);
        
        if self.version == 2 {
            self.transitions.as_ref().unwrap().write(bw, important_data);
        }
    }
}