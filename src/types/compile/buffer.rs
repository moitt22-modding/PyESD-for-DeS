use crate::types::structs::Buffer;
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_data::ImportantData;

impl Buffer {
    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData, type_name: &str) {
        let struct_type = important_data.get_type_by_name(type_name);
        bw.write_uint16(struct_type);

        let length = self.data.len() as u32;
        bw.write_uint32(length);

        for byte in &self.data {
            bw.write_uint8(*byte);
        }
    }
}