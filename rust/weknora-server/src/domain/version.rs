#[derive(Debug, Eq, PartialEq)]
pub struct VersionResponse {
    pub name: &'static str,
    pub version: &'static str,
    pub api_base_path: &'static str,
    pub implementation: &'static str,
}

impl Default for VersionResponse {
    fn default() -> Self {
        Self {
            name: "WeKnora",
            version: env!("CARGO_PKG_VERSION"),
            api_base_path: "/api/v1",
            implementation: "rust",
        }
    }
}

impl VersionResponse {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"name":"{}","version":"{}","api_base_path":"{}","implementation":"{}"}}"#,
            self.name, self.version, self.api_base_path, self.implementation
        )
    }
}
