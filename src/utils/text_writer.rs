use std::fs::{write};

pub struct TextWriter {
    pub lines: Vec<String>,
    pub path: String
}

impl TextWriter {
    pub fn new(path: &str) -> TextWriter {
        TextWriter {
            lines: Vec::new(),
            path: path.to_string()
        }
    }
    
    pub fn write_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn finish(&mut self) {
        let file_string = self.lines.join("\n");

        write(&self.path, (&*file_string).as_bytes()).expect("Error writing decompiled file");
    }
}