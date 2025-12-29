use crate::handlers::check_health;
use crate::handlers::save_page;
use crate::server::{HttpRequest, Method, build_response};

pub fn route_request(request: HttpRequest) -> String {
    // Take a request and decide what to do
    match (request.method, request.path.as_deref()) {
        (Some(Method::GET), Some("/health")) => check_health(),
        (Some(Method::POST), Some("/save")) => save_page(request.body),
        _ => build_response(404, "Resource not found"),
    }
}
