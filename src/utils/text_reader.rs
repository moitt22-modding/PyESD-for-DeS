use std::fs::read_to_string;

pub struct TextReader {
    pub lines: Vec<String>,
    pub line_idx: usize
}

impl TextReader {
    pub fn new(path: &str) -> TextReader {
        let lines: Vec<String> = read_to_string(path)
            .expect("Error reading decompiled file to text")
            .replace(" ", "")
            .replace("\r", "")
            .split("\n")
            .map(|v| v.to_string())
            .collect();

        TextReader {
            lines,
            line_idx: 0
        }
    }

    pub fn read_line(&mut self) -> String {
        let ret = self.lines[self.line_idx].clone();
        self.line_idx += 1;
        ret
    }
}