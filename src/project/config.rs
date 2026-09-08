use hotaru::akari::Value;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub schema_version: u32,
    pub project: String,
    pub script: String,
    pub script_config: Value,
}

impl ProjectConfig {
    pub const FILE_NAME: &'static str = "config.json";
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

    pub fn from_value(value: &Value) -> Result<Self, String> {
        let schema_version = value
            .try_get("schema_version")
            .map_err(|_| "config missing 'schema_version'".to_string())?
            .integer() as u32;
        if schema_version != Self::CURRENT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported config schema_version {schema_version}, expected {}",
                Self::CURRENT_SCHEMA_VERSION
            ));
        }
        let project = value
            .try_get("project")
            .map_err(|_| "config missing 'project'".to_string())?
            .string();
        let script = value
            .try_get("script")
            .map_err(|_| "config missing 'script'".to_string())?
            .string();
        let script_config = value
            .try_get("script_config")
            .map_err(|_| "config missing 'script_config'".to_string())?
            .clone();
        Ok(Self { schema_version, project, script, script_config })
    }

    pub fn read(path: impl AsRef<Path>) -> Result<Self, String> {
        let path_str = path.as_ref().to_str().ok_or("non-utf8 path")?;
        let value = Value::from_jsonf(path_str)?;
        Self::from_value(&value)
    }

    pub fn source(&self) -> Option<String> {
        self.script_config
            .try_get("source")
            .ok()
            .map(|v| v.string())
    }
}
