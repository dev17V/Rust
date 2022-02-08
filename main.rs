mod prints;
mod ip_tools;

use tokio::{net::{TcpListener}, sync::Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("localhost:1771").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            
            let mut logbuf = [0; 1024];
            let username = 
                match socket.read(&mut logbuf).await {
                    // socket closed
                    Ok(n) if n == 0 => "".to_string(),
                    Ok(n) =>  String::from_utf8(logbuf.to_vec()).unwrap().to_owned(),
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        "".to_string()
                    }
                };
                let password = 
                match socket.read(&mut logbuf).await {
                    // socket closed
                    Ok(n) if n == 0 => "".to_string(),
                    Ok(n) =>  String::from_utf8(logbuf.to_vec()).unwrap().to_owned(),
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        "".to_string()
                    }
                };
            let formatted = prints::get_banner(false);
                    if let Err(e) = socket.write_all(formatted.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }if let Err(e) = socket.write_all("You need to login".as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
            let mut buf = [0; 1024];
            let formatted = prints::get_banner(false);
                    if let Err(e) = socket.write_all(formatted.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
            // In a loop, read data from the socket and write the data back.
            loop {
                let _ = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                let content = String::from_utf8((&buf).to_vec()).unwrap();
                if content.starts_with("help")
                {
                    let formatted = prints::get_pink("geo".to_string());
                    if let Err(e) = socket.write_all(formatted.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
                else if content.starts_with("clear")
                {
                    if let Err(e) = socket.write_all(prints::get_banner(true).as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
                else if content.starts_with("geo")
                {
                    let ip = match content.split(' ').last() {
                        Some(ip) => ip,
                        None => { 
                            if let Err(e) = socket.write_all("You need to enter an IP\r\n".as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            }
                            "geo"
                        },
                    };
                    if ip.starts_with("geo") || ip.is_empty() {
                        if let Err(e) = socket.write_all(prints::get_pink("You need to enter an IP".to_string()).as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                    else {
                    let info = ip_tools::get_ip_info(ip.to_string()).await;
                    let formatted = prints::get_pink(info);
                    if let Err(e) = socket.write_all(formatted.as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                     }
                    }
                }
                else {
                    let formatted = format!("\x1b[38;2;255;0;255m{}\r\n\x1b[0m", "Command doesn't exist");
                    if let Err(e) = socket.write_all(formatted.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
            }
        });
    }
}
