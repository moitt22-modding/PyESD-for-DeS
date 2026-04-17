use crate::types::structs::{DLVectorReference, Project};
use crate::utils::binary_reader::BinaryReader;
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl Project {
     pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> Project {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "EzStateProject");

        let version = br.read_uint32();
        assert_eq!(version, 2);

        let maps = DLVectorReference::read(br, important_data);

        Project {
            struct_type,
            version,
            maps
        }
    }

    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        self.maps.decompile(tw, important_data);
    }
}