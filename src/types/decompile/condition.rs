use std::collections::HashMap;
use crate::types::structs::{Buffer, Condition, Evaluator};
use crate::utils::binary_reader::BinaryReader;
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl Condition {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Condition {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "EzState::detail::EzStateCondition");
        
        let version = br.read_uint32();
        assert_eq!(version, 1);
        
        let evaluator = Evaluator::read(br, important_data);
        
        Condition {
            struct_type,
            version,
            evaluator
        }
    }

    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        let buffer = &self.evaluator.buffer;
        Self::decompile_condition_buffer(buffer, tw, important_data);
    }

    pub fn decompile_condition_buffer(buffer: &Buffer, tw: &mut TextWriter, important_data: &ImportantData) {
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

        let mut condition_str = String::from("        if ");

        let mut cond_vec = Vec::new();

        let mut counter = 0;
        while counter < buffer.data.len() {
            if buffer.data[counter] <= 0x7F {
                cond_vec.push((buffer.data[counter] - 64).to_string());
                counter += 1;
            }

            else if buffer.data[counter] == 0x82 {
                let sized_slice: [u8; 4] = (&buffer.data[counter + 1 .. counter + 5]).try_into().expect("Error converting to owned slice");
                let int_val = u32::from_le_bytes(sized_slice);

                cond_vec.push(int_val.to_string());
                counter += 5;
            }

            else if buffer.data[counter] >= 0x84 && buffer.data[counter] <= 0x8A {
                let arg_amount = buffer.data[counter] - 0x84;

                let mut args = Vec::new();
                for _ in 0..arg_amount {
                    args.push(cond_vec.pop().expect("Error popping the stack"));
                }
                args.reverse();

                let id = cond_vec.pop().expect("Error popping the stack");

                let mut func_str;
                if important_data.func_defs.contains_key(&id) {
                    func_str = important_data.func_defs[&id].clone();
                }
                else {
                    panic!("Invalid function id found {}", id);
                }

                func_str.push('(');
                for (i, arg) in args.iter().enumerate() {
                    func_str.push_str(&*arg);
                    if i != args.len() - 1{
                        func_str.push(',');
                    }
                }

                func_str.push(')');

                cond_vec.push(func_str);
                counter += 1;
            }

            else if operators.contains_key(&buffer.data[counter]) {
                let operator = &operators[&buffer.data[counter]];
                if operator != "not" {
                    let operand1 = cond_vec.pop().expect("Error popping the stack");
                    let operand2 = cond_vec.pop().expect("Error popping the stack");

                    cond_vec.push(format!("{} {} {}", operand1, operand2, operator));
                }
                else {
                    let operand = cond_vec.pop().expect("Error popping the stack");
                    cond_vec.push(format!("{} {}", operand, operator));
                }

                counter += 1;
            }

            else if buffer.data[counter] == 0xA1 {
                break;
            }

            else {
                println!("{}", buffer.data[counter]);
            }
        }

        let condition = &cond_vec[0];
        if condition == "1" {
            tw.write_line("        if True:".to_string());
            return;
        }

        let mut node_stack: Vec<NodeType> = Vec::new();
        let tokens = condition.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

        for token in tokens {
            if token == "==" || token == "!=" || token == "<=" || token == ">=" || token == "<" || token == ">" || token == "+" || token == "-" || token == "*" || token == "/" || token == "and" || token == "or" {
                let arg1 = node_stack.pop().expect("Error popping the stack");
                let arg2 = node_stack.pop().expect("Error popping the stack");

                if token == "==" || token == "!=" || token == "<=" || token == ">=" || token == "<" || token == ">"  {
                    node_stack.push(
                        NodeType::Eq(
                            Node {
                                children: vec![arg1, arg2],
                                operator: token
                            }
                        )
                    )
                }
                else if token == "+" || token == "-" {
                    node_stack.push(
                        NodeType::LowMath(
                            Node {
                                children: vec![arg1, arg2],
                                operator: token
                            }
                        )
                    )
                }
                else if token == "*" || token == "/" {
                    node_stack.push(
                        NodeType::HighMath(
                            Node {
                                children: vec![arg1, arg2],
                                operator: token
                            }
                        )
                    )
                }
                else if token == "and" {
                    node_stack.push(
                        NodeType::And(
                            Node {
                                children: vec![arg1, arg2],
                                operator: token
                            }
                        )
                    )
                }
                else if token == "or" {
                    node_stack.push(
                        NodeType::Or(
                            Node {
                                children: vec![arg1, arg2],
                                operator: token
                            }
                        )
                    )
                }
            }
            else if token == "not" {
                let arg = node_stack.pop().expect("Error popping the stack");

                node_stack.push(
                    NodeType::Not(
                        Node {
                            children: vec![arg],
                            operator: token
                        }
                    )
                )
            }
            else {
                node_stack.push(
                    NodeType::Value(token),
                );
            }
        }

        let ast = &node_stack[0];

         match ast {
             NodeType::HighMath(node) => condition_str.push_str(&*eval_high_math_node(node)),
             NodeType::LowMath(node) => condition_str.push_str(&*eval_low_math_node(node)),
             NodeType::Eq(node) => condition_str.push_str(&*eval_eq_node(node)),
             NodeType::And(node) => condition_str.push_str(&*eval_and_node(node)),
             NodeType::Or(node) => condition_str.push_str(&*eval_or_node(node)),
             NodeType::Not(node) => condition_str.push_str(&*eval_not_node(node)),
             NodeType::Value(value) => condition_str.push_str(&*value),
         }

        condition_str.push(':');
        tw.write_line(condition_str);
    }
}

