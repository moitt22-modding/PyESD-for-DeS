use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use crate::types::structs::{Buffer, DLVectorReference, Event, RefValue, Reference, Transition};
use crate::utils::binary_reader::{assert_values};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;
use crate::utils::important_data::ImportantData;
use crate::utils::text_reader::TextReader;

impl Transition {
    pub fn compile(condition: &String, tr: &mut TextReader, important_comp_data: &mut ImportantCompData) -> (Transition, u32) {
        let mut operators: HashMap<u8, String> = HashMap::new();
        operators.insert(0x8C, "+".to_string());
        operators.insert(0x8D, "N".to_string());
        operators.insert(0x8E, "-".to_string());
        operators.insert(0x8F, "*".to_string());
        operators.insert(0x90, "/".to_string());
        operators.insert(0x91, "<=".to_string());
        operators.insert(0x92, ">=".to_string());
        operators.insert(0x93, "<".to_string());
        operators.insert(0x94, ">".to_string());
        operators.insert(0x95, "==".to_string());
        operators.insert(0x96, "!=".to_string());
        operators.insert(0x98, "and".to_string());
        operators.insert(0x99, "or".to_string());
        operators.insert(0x9A, "not".to_string());

        let state_call = tr.read_line();
        let target_state_idx = state_call.replace("State_", "").replace("()", "").parse().expect("Error parsing str to int");

        let mut pass_event_vec: Vec<Reference> = Vec::new();
        if tr.line_idx != tr.lines.len() {
            let line = tr.read_line();
            if line == "#PassEvents:" || line == "PassEvents:" {
                loop {
                    if tr.line_idx == tr.lines.len() {
                        break;
                    }
                    let line = tr.read_line();

                    if line.starts_with("defState_") || line.starts_with("defMap_") || line.starts_with("if") {
                        tr.line_idx -= 1;
                        break;
                    }
                    else {
                        let event = Event::compile(&line, important_comp_data);

                        let event_ref = Reference {
                            ref_type: 7,
                            id: important_comp_data.get_next_id(),
                            value: Some(Arc::new(RefValue::Event(event)))
                        };

                        pass_event_vec.push(event_ref);
                    }
                }
            }
            else {
                tr.line_idx -= 1;
            }
        }

        let pass_events = DLVectorReference {
            struct_type: 2,
            ref_count: pass_event_vec.len() as u32,
            item_type: None,
            references: pass_event_vec
        };

        let condition = condition
            .replace("if", "if ")
            .replace("and", " and ")
            .replace("or", " or ")
            .replace("not", " not ");

        let postfix_condition = to_postfix(tokenize(condition), important_comp_data);

        let mut bytes: Vec<u8> = Vec::new();

        for cond in &postfix_condition {
            let cond = cond.replace(":", "");
            if cond == "True" {
                bytes = vec![65];
                break;
            }
            let cond = cond.clone();
            if operators.values().collect::<Vec<&String>>().contains(&&cond) {
                let mut operator: u8 = 0;
                for (byte, symbol) in &operators {
                    if *symbol == cond {
                        operator = *byte;
                    }
                }

                bytes.push(operator);
            }
            else if important_comp_data.func_defs.values().collect::<Vec<&String>>().contains(&&cond) {
                let func_name = cond.split("(").collect::<Vec<&str>>()[0].to_string();
                let mut func_id = 7;

                for (id, name) in &important_comp_data.func_defs {
                    if *name == func_name {
                        func_id = id.parse::<u8>().expect("Error parsing str to byte")
                    }
                }
                if func_id == 7 {
                    panic!("Invalid func name found")
                }
                func_id += 64;
                bytes.push(func_id);
            }
            else if cond.contains("(") {
                let arg_amount = cond.replace("(", "").replace(")", "").parse::<u8>().expect("Error parsing str to int");
                let call_byte = 0x84 + arg_amount;
                bytes.push(call_byte);
            }
            else {
                if cond == "" {
                    continue;
                }
                let cond_val = cond.parse::<i64>().expect("Error parsing str to int");
                if cond_val + 64 <= 0x7F {
                    bytes.push((cond_val + 64) as u8)
                }
                else {
                    bytes.push(0x82);
                    bytes.append(&mut (cond_val as u32).to_le_bytes().to_vec());
                }
            }
        }

        bytes.push(0xA1);

        let cond_buffer = Buffer {
            struct_type: 6,
            length: bytes.len() as u32,
            data: bytes
        };

        (Transition {
            struct_type: 5,
            version: 3,
            target_state: None,
            cond_buffer: Some(cond_buffer),
            cond_ref: None,
            pass_events
        }, target_state_idx)
    }

    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let struct_type = important_data.get_type_by_name("EzStateTransition");
        bw.write_uint16(struct_type);

