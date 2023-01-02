use git2::Repository;

pub fn is_repo(path: &str) -> bool {
    match Repository::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
