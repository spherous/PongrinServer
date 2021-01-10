use std::fs::OpenOptions;
use std::io::Write;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn log(message: &str) {
    let now = chrono::Utc::now();
    let now_str = now.format("%b %-d, %-I:%M").to_string();

    //let mut file = std::fs::File::create("pongrin-log.html").expect("create failed");
    let mut file = OpenOptions::new().create(true)
                                     .write(true)
                                     .append(true)
                                     .open("pongrin-log.html")
                                     .expect("create failed");

    file.write_all(now_str.as_bytes()).expect("write failed");
    file.write_all(":".as_bytes()).expect("write failed");
    file.write_all(message.as_bytes()).expect("write failed");
    file.write_all("<br>\n".as_bytes()).expect("write failed");

    print!("{}:{}\n", now_str, message);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let listener = TcpListener::bind("142.11.199.204:1234").await?;
    let listener = TcpListener::bind("0:1234").await?;
    log("PongrinServer Started.");

    loop {
        let (mut socket, client) = listener.accept().await.unwrap();
        log(&client.to_string());

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        log("Client disconnected from server.");
                        return
                    },
                    Ok(n) => 
                    {
                        let msg = String::from_utf8((&buf[..n]).to_vec()).unwrap();

                        let str1: &str = "Got = ";
                        let str2: &str = &msg;
                        let str3: &str = " from client.";
                        let _str4 = format!("{}{}{}", str1, str2, str3);
                        log(&_str4);
                        n
                    },
                    Err(e) => {
                        eprintln!("Failed to read from the socket; err = {:?}", e);
                        return
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
