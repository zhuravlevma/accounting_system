use std::fs;

use super::write::Write;

pub struct FileWriter {
    path: String,
}
impl Write for FileWriter {
    fn write(&self, message: String) {
        fs::write(&self.path, message).unwrap();
    }
}
