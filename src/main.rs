mod handlers;
mod router;
mod server;

use crate::server::handle_connection;
use std::io::Result;
use std::net::TcpListener;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
