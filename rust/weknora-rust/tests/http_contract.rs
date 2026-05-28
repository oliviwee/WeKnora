use weknora_rust::http::{HttpRequest, Method};
use weknora_rust::{config::Config, handle_request, state::AppState};

#[test]
fn healthz_reports_ok() {
    let response = call(Method::Get, "/healthz");

    assert_eq!(response.status, 200);
    assert!(response.body.contains(r#""status":"ok""#));
    assert!(response.body.contains(r#""service":"weknora-rs""#));
}

#[test]
fn version_uses_v1_api_base_path() {
    let response = call(Method::Get, "/api/v1/version");

    assert_eq!(response.status, 200);
    assert!(response.body.contains(r#""name":"WeKnora-Rust""#));
    assert!(response.body.contains(r#""api_base_path":"/api/v1""#));
    assert!(response.body.contains(r#""implementation":"rust""#));
}

#[test]
fn capabilities_expose_migration_phase() {
    let response = call(Method::Get, "/api/v1/capabilities");

    assert_eq!(response.status, 200);
    assert!(response.body.contains(r#""migration_phase":"foundation""#));
    assert!(response.body.contains(r#""health""#));
    assert!(response.body.contains(r#""chat""#));
}

#[test]
fn unknown_route_uses_typed_error_envelope() {
    let response = call(Method::Get, "/api/v1/chat");

    assert_eq!(response.status, 404);
    assert!(response.body.contains(r#""code":"route.not_found""#));
}

fn call(method: Method, path: &str) -> weknora_rust::http::HttpResponse {
    let state = AppState::new(Config::for_tests());
    let request = HttpRequest {
        method,
        path: path.to_string(),
    };
    handle_request(&request, &state)
}
