use std::path::Path;
use crate::*;

pub fn is_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

pub fn current_repo() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let binding = execute::string_return("git remote -v")?;
    let output = binding
        .lines()
        .map(|line| {
            line.split_whitespace().collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();
    
    if output[0][0] == "origin" {
        Ok(Some(output[0][1].to_string()))
    } else {
        Ok(None)
    }
    
}