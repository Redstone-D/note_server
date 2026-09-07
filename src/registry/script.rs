use super::Executor;
use hotaru::akari::Value;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

pub trait Script: Send + Sync {
    fn id(&self) -> &'static str;

    fn run<'a>(
        &'a self,
        executor: &'a Executor,
        sandbox: PathBuf,
        config: Value,
        event: Value,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
}
