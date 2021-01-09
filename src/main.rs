use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:1234").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Client disconnected from server.");
                        return
                    },
                    Ok(n) => 
                    {
                        let msg = String::from_utf8((&buf[..n]).to_vec()).unwrap();
                        println!("Got = {:?} from client.", msg);
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