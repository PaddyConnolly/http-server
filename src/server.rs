use crate::router::route_request;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::net::TcpStream;

pub struct HttpRequest {
    pub method: Option<Method>,
    pub path: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

fn get_status_code_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        404 => "Not found",
        _ => "Unknown",
    }
}

fn parse_start_line(string: &str) -> (Option<&str>, Option<&str>) {
    // Takes in first line of TCP Stream, and parses the method and path
    let mut parts = string.split_whitespace();
    (parts.next(), parts.next())
}

fn parse_content_length_header(string: &str) -> Option<usize> {
    // Takes in line of TCP Stream, and parses Content-Length if present
    let mut parts = string.split_whitespace();
    parts.next();
    parts.next()?.parse().ok()
}

pub fn build_response(status_code: u16, body: &str) -> String {
    // Takes in a status_code and builds a response
    format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        get_status_code_text(status_code),
        body.len(),
        body
    )
}

pub fn handle_connection(stream: TcpStream) -> Result<()> {
    // Handle the connection passed by TcpListener
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    let mut method: Option<Method> = None;
    let mut path: Option<String> = None;
    let mut content_length: Option<usize> = None;

    let mut body: Option<String> = None;

    while let Ok(_size) = reader.read_line(&mut line) {
        // Parse start line
        if method.is_none() {
            if let (Some(first), Some(second)) = parse_start_line(&line) {
                match first {
                    "POST" => {
                        method = Some(Method::POST);
                        path = Some(String::from(second));
                    }
                    "GET" => {
                        method = Some(Method::GET);
                        path = Some(String::from(second));
                    }
                    _ => {}
                }
            }
        }

        // Find Content-Length
        if line.starts_with("Content-Length:") {
            content_length = parse_content_length_header(&line);
        }

        // Find empty line, signalling end of headers
        if line.trim().is_empty() {
            if let Some(length) = content_length {
                let mut buffer = vec![0u8; length];

                if let Ok(()) = reader.read_exact(&mut buffer) {
                    body = String::from_utf8(buffer).ok();
                }
            }
            break;
        }
        line.clear();
    }

    let request = HttpRequest { method, path, body };

    let response = route_request(request);

    let response = response.as_bytes();

    let mut stream = reader.into_inner();

    stream.write_all(response)?;
    stream.flush()?;

    Ok(())
}
