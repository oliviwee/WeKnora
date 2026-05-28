use crate::http::{HttpResponse, escape_json};

#[derive(Debug)]
pub struct ApiError {
    pub status: u16,
    pub reason: &'static str,
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn not_found(path: &str) -> Self {
        Self {
            status: 404,
            reason: "Not Found",
            code: "route.not_found",
            message: format!("no Rust route is registered for {path}"),
        }
    }

    pub fn not_implemented(feature: &'static str) -> Self {
        Self {
            status: 501,
            reason: "Not Implemented",
            code: "feature.not_implemented",
            message: format!("{feature} has not been ported to the Rust server yet"),
        }
    }

    pub fn into_response(self) -> HttpResponse {
        HttpResponse::json(
            self.status,
            self.reason,
            format!(
                r#"{{"error":{{"code":"{}","message":"{}"}}}}"#,
                self.code,
                escape_json(&self.message)
            ),
        )
    }
}
