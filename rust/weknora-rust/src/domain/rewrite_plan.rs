#[derive(Debug, Eq, PartialEq)]
pub struct RewritePlanResponse {
    pub project: &'static str,
    pub strategy: &'static str,
    pub phases: Vec<&'static str>,
    pub components: Vec<ComponentPlan>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ComponentPlan {
    pub component: &'static str,
    pub current_stack: &'static str,
    pub target_stack: &'static str,
    pub reason: &'static str,
    pub migration_phase: &'static str,
}

impl Default for RewritePlanResponse {
    fn default() -> Self {
        Self {
            project: "WeKnora-Rust",
            strategy: "Rust core services + Python document/AI adapters + TypeScript UI + SQL data layer",
            phases: vec![
                "foundation",
                "api-contract",
                "repository-traits",
                "read-path",
                "rag-runtime",
                "workers",
                "write-path",
                "cli-and-desktop",
                "python-service-boundary",
            ],
            components: vec![
                ComponentPlan {
                    component: "api-server",
                    current_stack: "Go / Gin",
                    target_stack: "Rust",
                    reason: "Rust gives the long-running API boundary memory safety, predictable concurrency, and small single-binary deployment.",
                    migration_phase: "api-contract",
                },
                ComponentPlan {
                    component: "rag-retrieval-runtime",
                    current_stack: "Go",
                    target_stack: "Rust",
                    reason: "Retrieval fusion, chunk filtering, rerank preparation, and context assembly are latency-sensitive and benefit from Rust's low overhead.",
                    migration_phase: "rag-runtime",
                },
                ComponentPlan {
                    component: "task-workers-and-indexing",
                    current_stack: "Go",
                    target_stack: "Rust",
                    reason: "Background indexing and wiki ingest need robust concurrency, cancellation, retry, and resource control.",
                    migration_phase: "workers",
                },
                ComponentPlan {
                    component: "docreader",
                    current_stack: "Python",
                    target_stack: "Python service behind Rust boundary",
                    reason: "OCR, layout analysis, office parsing, and multimodal model adapters are most productive in the Python ecosystem.",
                    migration_phase: "python-service-boundary",
                },
                ComponentPlan {
                    component: "web-frontend",
                    current_stack: "Vue / TypeScript",
                    target_stack: "Vue / TypeScript",
                    reason: "Browser UI development, component libraries, and typed API clients are most efficient in TypeScript.",
                    migration_phase: "keep-stack",
                },
                ComponentPlan {
                    component: "cli",
                    current_stack: "Go",
                    target_stack: "Rust",
                    reason: "A Rust CLI can share API contracts and ship as a small static-friendly binary.",
                    migration_phase: "cli-and-desktop",
                },
                ComponentPlan {
                    component: "database-migrations",
                    current_stack: "SQL",
                    target_stack: "SQL",
                    reason: "Schema, indexes, and query plans should stay close to the database engine.",
                    migration_phase: "keep-stack",
                },
            ],
        }
    }
}

impl RewritePlanResponse {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"project":"{}","strategy":"{}","phases":{},"components":{}}}"#,
            escape_json(self.project),
            escape_json(self.strategy),
            json_string_array(&self.phases),
            json_component_array(&self.components),
        )
    }
}

fn json_component_array(values: &[ComponentPlan]) -> String {
    let items = values
        .iter()
        .map(|value| {
            format!(
                r#"{{"component":"{}","current_stack":"{}","target_stack":"{}","reason":"{}","migration_phase":"{}"}}"#,
                escape_json(value.component),
                escape_json(value.current_stack),
                escape_json(value.target_stack),
                escape_json(value.reason),
                escape_json(value.migration_phase),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn json_string_array(values: &[&str]) -> String {
    let items = values
        .iter()
        .map(|value| format!(r#""{}""#, escape_json(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