        assert_values(&self.version, vec![1, 3]);
        bw.write_uint32(self.version);

        if self.version == 1 {
            self.cond_ref.as_ref().unwrap().write(bw, 0, important_data);
            self.target_state.as_ref().unwrap().write(bw, 0, important_data);
        }
        else {
            self.cond_buffer.as_ref().unwrap().write(bw, important_data, "buffer");
        }

        self.pass_events.write(bw, important_data);
    }
}


fn tokenize(condition: String) -> Vec<String> {
    let condition_char_vec = condition
        .replace("not", " not ")
        .replace("and", " and ")
        .replace("or", " or ")
        .replace("==", " == ")
        .replace("!=", " != ")
        .replace("<=", " <= ")
        .replace(">=", " >= ")
        .replace("+", " + ")
        .replace("-", " - ")
        .replace("*", " * ")
        .replace("/", " / ")
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace(",", " ")
        .replace(":", "")
        .chars().collect::<Vec<char>>();

    let mut condition_tokenized_vec = Vec::new();

    for (idx, char) in condition_char_vec.iter().enumerate() {
        if *char == '<' && condition_char_vec[idx + 1] != '=' || *char == '>' && condition_char_vec[idx + 1] != '=' {
            condition_tokenized_vec.push(' ');
            condition_tokenized_vec.push(*char);
            condition_tokenized_vec.push(' ');
        } else {
            condition_tokenized_vec.push(*char);
        }
    }

    let condition = condition_tokenized_vec
        .iter().map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");


    let tokens = condition.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut filtered_tokens = Vec::new();

    for token in tokens {
        if token != "" {
            filtered_tokens.push(token)
        }
    }

    filtered_tokens
}

fn to_postfix(tokens: Vec<String>, important_comp_data: &ImportantCompData) -> Vec<String> {
    let mut operators: HashMap<String, u8> = HashMap::new();
    operators.insert("==".to_string(), 3);
    operators.insert("!=".to_string(), 3);
    operators.insert("<=".to_string(), 3);
    operators.insert(">=".to_string(), 3);
    operators.insert("<".to_string(), 3);
    operators.insert(">".to_string(), 3);
    operators.insert("and".to_string(), 1);
    operators.insert("or".to_string(), 0);
    operators.insert("not".to_string(), 2);
    operators.insert("+".to_string(), 4);
    operators.insert("-".to_string(), 4);
    operators.insert("*".to_string(), 5);
    operators.insert("/".to_string(), 5);
    let tokens = tokens;

    let mut output = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    let mut values_amount_in_parentheses = vec![0];
    let mut is_function_parentheses = vec![false];

    for token in tokens {
        if token.parse::<i64>().is_ok() {
            output.push(token);
            let len = values_amount_in_parentheses.len() - 1;
            values_amount_in_parentheses[len] += 1;
        }
        else if important_comp_data.func_defs.values().collect::<Vec<&String>>().contains(&&token) || token == "True" {
            output.push(token);
            is_function_parentheses.push(true);
            let len = values_amount_in_parentheses.len() - 1;
            values_amount_in_parentheses[len] += 1;
        }
        else if operators.contains_key(&token) {
            let len = values_amount_in_parentheses.len() - 1;
            values_amount_in_parentheses[len] -= 1;
            if stack.len() != 0 {
                while operators.contains_key(&stack[stack.len() - 1]) {
                    if operators[&stack[stack.len() - 1]] >= operators[&token] {
                        output.push(stack.pop().expect("Error popping the stack"));
                        if stack.len() == 0 {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
            }
            stack.push(token);
        }
        else if token == "(" {
            stack.push(token);
            values_amount_in_parentheses.push(0);
        }
        else if token == ")" {
            while &stack[stack.len() - 1] != "(" {
                output.push(stack.pop().expect("Error popping the stack"));
            }
            stack.pop();
            if is_function_parentheses.pop().expect("Error popping the stack") {
                output.push(format!("({})", values_amount_in_parentheses.pop().expect("Error popping the stack")))
            }
        }
        else if token == "if" || token == "," {  }
        else {
            panic!("Invalid condition part found: {:?}", token);
        }
    }

    while stack.len() != 0 {
        output.push(stack.pop().expect("Error popping the stack"))
    }

    output
}