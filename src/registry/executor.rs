use std::path::Path;

pub struct Executor;

impl Executor {
    pub const fn new() -> Self {
        Self
    }

    pub async fn run_bash(&self, cwd: &Path, command: String) -> Result<(), String> {
        let cwd_str = cwd.display().to_string();
        let cwd = cwd.to_path_buf();
        let cmdline = format!("bash -c {command:?}");
        eprintln!("[exec] {cmdline}  (cwd: {cwd_str})");
        let result = tokio::task::spawn_blocking(move || {
            std::process::Command::new("bash")
                .current_dir(&cwd)
                .arg("-c")
                .arg(&command)
                .status()
        })
        .await;
        Self::interpret(result).map_err(|e| format!("{cmdline}: {e}"))
    }

    pub async fn run_command(
        &self,
        cwd: &Path,
        program: &str,
        args: &[&str],
    ) -> Result<(), String> {
        let cwd_str = cwd.display().to_string();
        let cwd = cwd.to_path_buf();
        let program = program.to_string();
        let args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let cmdline = format!("{program} {}", args_vec.join(" "));
        eprintln!("[exec] {cmdline}  (cwd: {cwd_str})");
        let result = tokio::task::spawn_blocking(move || {
            std::process::Command::new(&program)
                .current_dir(&cwd)
                .args(&args_vec)
                .status()
        })
        .await;
        Self::interpret(result).map_err(|e| format!("{cmdline}: {e}"))
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
