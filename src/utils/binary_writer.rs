use std::fs::write;

macro_rules! write_int {
    ($br:expr, $int_type:ty, $value:expr) => {
        let mut bytes = $value.to_be_bytes().to_vec();
        $br.bytes.append(&mut bytes);
    };
}

pub struct BinaryWriter{
    bytes: Vec<u8>,
    path: String
}

impl BinaryWriter {
    pub fn new(path: &str) -> BinaryWriter {
        BinaryWriter {
            bytes: Vec::new(),
            path: path.to_string()
        }
    }
    pub fn finish(&self) {
        write(&self.path, &self.bytes).expect("Error writing bytes to file");
    }
    pub fn write_uint8(&mut self, val: u8) {
        write_int!(self, u8, val);
    }
    pub fn write_uint16(&mut self, val: u16) {
        write_int!(self, u16, val);
    }
    pub fn write_uint32(&mut self, val: u32) {
        write_int!(self, u32, val);
    }
    pub fn write_uint64(&mut self, val: u64) {
        write_int!(self, u64, val);
    }
    pub fn write_string(&mut self, str: String) {
        let mut bytes = str.as_bytes().to_vec();
        self.bytes.append(&mut bytes)
    }
}