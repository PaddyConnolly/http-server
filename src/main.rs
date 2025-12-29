use std::io::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
enum Method {
    GET,
    POST,
}

struct HttpRequest {
    method: Option<Method>,
    path: Option<String>,
    body: Option<String>,
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

fn check_health() -> String {
    build_response(200, "Healthy")
}

fn save_page(body: Option<String>) -> String {
    // pass
    if let Some(_c) = body {
        build_response(200, "Page saved sucessfully")
    } else {
        build_response(400, "Missing body")
    }
}

fn build_response(status_code: u16, body: &str) -> String {
    // Takes in a status_code and a builds a response
    format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        get_status_code_text(status_code),
        body.len(),
        body
    )
}

fn route_request(request: HttpRequest) -> String {
    // Take a request and decide what to do
    match (request.method, request.path.as_deref()) {
        (Some(Method::GET), Some("/health")) => check_health(),
        (Some(Method::POST), Some("/save")) => save_page(request.body),
        _ => build_response(404, "Resource not found"),
    }
}

fn handle_connection(stream: TcpStream) -> Result<()> {
    // Handle the connection passed by TcpListener
    let mut reader: BufReader<TcpStream> = BufReader::new(stream);
    let mut line: String = String::new();

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

    let request: HttpRequest = HttpRequest { method, path, body };

    let response = route_request(request);

    let response = response.as_bytes();

    let mut stream = reader.into_inner();

    stream.write_all(response)?;
    stream.flush()?;

    Ok(())
}

fn main() -> Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")?;

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
