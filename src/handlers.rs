use crate::db;
use crate::server::HttpRequest;

pub fn check_health() -> (u16, String) {
    (200, String::from("Healthy"))
}

pub fn save_page(request: HttpRequest) -> (u16, String) {
    match (request.headers.get("page-url"), request.body.as_deref()) {
        (Some(url), Some(body)) => match db::insert_page(url, body) {
            Ok(_) => (200, String::from("Page saved successfully")),
            Err(_) => (500, String::from("Database error")),
        },
        _ => (400, String::from("Missing url or body")),
    }
}
