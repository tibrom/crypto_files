use crate::domain::services::TerminalService;

pub struct Terminal;

impl TerminalService for Terminal {
    fn print_msg(&self, msg: String) {
        println!("{msg}");
    }
    fn print_message_in_line(&self, msg: String) {
        print!("\x1B[1A\x1B[2K");
        println!("{msg}");
    }
    fn print_error_msg(&self, msg: String) {
        eprintln!("{msg}");
    }
    fn print_chunk(&self, value: Vec<u8>) {
        let text = String::from_utf8_lossy(&value);
        print!("{text}");
    }
}
