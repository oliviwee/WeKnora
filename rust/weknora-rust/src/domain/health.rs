#[derive(Debug, Eq, PartialEq)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            status: "ok",
            service: "weknora-rs",
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

impl HealthResponse {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"status":"{}","service":"{}","version":"{}"}}"#,
            self.status, self.service, self.version
        )
    }
}
