use std::collections::HashMap;
use std::env::current_exe;
use std::fs::read_to_string;
use std::sync::Arc;
use crate::types::structs::{Header, RefValue};

pub struct ImportantData {
    pub ref_id_ref_data_map: HashMap<u32, Arc<RefValue>>,
    pub ref_ids_with_data: Vec<u32>,
    pub header_version: u16,
    pub type_table: HashMap<u16, String>,

    pub current_max_id: u32,
    pub old_new_id_dict: HashMap<u32, u32>,

    pub ids_holding_data: Vec<u32>,

    pub current_map_index: u32,
    pub current_state_index: u32,

    pub trans_id_idx_dict: HashMap<u32, usize>,
    pub trans_id_state_idx_dict: HashMap<u32, u32>,

    pub event_defs: HashMap<String, String>,
    pub func_defs: HashMap<String, String>,
}

impl ImportantData {
    pub fn default(header: &Header) -> ImportantData {
        let ref_id_ref_data_map: HashMap<u32, Arc<RefValue>> = HashMap::new();
        let mut type_table: HashMap<u16, String> = HashMap::with_capacity(header.string_count as usize);

        for (idx, shared_str) in header.strings.iter().enumerate() {
            type_table.insert(idx as u16, shared_str.str.clone());
        }

        let header_version = header.version;

        let func_def_path = current_exe()
            .expect("Error reading exe path")
            .parent()
            .expect("Error reading exe parent path")
            .join("FuncDefs.json");

        let func_def_str = read_to_string(func_def_path).expect("Error reading func_def string");
        let func_defs: HashMap<String, String> = serde_json::from_str(&*func_def_str).expect("Error parsing json");

        
        let even_def_path = current_exe()
            .expect("Error reading exe path")
            .parent()
            .expect("Error reading exe parent path")
            .join("EventDefs.json");

        let event_def_str = read_to_string(even_def_path).expect("Error reading func_def string");
        let event_defs: HashMap<String, String> = serde_json::from_str(&*event_def_str).expect("Error parsing json");

        ImportantData {
            ref_id_ref_data_map,
            ref_ids_with_data: Vec::new(),
            type_table,
            header_version,
            current_max_id: 0,
            old_new_id_dict: HashMap::new(),
            ids_holding_data: Vec::new(),
            current_map_index: 0,
            current_state_index: 0,
            trans_id_idx_dict: HashMap::new(),
            trans_id_state_idx_dict: HashMap::new(),
            func_defs,
            event_defs
        }
    }

    pub fn get_type_name(&self, type_id: u16) -> String {
        self.type_table[&(type_id - 1)].clone()
    }

    pub fn get_type_by_name(&self, type_name: &str) -> u16 {
        for (type_id, name) in &self.type_table {
            if *name == type_name.to_string() {
                return *type_id + 1;
            }
        }

        panic!("Invalid type table found in get_type_by_name");
    }

    pub fn should_read_data(&mut self, id: u32) -> bool {
        if self.ref_ids_with_data.contains(&id) {
            false
        }
        else {
            self.ref_ids_with_data.push(id);
            true
        }
    }

    pub fn should_write_data(&mut self, id: u32) -> bool {
        if self.ids_holding_data.contains(&id) {
            return false
        }
        else {
            self.ids_holding_data.push(id);
            return true
        }
    }
}