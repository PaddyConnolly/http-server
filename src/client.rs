use std::io::{Result, Write};
use std::net::TcpStream;

pub fn post(host: &str, path: &str) -> Result<()> {
    let mut stream = TcpStream::connect(host)?;
    let request = format!(
        "POST {} HTTP/1.1\r\n\
        Host: {}\r\n\
        Content-Length: 0\r\n\
        \r\n",
        path, host
    );
    stream.write_all(request.as_bytes())?;
    stream.flush()?;
    Ok(())
}
