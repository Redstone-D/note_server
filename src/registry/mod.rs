mod executor;
mod script;
mod scripts;

pub use executor::Executor;
pub use script::Script;
pub use scripts::{FolderSync, GithubSync};

use hotaru::akari::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    let raw = std::env::var("NOTES_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./published"));
    let root = if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir().map(|c| c.join(&raw)).unwrap_or(raw)
    };
    Registry::new(root).with(FolderSync).with(GithubSync)
});

pub struct Registry {
    publication_root: PathBuf,
    executor: Executor,
    scripts: HashMap<&'static str, Box<dyn Script>>,
}

impl Registry {
    pub fn new(publication_root: PathBuf) -> Self {
        Self {
            publication_root,
            executor: Executor::new(),
            scripts: HashMap::new(),
        }
    }

    pub fn with(mut self, script: impl Script + 'static) -> Self {
        self.scripts.insert(script.id(), Box::new(script));
        self
    }

    pub async fn dispatch(
        &self,
        project: &str,
        id: &str,
        config: Value,
        event: Value,
    ) -> Result<(), String> {
        let sandbox = self.sandbox_for(project)?;
        tokio::fs::create_dir_all(&sandbox)
            .await
            .map_err(|e| format!("mkdir sandbox: {e}"))?;
        match self.scripts.get(id) {
            Some(s) => s.run(&self.executor, sandbox, config, event).await,
            None => Err(format!("unknown script: {id}")),
        }
    }

    pub fn sandbox_for(&self, project: &str) -> Result<PathBuf, String> {
        if project.is_empty()
            || project.contains('/')
            || project.contains('\\')
            || project.contains("..")
            || project.starts_with('.')
        {
            return Err(format!("invalid project id: {project}"));
        }
        Ok(self.publication_root.join(project))
    }

    pub fn executor(&self) -> &Executor {
        &self.executor
    }

    pub fn read_project_file(&self, project: &str, rel: &str) -> Result<Vec<u8>, String> {
        let sandbox = self.sandbox_for(project)?;
        if rel.is_empty() || rel.starts_with('/') || rel.starts_with('\\') || rel.contains("..") {
            return Err("invalid path".into());
        }
        let full = sandbox.join(rel);
        let canonical_full = std::fs::canonicalize(&full).map_err(|e| e.to_string())?;
        let canonical_sandbox = std::fs::canonicalize(&sandbox).map_err(|e| e.to_string())?;
        if !canonical_full.starts_with(&canonical_sandbox) {
            return Err("outside sandbox".into());
        }
        std::fs::read(&canonical_full).map_err(|e| e.to_string())
    }
}
