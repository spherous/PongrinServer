use tokio::fs::OpenOptions;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn log(message: &str) -> Result<(), std::io::Error>{
    let now = chrono::Utc::now();
    let now_str = now.format("%b %-d, %-I:%M").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("pongrin-log.html")
        .await?;
   
    let msg = format!("{:?}: {:?}<br>\n", now_str, message);
    file.write_all(msg.as_bytes()).await?;
    
    println!("{}: {}", now_str, message);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    
    tokio::spawn(async {
        log("PongrinServer Started.").await.unwrap();
    });

    loop {
        let (mut socket, client) = listener.accept().await?;

        tokio::spawn(async move {
            log(&format!("Client connected: {:?}", client)).await.unwrap();
        });

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let bytes_array_length = match socket.read(&mut buf).await {
                    Ok(bal) if bal == 0 => {
                        tokio::spawn(async move {
                            log("Client disconnected from server.").await.unwrap();
                        });
                        None
                    },
                    Ok(bal) => {
                        let msg = String::from_utf8((&buf[..bal]).to_vec()).unwrap();
                        tokio::spawn(async move {
                            log(&format!("Got = {} from client", msg)).await.unwrap();
                        });
                        Some(bal)
                    },
                    Err(e) => {
                        tokio::spawn(async move {
                            log(&format!("Failed to read from the socket; err = {:?}", e)).await.unwrap();
                        });
                        None
                    }
                };

                match bytes_array_length {
                    Some(bal) => {
                        if let Err(e) = socket.write_all(&buf[0..bal]).await {
                            eprintln!("Failed to write to socket; err = {:?}", e);
                            return
                        }
                    }
                    None => return
                }
            }
        });
    }
}