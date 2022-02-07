use super::write::Write;

pub struct Terminal;
impl Write for Terminal {
    fn write(&self, message: String) {
        println!("{}", message);
    }
}
