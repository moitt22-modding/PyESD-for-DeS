use crate::types::structs::SharedString;
use crate::utils::binary_writer::BinaryWriter;

impl SharedString {

    pub fn write(&self, bw: &mut BinaryWriter) {
        let length = self.str.len() as u32;
        bw.write_uint32(length);
        
        bw.write_string(self.str.clone())
    }
}