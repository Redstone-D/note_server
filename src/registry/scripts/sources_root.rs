use std::path::{Path, PathBuf};
use std::sync::LazyLock;

const DEPLOYABLE_SUBDIR: &str = "latex_target";

pub struct SourcesRoot {
    root: PathBuf,
}

impl SourcesRoot {
    pub fn global() -> &'static Self {
        static INSTANCE: LazyLock<SourcesRoot> = LazyLock::new(|| {
            let raw = std::env::var("NOTES_SOURCES_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./sources"));
            // Resolve to absolute against the server's startup CWD so relative
            // env values ("./sources") don't reinterpret against the sandbox
            // CWD that `run_command` uses. `canonicalize` would need the dir to
            // exist; join with `current_dir()` works even before first deploy.
            let root = if raw.is_absolute() {
                raw
            } else {
                std::env::current_dir().map(|c| c.join(&raw)).unwrap_or(raw)
            };
            SourcesRoot { root }
        });
        &INSTANCE
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn checkout_dir(&self, project: &str) -> PathBuf {
        self.root.join(project)
    }

    pub fn deployable_dir(&self, project: &str) -> PathBuf {
        self.checkout_dir(project).join(DEPLOYABLE_SUBDIR)
    }
}
