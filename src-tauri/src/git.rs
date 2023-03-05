use std::{path::Path, error::Error};
use git2::{Repository, IndexAddOption, Signature, Commit, ObjectType};
use crate::dirs;

pub fn is_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

pub fn current_repo(path: &Path) -> Result<String, Box<dyn Error>> {
    
    let repo = git2::Repository::open(path)?;
    
    let binding = repo.find_remote("origin").expect("remote origin doesnt exist.");
    Ok(binding.url().expect("invalid git remote url formatting, must be utf-8.").to_string())
}

pub fn fetch_and_merge(branch: String) -> Result<(), Box<dyn Error>> {
    let repo = git2::Repository::open(&dirs::exec())?;
    
    add_and_commit(repo)?;
    
    // execute::color(&format!("git fetch origin {}", branch))?;
    // execute::color(&format!(" git merge -s recursive -X theirs origin/{}", branch))?;
    // execute::no_output(&format!("git switch {}", branch))?;
    
    Ok(())
}

fn add_and_commit(repo: Repository) -> Result<(), Box<dyn Error>> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let parent_commit = find_last_commit(&repo)?;
    let signature = Signature::now("someone", "someone@some.mail")?;
    repo.commit(Some("HEAD"), &signature, &signature, "commit", &tree, &[&parent_commit])?;
    
    Ok(())
}

fn find_last_commit(repo: &Repository) -> Result<Commit, Box<dyn Error>> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    Ok(obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))?)
}

fn fetch(repo: &Repository, branch: String) -> Result<(), Box<dyn Error>> {
    repo.find_remote("origin")?.fetch(&[branch], None, None)?;
    
    Ok(())
}