use std::collections::HashMap;
use std::env::current_exe;
use std::fs::read_to_string;
use crate::utils::read_def;

pub struct ImportantCompData {
    pub highest_id: u32,

    pub cond_data_trans_id_dict: HashMap<(Vec<u8>, u32), u32>,
    pub event_data_event_id_dict: HashMap<(u32, Vec<Vec<u8>>), u32>,

    pub transition_ids: Vec<u32>,
    pub transition_count: u32,
    pub state_ref_order: Vec<u32>,
    pub state_idx_state_id_dict: HashMap<u32, u32>,

    pub func_defs: HashMap<String, String>,
    pub event_defs: HashMap<String, String>
}

impl ImportantCompData {
    pub fn new() -> ImportantCompData {
        let func_def_path = current_exe()
            .expect("Error reading exe path")
            .parent()
            .expect("Error reading exe parent path")
            .join("FuncDefs.json");

        let func_defs: HashMap<String, String> = read_def(func_def_path.to_str().expect("Error reading path").to_string());


        let even_def_path = current_exe()
            .expect("Error reading exe path")
            .parent()
            .expect("Error reading exe parent path")
            .join("EventDefs.json");

        let event_defs: HashMap<String, String> = read_def(even_def_path.to_str().expect("Error reading path").to_string());

        ImportantCompData {
            highest_id: 0,
            cond_data_trans_id_dict: HashMap::new(),
            event_data_event_id_dict: HashMap::new(),
            transition_ids: Vec::new(),
            transition_count: 0,
            state_ref_order: Vec::new(),
            state_idx_state_id_dict: HashMap::new(),
            func_defs,
            event_defs,
        }
    }

    pub fn get_next_id(&mut self) -> u32 {
        self.highest_id += 1;
        self.highest_id
    }
}