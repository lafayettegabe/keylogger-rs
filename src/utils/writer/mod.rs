use std::fs::OpenOptions;
use std::io::{Result, Write};

pub struct FileWriter {
    file: std::fs::File,
}

impl FileWriter {
    pub fn new(path: &str) -> Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self { file })
    }

    pub fn write_line(&mut self, content: &str) -> Result<()> {
        writeln!(self.file, "{}", content)?;
        self.file.flush()
    }
}
