use crate::types::structs::{Buffer, Evaluator};
use crate::utils::binary_reader::BinaryReader;
use crate::utils::important_data::ImportantData;
impl Evaluator {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Evaluator {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "EzStateEvaluator");

        let version = br.read_uint32();
        assert_eq!(version, 1);

        let buffer = Buffer::read(br, important_data);

        Evaluator {
            struct_type,
            version,
            buffer
        }
    }
}