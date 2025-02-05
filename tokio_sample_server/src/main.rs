use std::io::Read;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            //let mut buf[] = Vec::new();
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // change the stuff to be written back
                //let reply = "you wrote > ".as_bytes();
                //buf.extend(reply);

                let prefix = "you wrote > ";
                let mut response = Vec::with_capacity(prefix.len() + n);
                response.extend_from_slice(prefix.as_bytes());
                response.extend_from_slice(&buf[..n]);

                // Add newline if it's not there
                if !response.ends_with(b"\n") {
                    response.extend_from_slice(b"\n");
                }

                if let Err(e) = socket.write_all(&response).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
