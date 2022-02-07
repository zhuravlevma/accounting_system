use base64::encode;

use super::write::Write;

pub struct SecureWriter {
    pub writer: Box<dyn Write>,
}
impl Write for SecureWriter {
    fn write(&self, message: String) {
        self.writer.write(encode(message));
    }
}
