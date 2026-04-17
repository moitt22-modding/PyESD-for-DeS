use crate::types::structs::Buffer;
use crate::utils::binary_reader::{assert_values, BinaryReader};
use crate::utils::important_data::ImportantData;
impl Buffer {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Buffer {
        let struct_type = br.read_uint16();
        assert_values(&important_data.get_type_name(struct_type), vec!["buffer".to_string(), "DLVector".to_string()]);
        
        let length = br.read_uint32();
        
        let mut data = Vec::with_capacity(length as usize);
        for _ in 0..length {
            data.push(br.read_uint8());
        }
        
        Buffer {
            struct_type,
            length,
            data
        }
    }
}