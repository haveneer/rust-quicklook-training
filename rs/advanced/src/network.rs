use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::parser::parse_instruction;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New client: {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return, // client fermé
                    Ok(n) => n,
                    Err(_) => return,
                };
                if let Some(instr) = parse_instruction(&buffer[..n]) {
                    // exécuter (soit localement, soit via un shared Arc<Machine>)
                    // ...
                    // Envoyer éventuellement un ack
                    let _ = socket.write_all(b"ok").await;
                }
            }
        });
    }
}
