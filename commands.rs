use tokio::{net::{TcpListener, TcpStream}, sync::Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn WriteBanner(ss: &mut TcpStream)
{
    let formatted = get_banner(false);
    if let Err(e) = ss.write_all(formatted.as_bytes()).await {
        eprintln!("failed to write to socket; err = {:?}", e);
        return;
    }if let Err(e) = ss.write_all("You need to login".as_bytes()).await {
        eprintln!("failed to write to socket; err = {:?}", e);
        return;
    }
let mut buf = [0; 1024];
let formatted = get_banner(false);
    if let Err(e) = ss.write_all(formatted.as_bytes()).await {
        eprintln!("failed to write to socket; err = {:?}", e);
        return;
    }
}
fn get_banner(clear : bool) -> String
{ 
    let mut banner = String::new();
    if clear {
        banner.push_str("\x1b[2J\x1b[1;1H");
    }
    banner.push_str("Toshi's C2");
    let formatted = format!("\x1b[38;2;255;0;255m{}\r\n\x1b[0m", banner);
    formatted
}