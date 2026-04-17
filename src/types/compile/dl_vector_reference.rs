use crate::types::structs::DLVectorReference;
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_data::ImportantData;


impl DLVectorReference {
    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let struct_type = important_data.get_type_by_name("DLVector");
        bw.write_uint16(struct_type);

        let count = self.references.len() as u32;
        bw.write_uint32(count);

        if count > 0 {
            if important_data.header_version == 0 {
                bw.write_uint16(self.item_type.unwrap());
                for i in 0..count {
                    self.references[i as usize].write(bw, self.item_type.unwrap(), important_data);
                }
            }
            else {
                for i in 0..count {
                    self.references[i as usize].write(bw, 0, important_data)
                }
            }
        }
    }
}