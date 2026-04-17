use crate::types::structs::Evaluator;
use crate::utils::binary_writer::BinaryWriter;
use crate::utils::important_data::ImportantData;

impl Evaluator {
    pub fn write(&self, bw: &mut BinaryWriter, important_data: &mut ImportantData) {
        let struct_type = important_data.get_type_by_name("EzStateEvaluator");
        bw.write_uint16(struct_type);
        
        let version = 1;
        bw.write_uint32(version);
        
        self.buffer.write(bw, important_data, "DLVector");
    }
}