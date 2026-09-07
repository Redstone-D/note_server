use hotaru::akari::Value;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProjectManifest {
    pub schema_version: u32,
    pub project: String,
    pub title: String,
}

impl ProjectManifest {
    pub const FILE_NAME: &'static str = "manifest.json";
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    pub fn new(project: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            project: project.into(),
            title: title.into(),
        }
    }

    pub fn to_value(&self) -> Value {
        let mut v = Value::new_dict();
        v.set("schema_version", self.schema_version as i64);
        v.set("project", self.project.clone());
        v.set("title", self.title.clone());
        v
    }

    pub fn from_value(value: &Value) -> Result<Self, String> {
        let schema_version = value
            .try_get("schema_version")
            .map_err(|_| "manifest missing 'schema_version'".to_string())?
            .integer() as u32;
        if schema_version != Self::CURRENT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported manifest schema_version {schema_version}, expected {}",
                Self::CURRENT_SCHEMA_VERSION
            ));
        }
        let project = value
            .try_get("project")
            .map_err(|_| "manifest missing 'project'".to_string())?
            .string();
        let title = value
            .try_get("title")
            .map_err(|_| "manifest missing 'title'".to_string())?
            .string();
        Ok(Self { schema_version, project, title })
    }

    pub fn read(path: impl AsRef<Path>) -> Result<Self, String> {
        let path_str = path.as_ref().to_str().ok_or("non-utf8 path")?;
        let value = Value::from_jsonf(path_str)?;
        Self::from_value(&value)
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path_str = path.as_ref().to_str().ok_or("non-utf8 path")?;
        self.to_value().into_jsonf(path_str)
    }
}
