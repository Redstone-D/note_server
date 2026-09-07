use crate::registry::{Executor, Script};
use hotaru::akari::Value;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::LazyLock;

const DEPLOYABLE_SUBDIR: &str = "latex_target";

static SOURCES_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    let raw = std::env::var("NOTES_SOURCES_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./sources"));
    // Resolve to absolute against the server's startup CWD so relative env
    // values ("./sources") don't reinterpret against the sandbox CWD that
    // `run_command` uses. `canonicalize` would need the dir to exist; a plain
    // join with `current_dir()` works even before first deploy.
    if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir().map(|c| c.join(&raw)).unwrap_or(raw)
    }
});

pub struct FolderSync;

impl Script for FolderSync {
    fn id(&self) -> &'static str {
        "folder-sync"
    }

    fn run<'a>(
        &'a self,
        executor: &'a Executor,
        sandbox: PathBuf,
        _config: Value,
        _event: Value,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let folder = sandbox
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| "folder-sync: sandbox has no folder name".to_string())?;
            let src = SOURCES_ROOT.join(folder).join(DEPLOYABLE_SUBDIR);
            let src_arg = format!("{}/", src.display());
            let dest = sandbox.display().to_string();
            executor
                .run_command(
                    &sandbox,
                    "rsync",
                    &["-a", "--delete", &src_arg, &dest],
                )
                .await
        })
    }
}
