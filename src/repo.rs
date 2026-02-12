use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

use crate::cli::Args;

pub fn find_git_repos(base_dir: &Path, args: &Args) -> Result<Vec<PathBuf>> {
    let mut repos = Vec::new();

    // Check for whitelist file
    let whitelist_path = base_dir.join(".git-standup-ignore");
    let search_paths: Vec<PathBuf> = if whitelist_path.exists() {
        fs::read_to_string(&whitelist_path)?
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| base_dir.join(line.trim()))
            .collect()
    } else {
        vec![base_dir.to_path_buf()]
    };

    // Check if current directory is a git repo and not forcing recursion
    let is_git_repo = base_dir.join(".git").exists();
    if is_git_repo && !args.force_recursion {
        repos.push(base_dir.to_path_buf());
        return Ok(repos);
    }

    // Recursively search for git repositories
    for search_path in search_paths {
        let walker = WalkDir::new(&search_path)
            .max_depth(args.max_depth)
            .follow_links(args.follow_links)
            .into_iter()
            .filter_entry(|e| {
                // Skip hidden directories except .git
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') || name == ".git"
            });

        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_name() == ".git" && entry.file_type().is_dir() {
                if let Some(parent) = entry.path().parent() {
                    repos.push(parent.to_path_buf());
                }
            }
        }
    }

    // If we're inside a git repo but didn't find any through walking
    if repos.is_empty() {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(base_dir)
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
                repos.push(PathBuf::from(root));
            }
        }
    }

    Ok(repos)
}
