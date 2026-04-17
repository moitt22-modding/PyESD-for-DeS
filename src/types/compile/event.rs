use crate::types::structs::{Buffer, Event};
use crate::utils::binary_reader::{assert_values};
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_comp_data::ImportantCompData;
use crate::utils::important_data::ImportantData;
impl Event {
    pub fn compile(event_str: &String, important_comp_data: &mut ImportantCompData) -> Event {
        let event_name: String = event_str.split("(").collect::<Vec<&str>>()[0].to_string();

        let mut event_id = 0;

        if !important_comp_data.event_defs.values().collect::<Vec<&String>>().contains(&&event_name) {
            panic!("Invalid event name found")
        }

        for (id, name) in &important_comp_data.event_defs {
            if *name == event_name {
                event_id = id.parse().expect("Error parsing str to int");
            }
        }

        let mut arg_amount = event_str.split(",").collect::<Vec<&str>>().len() as u32 + 1;

        if arg_amount == 2 {
            if event_name.contains("()") {
                arg_amount = 1;
            }
        }

        let args = event_str
            .split("(")
            .collect::<Vec<&str>>()[1]
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let mut buffers: Vec<Buffer> = Vec::with_capacity(arg_amount as usize - 1);

        for arg in args {
            if arg.replace(" ", "") == "" {
            }
            else if arg.starts_with("\"") {

                let mut bytes: Vec<u8> = Vec::new();

                bytes.push(0xA5);


                let str = arg.replace("\"", "");
                let str_bytes = str.encode_utf16().collect::<Vec<u16>>();
                let str_bytes = unsafe {str_bytes.align_to::<u8>()}.1;
                bytes.append(&mut str_bytes.to_vec());

                bytes.push(0);
                bytes.push(0);
                bytes.push(0xA1);

                let buffer = Buffer {
                    struct_type: 6,
                    length: bytes.len() as u32,
                    data: bytes
                };
                buffers.push(buffer);
            }
            else if arg.parse::<i64>().expect("Error parsing str to int") + 64 <= 0x7F   {
                let mut bytes: Vec<u8> = Vec::new();

                bytes.push((arg.parse::<i8>().expect("Error parsing str to sbyte") + 64) as u8);
                bytes.push(0xA1);

                let buffer = Buffer {
                    struct_type: 6,
                    length: bytes.len() as u32,
                    data: bytes,
                };

                buffers.push(buffer);
            }
            else {
                let mut bytes: Vec<u8> = Vec::new();

                bytes.push(0x82);

                let int_val = arg.parse::<i32>().expect("Error parsing str to int");
                bytes.append(&mut int_val.to_le_bytes().to_vec());

                bytes.push(0xA1);

                let buffer = Buffer {
                    struct_type: 6,
                    length: bytes.len() as u32,
                    data: bytes,
                };

                buffers.push(buffer);
            }
        }

        Event {
            struct_type: 7,
            version: 2,
            event_id,
            arg_amount,
            arg_evaluators: None,
            arg_buffers: Some(buffers)
        }
    }

    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let mut struct_type = 0;

        for (id, name) in &important_data.type_table {
            if name == "EzStateExternalEvent" {
                struct_type = id + 1;
                break;
            }
            else if name == "EzStateExternalEventT<ES_EVENT_PARAM_NUM_6>" {
                struct_type = id + 1;
                break;
            }
        }

        bw.write_uint16(struct_type);

        assert_values(&self.version, vec![1, 2]);
        bw.write_uint32(self.version);

        bw.write_uint32(self.event_id);

        let arg_amount;
        if let Some(evaluators) = &self.arg_evaluators {
            arg_amount = evaluators.len() as u32 + 1;
        }
        else {
            arg_amount = self.arg_buffers.as_ref().unwrap().len() as u32 + 1;
        }

        bw.write_uint32(arg_amount);

        if arg_amount <= 1 {
            return;
        }

        if self.version == 1 {
            for evaluator in self.arg_evaluators.as_ref().unwrap() {
                evaluator.write(bw, important_data);
            }
        }
        else {
            for buffer in self.arg_buffers.as_ref().unwrap() {
                buffer.write(bw, important_data, "buffer");
            }
        }
    }
}