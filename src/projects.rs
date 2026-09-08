use crate::project::ProjectConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static CONFIG_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    let raw = std::env::var("NOTES_CONFIG_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./config"));
    if raw.is_absolute() {
        raw
    } else {
        std::env::current_dir().map(|c| c.join(&raw)).unwrap_or(raw)
    }
});

// Loaded once at first access by scanning `$NOTES_CONFIG_ROOT/*/config.json`.
// Add or remove a project = create or delete a directory under CONFIG_ROOT,
// then restart the server.
pub static PROJECTS: LazyLock<HashMap<String, ProjectConfig>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let entries = match std::fs::read_dir(&*CONFIG_ROOT) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("[projects] cannot read {}: {e}", CONFIG_ROOT.display());
            return map;
        }
    };
    for entry in entries.flatten() {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if !meta.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let config_path = entry.path().join(ProjectConfig::FILE_NAME);
        match ProjectConfig::read(&config_path) {
            Ok(cfg) => {
                map.insert(name, cfg);
            }
            Err(e) => {
                eprintln!(
                    "[projects] skipping {}: {e}",
                    config_path.display()
                );
            }
        }
    }
    map
});
