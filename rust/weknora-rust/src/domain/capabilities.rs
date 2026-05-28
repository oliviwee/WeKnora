#[derive(Debug, Eq, PartialEq)]
pub struct CapabilitiesResponse {
    pub migration_phase: &'static str,
    pub implemented: Vec<&'static str>,
    pub compatibility_stubs: Vec<&'static str>,
}

impl Default for CapabilitiesResponse {
    fn default() -> Self {
        Self {
            migration_phase: "foundation",
            implemented: vec!["health", "version", "capabilities", "typed-error-envelope"],
            compatibility_stubs: vec![
                "auth",
                "knowledge-base",
                "knowledge",
                "chunks",
                "chat",
                "agents",
                "models",
                "datasources",
                "wiki",
                "mcp",
            ],
        }
    }
}

impl CapabilitiesResponse {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"migration_phase":"{}","implemented":{},"compatibility_stubs":{}}}"#,
            self.migration_phase,
            json_string_array(&self.implemented),
            json_string_array(&self.compatibility_stubs)
        )
    }
}

fn json_string_array(values: &[&str]) -> String {
    let items = values
        .iter()
        .map(|value| format!(r#""{value}""#))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}
