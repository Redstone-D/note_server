use hotaru::akari::Value;
use std::path::Path;

const APPROVED_EXTENSIONS: &[&str] = &["pdf", "html", "png"];

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub files: Vec<File>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectTree {
    groups: Vec<Group>,
}

impl ProjectTree {
    pub fn walk(root: impl AsRef<Path>) -> Result<Self, String> {
        let root = root.as_ref();

        let mut root_files = Vec::new();
        let mut named_groups = Vec::new();
        for entry in std::fs::read_dir(root)
            .map_err(|e| format!("read {}: {e}", root.display()))?
        {
            let entry = entry.map_err(|e| e.to_string())?;
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            if metadata.file_type().is_symlink() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if !Self::is_safe_name(&name) {
                continue;
            }
            if metadata.is_dir() {
                let files = Self::files_under(&entry.path())?;
                if !files.is_empty() {
                    named_groups.push(Group { name, files });
                }
            } else if metadata.is_file()
                && name != crate::project::ProjectManifest::FILE_NAME
                && Self::has_approved_extension(&name)
            {
                root_files.push(File { name });
            }
        }
        root_files.sort_by(|a, b| a.name.cmp(&b.name));
        named_groups.sort_by(|a, b| a.name.cmp(&b.name));

        let mut groups = Vec::with_capacity(named_groups.len() + 1);
        if !root_files.is_empty() {
            groups.push(Group { name: String::new(), files: root_files });
        }
        groups.extend(named_groups);
        Ok(Self { groups })
    }

    pub fn groups(&self) -> &[Group] {
        &self.groups
    }

    pub fn render_with_urls(&self, project: &str) -> Value {
        let mut list = Value::new_list();
        for group in &self.groups {
            let mut g = Value::new_dict();
            g.set("name", group.name.clone());
            let mut files = Value::new_list();
            for file in &group.files {
                let mut f = Value::new_dict();
                f.set("name", file.name.clone());
                let href = if group.name.is_empty() {
                    format!("/{}/{}", project, file.name)
                } else {
                    format!("/{}/{}/{}", project, group.name, file.name)
                };
                f.set("href", href);
                files.push(f);
            }
            g.set("files", files);
            list.push(g);
        }
        list
    }

    pub fn files_in(&self, group: &str) -> &[File] {
        self.groups
            .iter()
            .find(|g| g.name == group)
            .map(|g| g.files.as_slice())
            .unwrap_or(&[])
    }

    fn files_under(dir: &Path) -> Result<Vec<File>, String> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if !Self::is_safe_name(&name) || !Self::has_approved_extension(&name) {
                continue;
            }
            files.push(File { name });
        }
        files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(files)
    }

    fn is_safe_name(name: &str) -> bool {
        !name.is_empty()
            && !name.starts_with('.')
            && !name.contains('/')
            && !name.contains('\\')
            && name != ".."
    }

    fn has_approved_extension(name: &str) -> bool {
        match Path::new(name).extension().and_then(|s| s.to_str()) {
            Some(ext) => APPROVED_EXTENSIONS
                .iter()
                .any(|a| a.eq_ignore_ascii_case(ext)),
            None => false,
        }
    }
}

impl From<&File> for Value {
    fn from(file: &File) -> Value {
        let mut v = Value::new_dict();
        v.set("name", file.name.clone());
        v
    }
}

impl From<&Group> for Value {
    fn from(group: &Group) -> Value {
        let mut v = Value::new_dict();
        v.set("name", group.name.clone());
        let mut files = Value::new_list();
        for f in &group.files {
            files.push(Value::from(f));
        }
        v.set("files", files);
        v
    }
}

impl From<&ProjectTree> for Value {
    fn from(tree: &ProjectTree) -> Value {
        let mut list = Value::new_list();
        for g in &tree.groups {
            list.push(Value::from(g));
        }
        list
    }
}

impl TryFrom<Value> for File {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let name = value
            .try_get("name")
            .map_err(|_| "file: missing 'name'".to_string())?
            .string();
        if name.is_empty() {
            return Err("file: empty 'name'".into());
        }
        Ok(Self { name })
    }
}

impl TryFrom<Value> for Group {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let name = value
            .try_get("name")
            .map_err(|_| "group: missing 'name'".to_string())?
            .string();
        if name.is_empty() {
            return Err("group: empty 'name'".into());
        }
        let files = match value.try_get("files").ok().cloned() {
            Some(Value::List(items)) => items
                .into_iter()
                .map(File::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            _ => Vec::new(),
        };
        Ok(Self { name, files })
    }
}

impl TryFrom<Value> for ProjectTree {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(items) => {
                let groups = items
                    .into_iter()
                    .map(Group::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self { groups })
            }
            other => Err(format!("project tree: expected list, got {}", other.type_of())),
        }
    }
}
