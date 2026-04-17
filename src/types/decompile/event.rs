use crate::types::structs::{Buffer, Evaluator, Event};
use crate::utils::binary_reader::{assert_values, BinaryReader};
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl Event {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Event {
        let struct_type = br.read_uint16();
        assert_values(&important_data.get_type_name(struct_type), vec!["EzStateExternalEvent".to_string(), "EzStateExternalEventT<ES_EVENT_PARAM_NUM_6>".to_string()]);

        let version = br.read_uint32();
        assert_values(&version, vec![1, 2]);

        let event_id = br.read_uint32();
        let arg_amount = br.read_uint32();

        let mut arg_evaluators = None;
        let mut arg_buffers = None;

        if version == 1 {
            let mut arg_evaluators_vec = Vec::with_capacity(arg_amount as usize - 1);
            for _ in 0..arg_amount - 1 {
                arg_evaluators_vec.push(Evaluator::read(br, important_data));
            }
            arg_evaluators = Some(arg_evaluators_vec);
        }
        else {
            let mut arg_buffers_vec = Vec::with_capacity(arg_amount as usize - 1);
            for _ in 0..arg_amount - 1 {
                arg_buffers_vec.push(Buffer::read(br, important_data));
            }
            arg_buffers = Some(arg_buffers_vec)
        }

        Event {
            struct_type,
            version,
            event_id,
            arg_amount,
            arg_evaluators,
            arg_buffers
        }
    }

    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        let mut args: Vec<Vec<u8>> = Vec::new();

        if self.arg_buffers.is_some() {
            let buffers = self.arg_buffers.as_ref().unwrap();
            for buffer in buffers {
                args.push(buffer.data.clone());
            }
        }
        else {
            let evaluators = self.arg_evaluators.as_ref().unwrap();
            for evaluator in evaluators {
                args.push(evaluator.buffer.data.clone())
            }
        }

        let mut event_string = String::from("        ");

        if !important_data.event_defs.contains_key(&self.event_id.to_string()) {
            panic!("Invalid event id: {}", self.event_id)
        }

        let event_name = important_data.event_defs[&self.event_id.to_string()].to_string() + "(";
        event_string.push_str(&*event_name);


        for (idx, arg) in args.iter_mut().enumerate() {
            arg.remove(arg.len() - 1);
            if arg[0] <= 0x7F && arg.len() == 1 {
                let arg_value = arg[0] as i32 - 64;
                event_string.push_str(&*arg_value.to_string())
            }

            else if arg[0] == 0xA5 {
                arg.remove(0);

                let u16_array: Vec<u16> = arg
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                let mut str = String::from_utf16(u16_array.as_slice()).expect("Error reading arg string");
                str.remove(str.len() - 1);

                event_string.push_str(&*format!("\"{}\"", str));
            }

            else if arg[0] == 0x82 {
                arg.remove(0);

                let int_bytes: [u8; 4] = arg.as_slice().try_into().expect("Error converting int bytes to sized byte slice");

                let value = i32::from_le_bytes(int_bytes);

                event_string.push_str(&*value.to_string());
            }

            if idx != self.arg_amount as usize - 2 {
                event_string.push_str(", ")
            }
        }
        event_string.push_str(")");

        tw.write_line(event_string.clone())
    }
}