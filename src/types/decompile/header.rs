use crate::types::structs::{Header, SharedString};
use crate::utils::binary_reader::{assert_values, BinaryReader};

impl Header {
    pub fn read(br: &mut BinaryReader) -> Header {
        let magic = br.read_string(4);
        assert_eq!(magic, "DLSE");

        let unk04 = br.read_uint16();
        assert_eq!(unk04, 2);

        let version = br.read_uint16();
        assert_values(&version, vec![0u16, 1u16, 3u16]);

        let mut unk08 = None;
        let mut transition_count = None;
        let mut highest_ref_id_plus_one = None;

        if version == 3 {
            unk08 = Some(br.read_uint32());
            transition_count = Some(br.read_uint32());
            highest_ref_id_plus_one = Some(br.read_uint32());
        }

        let string_count = br.read_uint16();
        let mut strings = Vec::with_capacity(string_count as usize);

        for _ in 0..string_count {
            strings.push(SharedString::read(br));
        }
        
        Header {
            magic,
            unk04,
            version,
            unk08,
            transition_count,
            highest_ref_id_plus_one,
            string_count,
            strings
        }
    }
}