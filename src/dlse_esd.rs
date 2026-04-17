use std::fs::{exists, rename};
use std::ops::Deref;
use crate::types::structs::{Header, Project, RefValue, Reference};
use crate::utils::binary_reader::BinaryReader;
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;
use crate::utils::important_data::ImportantData;
use crate::utils::text_reader::TextReader;
use crate::utils::text_writer::TextWriter;

pub struct DlseEsd {
    pub header: Header,
    pub project: Project,
    pub unkx00: u64,
    pub state_refs: Option<Vec<Reference>>,
    pub important_data: ImportantData
}

impl DlseEsd {
     fn read(path: &str) -> DlseEsd {
        let mut br = BinaryReader::new(path.to_string());

        let header = Header::read(&mut br);

        let mut important_data = ImportantData::default(&header);

        let project = Project::read(&mut br, &mut important_data);

        let unkx00 = br.read_uint64();

        let mut state_refs = None;

        if header.version == 3 {
            let mut state_ref_vec = Vec::with_capacity(header.transition_count.unwrap() as usize);
            for _ in 0..header.transition_count.unwrap() {
                state_ref_vec.push(Reference::read(&mut br, 0, &mut important_data))
            }

            for (idx, state_ref) in state_ref_vec.iter().enumerate() {
                for (trans_id, trans_idx) in important_data.trans_id_idx_dict.clone() {
                    if trans_idx == idx {
                        if let RefValue::State(state) = important_data.ref_id_ref_data_map[&state_ref.id].deref() {
                            important_data.trans_id_state_idx_dict.insert(trans_id, state.state_index);
                        }
                    }
                }

            }

            state_refs = Some(state_ref_vec);
        }

        DlseEsd {
            header,
            project,
            unkx00,
            state_refs,
            important_data
        }
    }

    pub fn decompile(path: &str) {
        let esd = DlseEsd::read(path);

        let mut decompiled_file_path = path.to_string();
        decompiled_file_path.remove(path.len() - 1);
        decompiled_file_path.remove(path.len() - 2);
        decompiled_file_path.remove(path.len() - 3);

        decompiled_file_path.push_str("esd.py");


        let mut tw = TextWriter::new(&*decompiled_file_path);

        esd.project.decompile(&mut tw, &esd.important_data);

        tw.finish();
    }
}
impl DlseEsd {
    pub fn compile(src_path: &str) {
        let dest_path = src_path.replace(".esd.py", ".esd");
        let bak_path = dest_path.clone() + ".bak";

        if exists(&dest_path).expect("Error checking if file exists") {
            rename(&dest_path, bak_path).expect("Error creating backup")
        }

        let mut tr = TextReader::new(src_path);

        let mut important_comp_data = ImportantCompData::new();

        let project = Project::compile(&mut tr, &mut important_comp_data);
        let header = Header::compile(&mut important_comp_data);
        let important_data = ImportantData::default(&header);

        let mut state_refs = Vec::new();

        for state_idx in important_comp_data.state_ref_order {
            let state_id = important_comp_data.state_idx_state_id_dict[&state_idx];

            state_refs.push(
                Reference {
                    ref_type: 4,
                    id: state_id,
                    value: None
                }
            )
        }

        let mut esd = DlseEsd {
            header,
            project,
            unkx00: 0x2695380AB4E4C27,
            state_refs: Some(state_refs),
            important_data
        };

        esd.write(&*dest_path);
    }

    pub fn write(&mut self, path: &str) {
        let mut bw = BinaryWriter::new(path);

        self.header.write(&mut bw);
        self.project.write(&mut bw, &mut self.important_data);
        bw.write_uint64(self.unkx00);
        
        if self.header.version == 3 {
            for state_ref in self.state_refs.as_ref().unwrap() {
                state_ref.write(&mut bw, 0, &mut self.important_data)
            }
        }

        bw.finish();
    }
}