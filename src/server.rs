use crate::router::route_request;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::net::TcpStream;

pub struct HttpRequest {
    pub method: Option<Method>,
    pub path: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    OPTIONS,
}

fn get_status_code_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        404 => "Not found",
        _ => "Unknown",
    }
}

fn parse_request_line(line: &str) -> (Option<&str>, Option<&str>) {
    // Takes in first line of TCP Stream, and parses the method and path
    let mut parts = line.split_whitespace();
    (parts.next(), parts.next())
}

fn parse_header(line: &str, headers: &mut HashMap<String, String>) {
    if let Some((key, value)) = line.split_once(':') {
        headers.insert(
            key.to_string().trim().to_lowercase(),
            value.trim().to_string(),
        );
    }
}

fn parse_body(
    reader: &mut BufReader<TcpStream>,
    headers: &HashMap<String, String>,
) -> Option<String> {
    let length = headers.get("content-length")?.parse::<usize>().ok()?;
    let mut buffer = vec![0u8; length];
    reader.read_exact(&mut buffer).ok()?;
    String::from_utf8(buffer).ok()
}

pub fn build_response(status_code: u16, body: &str) -> String {
    // Takes in a status_code and builds a response
    format!(
        "HTTP/1.1 {} {}\r\n\
        Content-Length: {}\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Access-Control-Allow-Methods: POST, GET, OPTIONS\r\n\
        Access-Control-Allow-Headers: Page-URL, Content-Type\r\n\
        \r\n\
        {}",
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
    let mut headers: HashMap<String, String> = HashMap::new();

    let mut body: Option<String> = None;

    while let Ok(_size) = reader.read_line(&mut line) {
        // Parse start line
        if method.is_none() {
            if let (Some(first), Some(second)) = parse_request_line(&line) {
                match first {
                    "POST" => {
                        method = Some(Method::POST);
                        path = Some(String::from(second));
                    }
                    "GET" => {
                        method = Some(Method::GET);
                        path = Some(String::from(second));
                    }
                    "OPTIONS" => {
                        method = Some(Method::OPTIONS);
                        path = Some(String::from(second));
                    }
                    _ => {}
                }
            } else {
                let response = build_response(400, "Bad Request");
                let mut stream = reader.into_inner();
                stream.write_all(response.as_bytes()).ok();
                stream.flush().ok();
                return Ok(());
            }

            line.clear();
            continue;
        }

        // Find empty line, signalling end of headers, starting parsing body
        if line.trim().is_empty() {
            body = parse_body(&mut reader, &headers);
            break;
        }

        // Otherwise, line is a header
        parse_header(&line, &mut headers);

        line.clear();
    }

    let request = HttpRequest {
        method,
        path,
        headers,
        body,
    };

    let response = route_request(request);

    let response_string = build_response(response.0, &response.1);

    let mut stream = reader.into_inner();

    stream.write_all(response_string.as_bytes())?;
    stream.flush()?;

    Ok(())
}
