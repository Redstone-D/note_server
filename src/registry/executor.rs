use std::path::Path;

pub struct Executor;

impl Executor {
    pub const fn new() -> Self {
        Self
    }

    pub async fn run_bash(&self, cwd: &Path, command: String) -> Result<(), String> {
        let cwd = cwd.to_path_buf();
        let result = tokio::task::spawn_blocking(move || {
            std::process::Command::new("bash")
                .current_dir(&cwd)
                .arg("-c")
                .arg(&command)
                .status()
        })
        .await;
        Self::interpret(result)
    }

    pub async fn run_command(
        &self,
        cwd: &Path,
        program: &str,
        args: &[&str],
    ) -> Result<(), String> {
        let cwd = cwd.to_path_buf();
        let program = program.to_string();
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let result = tokio::task::spawn_blocking(move || {
            std::process::Command::new(&program)
                .current_dir(&cwd)
                .args(&args)
                .status()
        })
        .await;
        Self::interpret(result)
    }

    pub async fn ensure_dir(&self, path: &Path) -> Result<(), String> {
        let path = path.to_path_buf();
        tokio::task::spawn_blocking(move || std::fs::create_dir_all(&path))
            .await
            .map_err(|e| format!("mkdir join: {e}"))?
            .map_err(|e| format!("mkdir: {e}"))
    }

    fn interpret(
        result: Result<std::io::Result<std::process::ExitStatus>, tokio::task::JoinError>,
    ) -> Result<(), String> {
        match result {
            Ok(Ok(s)) if s.success() => Ok(()),
            Ok(Ok(s)) => Err(format!("exit {}", s.code().unwrap_or(-1))),
            Ok(Err(e)) => Err(e.to_string()),
            Err(e) => Err(format!("join: {e}")),
        }
    }
}
