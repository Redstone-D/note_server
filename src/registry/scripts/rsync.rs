use crate::registry::Executor;
use std::path::Path;

pub struct Rsync;

impl Rsync {
    pub async fn sync(executor: &Executor, src: &Path, dest: &Path) -> Result<(), String> {
        let src_arg = format!("{}/", src.display());
        let dest_arg = dest.display().to_string();
        executor
            .run_command(dest, "rsync", &["-a", "--delete", &src_arg, &dest_arg])
            .await
    }
}
