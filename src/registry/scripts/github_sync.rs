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
    if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir().map(|c| c.join(&raw)).unwrap_or(raw)
    }
});

pub struct GithubSync;

impl Script for GithubSync {
    fn id(&self) -> &'static str {
        "github-sync"
    }

    fn run<'a>(
        &'a self,
        executor: &'a Executor,
        sandbox: PathBuf,
        config: Value,
        _event: Value,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let repo = config.get("repo").string();
            if repo.is_empty() {
                return Err("github-sync: missing config.repo".into());
            }
            let folder = sandbox
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| "github-sync: sandbox has no folder name".to_string())?
                .to_string();

            executor.ensure_dir(&SOURCES_ROOT).await?;
            let checkout = SOURCES_ROOT.join(&folder);

            if checkout.join(".git").exists() {
                executor
                    .run_command(&checkout, "git", &["pull", "--ff-only"])
                    .await?;
            } else {
                executor
                    .run_command(
                        &SOURCES_ROOT,
                        "git",
                        &["clone", "--depth", "1", &repo, &folder],
                    )
                    .await?;
            }

            let src = checkout.join(DEPLOYABLE_SUBDIR);
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
