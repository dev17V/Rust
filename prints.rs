use tokio::{net::{TcpListener, TcpStream}, sync::Mutex};

pub fn get_pink(content: String) -> String
{ 
    let formatted = format!("\x1b[38;2;255;0;255m{}\r\n\x1b[0m", content);
    formatted
}
pub fn get_banner(clear : bool) -> String
{ 
    let mut banner = String::new();
    if clear {
        banner.push_str("\x1b[2J\x1b[1;1H");
    }
    banner.push_str("Toshi's C2");
    let formatted = format!("\x1b[38;2;255;0;255m{}\r\n\x1b[0m", banner);
    formatted
}