use crate::domain::capabilities::CapabilitiesResponse;
use crate::domain::health::HealthResponse;
use crate::domain::rewrite_plan::RewritePlanResponse;
use crate::domain::version::VersionResponse;
use crate::http::HttpResponse;

pub fn healthz() -> HttpResponse {
    HttpResponse::json(200, "OK", HealthResponse::default().to_json())
}

pub fn version() -> HttpResponse {
    HttpResponse::json(200, "OK", VersionResponse::default().to_json())
}

pub fn capabilities() -> HttpResponse {
    HttpResponse::json(200, "OK", CapabilitiesResponse::default().to_json())
}

pub fn rewrite_plan() -> HttpResponse {
    HttpResponse::json(200, "OK", RewritePlanResponse::default().to_json())
}
