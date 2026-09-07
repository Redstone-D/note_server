use hotaru::akari::Value;

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub schema_version: u32,
    pub project: String,
    pub script: String,
    pub script_config: Value,
}

impl ProjectConfig {
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    pub fn new(
        project: impl Into<String>,
        script: impl Into<String>,
        script_config: Value,
    ) -> Self {
        Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            project: project.into(),
            script: script.into(),
            script_config,
        }
    }

    pub fn source(&self) -> Option<String> {
        self.script_config
            .try_get("source")
            .ok()
            .map(|v| v.string())
    }
}
