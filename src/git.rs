use std::path::Path;


pub fn is_repo(path: &Path) -> bool {
    path.join(".git").exists()
}
