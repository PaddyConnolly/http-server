use crate::server::build_response;

pub fn check_health() -> String {
    build_response(200, "Healthy")
}

pub fn save_page(body: Option<String>) -> String {
    if body.is_some() {
        build_response(200, "Page saved successfully")
    } else {
        build_response(400, "Missing body")
    }
}
