use crate::project::ProjectConfig;
use hotaru::akari::Value;
use std::collections::HashMap;
use std::sync::LazyLock;

// The project id is also the local checkout folder name under
// `$NOTES_SOURCES_ROOT`. `github-sync` clones from `repo` on first deploy and
// `git pull --ff-only`s on subsequent deploys, then rsyncs `latex_target/`
// into the sandbox. Use `folder-sync` (with an empty config) for projects
// whose checkout is managed manually.
pub static PROJECTS: LazyLock<HashMap<&'static str, ProjectConfig>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for (id, repo) in [
        (
            "real-analysis",
            "https://github.com/Redstone-D/real_analysis_notes.git",
        ),
        // PLACEHOLDER — replace with real repo URL when it exists.
        (
            "functional-analysis",
            "https://github.com/Redstone-D/functional_analysis_notes.git",
        ),
    ] {
        let mut cfg = Value::new_dict();
        cfg.set("repo", repo);
        map.insert(id, ProjectConfig::new(id, "github-sync", cfg));
    }
    map
});
