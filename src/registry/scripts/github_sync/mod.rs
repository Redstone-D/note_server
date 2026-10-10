use crate::registry::scripts::{Rsync, SourcesRoot};
use crate::registry::{Executor, Script};
use hotaru::akari::Value;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

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

            let sources = SourcesRoot::global();
            tokio::fs::create_dir_all(sources.root())
                .await
                .map_err(|e| format!("mkdir sources root: {e}"))?;
            let checkout = sources.checkout_dir(&folder);

            if checkout.join(".git").exists() {
                executor
                    .run_command(&checkout, "git", &["pull", "--ff-only"])
                    .await?;
            } else {
                executor
                    .run_command(
                        sources.root(),
                        "git",
                        &["clone", "--depth", "1", &repo, &folder],
                    )
                    .await?;
            }

            let src = sources.deployable_dir(&folder);
            Rsync::sync(executor, &src, &sandbox).await
        })
    }
}
