mod client;
mod db;
mod handlers;
mod router;
mod server;

use crate::server::handle_connection;
use std::error::Error;
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn Error>> {
    db::db_init()?;

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("\x1b[1;92m   Listening\x1b[0m at http://127.0.0.1:8080");

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
