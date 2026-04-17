use std::sync::Arc;
use crate::types::structs::{DLVectorReference, Event, RefValue, Reference, State, Transition};
use crate::utils::binary_reader::{assert_values};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;
use crate::utils::important_data::ImportantData;
use crate::utils::text_reader::TextReader;

impl State {
    pub fn compile(tr: &mut TextReader, state_idx: u32, important_comp_data: &mut ImportantCompData) -> State {
        let mut is_entry_event = false;
        let mut is_exit_event = false;
        let mut is_transition = false;

        let mut transition_vec: Vec<Reference> = Vec::new();
        let mut entry_event_vec: Vec<Reference> = Vec::new();
        let mut exit_event_vec: Vec<Reference> = Vec::new();

        loop {
            let line = tr.read_line();

            if line == "#EntryEvents:" {
                is_entry_event = true;
                is_exit_event = false;
                is_transition = false;
            }
            else if line == "#ExitEvents:" {
                is_entry_event = false;
                is_exit_event = true;
                is_transition = false;
            }
            else if line == "#Transitions:" {
                is_entry_event = false;
                is_exit_event = false;
                is_transition = true;
            }

            if tr.line_idx == tr.lines.len() {
                break;
            }

            if line.starts_with("defState_") || line.starts_with("defMap_") {
                tr.line_idx -= 1;
                break;
            }
            
            if line.contains("(") || line.contains("if") {
                if is_entry_event || is_exit_event {
                    let event = Event::compile(&line, important_comp_data);
                    
                    let mut arg_buffers = Vec::new();
                    
                    for buffer in event.arg_buffers.as_ref().unwrap() {
                        arg_buffers.push(
                            buffer.data.clone()
                        )
                    }
                    
                    let val;
                    let ref_id;
                    if important_comp_data.event_data_event_id_dict.contains_key(&(event.event_id, arg_buffers.clone())) {
                        val = None;
                        ref_id = important_comp_data.event_data_event_id_dict[&(event.event_id, arg_buffers.clone())]
                    }
                    else {
                        ref_id = important_comp_data.get_next_id();
                        important_comp_data.event_data_event_id_dict.insert((event.event_id, arg_buffers), ref_id);
                        val = Some(Arc::new(RefValue::Event(event)));
                    }

                    if is_entry_event {
                        entry_event_vec.push(
                            Reference {
                                ref_type: 7,
                                id: ref_id,
                                value: val
                            }
                        )
                    }
                    else if is_exit_event {
                        exit_event_vec.push(
                            Reference {
                                ref_type: 7,
                                id: ref_id,
                                value: val
                            }
                        )
                    }
                }

                else if is_transition {
                    important_comp_data.transition_count += 1;
                    let (transition, target_state) = Transition::compile(&line, tr, important_comp_data);

                    let transition_id;
                    if !important_comp_data.cond_data_trans_id_dict.contains_key(&(transition.cond_buffer.as_ref().unwrap().data.clone(), target_state)) {
                        transition_id = important_comp_data.get_next_id();
                        important_comp_data.cond_data_trans_id_dict.insert((transition.cond_buffer.as_ref().unwrap().data.clone(), target_state), transition_id);
                    }
                    else {
                        transition_id = important_comp_data.cond_data_trans_id_dict[&(transition.cond_buffer.as_ref().unwrap().data.clone(), target_state)];
                    }

                    let ref_value;
                    if important_comp_data.transition_ids.contains(&transition_id) {
                        ref_value = None;
                    }
                    else {
                        ref_value = Some(Arc::new(RefValue::Transition(transition)));
                        important_comp_data.state_ref_order.push(target_state);
                        important_comp_data.transition_ids.push(transition_id);
                    }

                    transition_vec.push(
                        Reference {
                            ref_type: 5,
                            id: transition_id,
                            value: ref_value,
                        }
                    )
                }
            }

            if tr.line_idx == tr.lines.len() {
                break;
            }

            if line.starts_with("defState_") || line.starts_with("defMap_") {
                tr.line_idx -= 1;
                break;
            }
        }

        let transitions = DLVectorReference {
            struct_type: 2,
            ref_count: transition_vec.len() as u32,
            item_type: None,
            references: transition_vec
        };

        let entry_events = DLVectorReference {
            struct_type: 2,
            ref_count: entry_event_vec.len() as u32,
            item_type: None,
            references: entry_event_vec
        };

        let exit_events = DLVectorReference {
            struct_type: 2,
            ref_count: exit_event_vec.len() as u32,
            item_type: None,
            references: exit_event_vec
        };

        State {
            struct_type: 4,
            version: 2,
            state_index: state_idx,
            transitions,
            entry_events,
            exit_events
        }
    }

    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let struct_type = important_data.get_type_by_name("EzStateMapState");
        assert_values(&self.version, vec![1, 2]);

        bw.write_uint16(struct_type);
        bw.write_uint32(self.version);

        let state_index = important_data.current_state_index;
        important_data.current_state_index += 1;
        bw.write_uint32(state_index);
        
        self.transitions.write(bw, important_data);
        self.entry_events.write(bw, important_data);
        self.exit_events.write(bw, important_data);
    }
}