fn eval_high_math_node(parent_node: &Node) -> String {
    let children = &parent_node.children;
    let mut node_str = String::new();

    for i in 0..=1 {
        match &children[i] {
            NodeType::HighMath(node) => node_str.push_str(&*eval_high_math_node(&node)),
            NodeType::LowMath(node) => node_str.push_str(&*format!("({})", eval_low_math_node(&node))),
            NodeType::Value(value) => node_str.push_str(&*value),
            _ => panic!("Invalid high math node found")
        }
        if i == 0 {
            node_str.push_str(&*format!(" {} ", parent_node.operator));
        }
    }

   node_str
}

fn eval_low_math_node(parent_node: &Node) -> String {
    let children = &parent_node.children;
    let mut node_str = String::new();

    for i in 0..=1 {
        match &children[i] {
            NodeType::HighMath(node) => node_str.push_str(&eval_high_math_node(&node)),
            NodeType::LowMath(node) => node_str.push_str(&*eval_low_math_node(&node)),
            NodeType::Value(value) => node_str.push_str(&*value),
            _ => panic!("Invalid low math node found")
        }
        if i == 0 {
            node_str.push_str(&*format!(" {} ", parent_node.operator));
        }
    }

    node_str
}

fn eval_eq_node(parent_node: &Node) -> String {
    let children = &parent_node.children;
    let mut node_str = String::new();

    for i in 0..=1 {
        match &children[i] {
            NodeType::HighMath(node) => node_str.push_str(&eval_high_math_node(node)),
            NodeType::LowMath(node) => node_str.push_str(&*eval_low_math_node(node)),
            NodeType::Value(value) => node_str.push_str(&**value),
            _ => panic!("Invalid Eq node found: {:?}", parent_node),
        }
        if i == 0 {
            node_str.push_str(&*format!(" {} ", parent_node.operator));
        }
    }

    node_str
}

fn eval_and_node(parent_node: &Node) -> String {
    let children = &parent_node.children;
    let mut node_str = String::new();

    for i in 0..=1 {
        match &children[i] {
            NodeType::Eq(node) => node_str.push_str(&*eval_eq_node(node)),
            NodeType::And(node) => node_str.push_str(&*eval_and_node(node)),
            NodeType::Not(node) => node_str.push_str(&*eval_not_node(node)),
            NodeType::Or(node) => node_str.push_str(&*format!("({})", eval_or_node(node))),
            NodeType::Value(value) => node_str.push_str(&**value),
            _ => panic!("Invalid And node found"),
        }
        if i == 0 {
            node_str.push_str(&*format!(" {} ", parent_node.operator));
        }
    }

    node_str
}

fn eval_or_node(parent_node: &Node) -> String {
    let children = &parent_node.children;
    let mut node_str = String::new();

    for i in 0..=1 {
        match &children[i] {
            NodeType::Eq(node) => node_str.push_str(&*eval_eq_node(node)),
            NodeType::And(node) => node_str.push_str(&*eval_and_node(node)),
            NodeType::Not(node) => node_str.push_str(&*eval_not_node(node)),
            NodeType::Or(node) => node_str.push_str(&*eval_or_node(node)),
            NodeType::Value(value) => node_str.push_str(&**value),
            _ => panic!("Invalid Or node found"),
        }
        if i == 0 {
            node_str.push_str(&*format!(" {} ", parent_node.operator));
        }
    }

    node_str
}

fn eval_not_node(parent_node: &Node) -> String {
    let children = &parent_node.children;
    let mut node_str = String::new();

    node_str.push_str(&*format!(" {} ", parent_node.operator));

    match &children[0] {
        NodeType::Eq(node) => node_str.push_str(&*eval_eq_node(node)),
        NodeType::And(node) => node_str.push_str(&*format!("({})", eval_and_node(node))),
        NodeType::Not(node) => node_str.push_str(&*eval_not_node(node)),
        NodeType::Or(node) => node_str.push_str(&*format!("({})", eval_or_node(node))),
        NodeType::Value(value) => node_str.push_str(&**value),
        _ => panic!("Invalid Not node found"),
    }

    node_str
}

#[derive(Debug)]
struct Node {
    pub children: Vec<NodeType>,
    pub operator: String
}

#[derive(Debug)]
enum NodeType {
    And(Node),
    Or(Node),
    Not(Node),
    HighMath(Node),
    LowMath(Node),
    Eq(Node),
    Value(String)
}