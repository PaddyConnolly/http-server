use crate::handlers::check_health;
use crate::handlers::save_page;
use crate::server::{HttpRequest, Method};

pub fn route_request(request: HttpRequest) -> (u16, String) {
    // Take a request and decide what to do
    match (&request.method, request.path.as_deref()) {
        (Some(Method::GET), Some("/health")) => check_health(),
        (Some(Method::POST), Some("/save")) => save_page(request),
        (Some(Method::OPTIONS), _) => (200, String::new()),
        _ => (404, String::from("Resource not found")),
    }
}
