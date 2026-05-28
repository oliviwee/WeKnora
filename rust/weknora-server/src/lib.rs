pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod http;
pub mod state;

use crate::api::{capabilities, healthz, version};
use crate::error::ApiError;
use crate::http::{HttpRequest, HttpResponse, Method};
use crate::state::AppState;

pub fn handle_request(request: &HttpRequest, _state: &AppState) -> HttpResponse {
    match (&request.method, request.path.as_str()) {
        (Method::Get, "/healthz") => healthz(),
        (Method::Get, "/api/v1/version") => version(),
        (Method::Get, "/api/v1/capabilities") => capabilities(),
        _ => ApiError::not_found(&request.path).into_response(),
    }
}
