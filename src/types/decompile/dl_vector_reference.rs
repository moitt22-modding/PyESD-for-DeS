use crate::types::structs::{DLVectorReference, Reference};
use crate::utils::binary_reader::BinaryReader;
use crate::utils::important_data::ImportantData;
use crate::utils::text_writer::TextWriter;

impl DLVectorReference {
    pub fn read(br: &mut BinaryReader, important_data: &mut ImportantData) -> DLVectorReference {
        let struct_type = br.read_uint16();
        assert_eq!(important_data.get_type_name(struct_type), "DLVector");

        let ref_count = br.read_uint32();

        let mut item_type = None;

        let mut references = Vec::with_capacity(ref_count as usize);

        if ref_count <= 0 {
            return DLVectorReference {
                struct_type,
                ref_count,
                item_type,
                references
            }
        }

        if important_data.header_version == 0 {
            item_type = Some(br.read_uint16());
            for _ in 0..ref_count {
                references.push(Reference::read(br, item_type.unwrap(), important_data));
            }
        }
        else {
            for _ in 0..ref_count {
                references.push(Reference::read(br, 0, important_data));
            }
        }

        DLVectorReference {
            struct_type,
            ref_count,
            item_type,
            references
        }
    }

    pub fn decompile(&self, tw: &mut TextWriter, important_data: &ImportantData) {
        for reference in &self.references {
            reference.decompile(tw, important_data);
        }
    }
}