use std::io::{Read, Write};
use std::{fs::File, path::PathBuf};

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let base_path = PathBuf::from("/tmp");
        let mut number = 0;
        let (file_path, file) = loop {
            let next_path = base_path.join(format!("random_name.{number}"));
            if let Ok(file) = File::create_new(&next_path) {
                break (next_path, file);
            }
            number += 1;
        };

        Ok(Self { file_path, file })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let _ = self.file.write(data)?;
        Ok(())
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut result = String::new();
        let mut file = File::open(&self.file_path)?;
        file.read_to_string(&mut result)?;
        Ok(result)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.file_path);
    }
}
