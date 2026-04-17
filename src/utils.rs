use std::collections::HashMap;
use std::fs::read_to_string;

pub mod binary_reader;
pub mod important_data;
pub mod binary_writer;
pub mod text_writer;
pub mod text_reader;
pub mod important_comp_data;

pub fn read_def(path: String) -> HashMap<String, String> {
    let file_string = read_to_string(path).expect("Error reading def");
    let file_string = file_string.replace("{", "").replace("}", "").replace("\"", "").replace(",", "").replace("\r", "");
    
    let lines = file_string.split("\n").map(|s|s.to_string()).collect::<Vec<String>>();
    let mut map = HashMap::new();
    
    for line in lines {
        if line != " " && line != "" {
            print!("{}", line);
            let parts = line.split(":").map(|s|s.to_string()).collect::<Vec<String>>();
            println!("{:?}", parts);
            map.insert(parts[0].replace(" ", "").clone(), parts[1].replace(" ", "").clone());
        }
    }
    
    map
}