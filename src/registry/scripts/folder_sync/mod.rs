use crate::registry::scripts::{Rsync, SourcesRoot};
use crate::registry::{Executor, Script};
use hotaru::akari::Value;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

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
            let src = SourcesRoot::global().deployable_dir(folder);
            Rsync::sync(executor, &src, &sandbox).await
        })
    }
}
