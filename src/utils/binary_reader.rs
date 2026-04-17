use std::fmt::{Debug, Display};
use std::fs::read;

pub fn assert_values<T: PartialEq + Debug + Display>(val: &T, cmp_values: Vec<T>) {
    let mut is_valid = false;

    for cmp_val in &cmp_values {
        if *val == *cmp_val {
            is_valid = true
        }
    }

    if !is_valid {
        panic!("Error asserting values Expected: {} Found: {:?}", val, cmp_values)
    }
}

macro_rules! read_int {
    ($br:expr, $int_type:ty) => {
        const SIZE: usize = size_of::<$int_type>();

        let bytes = &($br.bytes[$br.current_position .. $br.current_position + SIZE]);
        let bytes: [u8; SIZE] = bytes.try_into().expect("Error converting slice ref to owned slice");
        $br.current_position += SIZE;

        let ret_val = <$int_type>::from_be_bytes(bytes);
        return ret_val;
    };
}

pub struct BinaryReader {
    pub bytes: Vec<u8>,
    pub current_position: usize,
}

impl BinaryReader {
    pub fn new(path: String) -> BinaryReader {
        BinaryReader {
            bytes: read(path).expect("Error reading file in BinaryReader"),
            current_position: 0
        }
    }

    pub fn read_uint8(&mut self) -> u8 {
        read_int!(self, u8);
    }
    pub fn read_uint16(&mut self) -> u16 {
        read_int!(self, u16);
    }
    pub fn read_uint32(&mut self) -> u32 {
        read_int!(self, u32);
    }
    pub fn read_uint64(&mut self) -> u64 {
        read_int!(self, u64);
    }
    pub fn read_string(&mut self, length: u32) -> String {
        let length = length as usize;
        let bytes = &self.bytes[self.current_position .. self.current_position + length];

        self.current_position += length;

        String::from_utf8(bytes.to_vec()).expect("Error reading utf8 from slice")
    }
}