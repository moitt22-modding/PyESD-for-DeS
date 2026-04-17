use crate::types::structs::SharedString;
use crate::utils::binary_reader::BinaryReader;

impl SharedString {
    pub fn read(br: &mut BinaryReader) -> SharedString {
        let length = br.read_uint32();
        let str = br.read_string(length);
        
        SharedString {
            length,
            str
        }
    }
}