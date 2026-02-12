mod cli;
mod date;
mod git;
mod output;
mod repo;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::fs;

use cli::Args;
use git::run_standup;
use repo::find_git_repos;

fn main() -> Result<()> {
    let args = Args::parse();

    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let report_path = current_dir.join("git-standup-report.txt");

    // Remove existing report file if generating new one
    if args.report && report_path.exists() {
        fs::remove_file(&report_path).context("Failed to remove existing report file")?;
    }

    // Find all git repositories
    let repos = find_git_repos(&current_dir, &args)?;

    if repos.is_empty() {
        eprintln!("{}", "You must be inside a git repository!".yellow());
        return Ok(());
    }

    for repo in repos {
        run_standup(&repo, &args, &report_path)?;
    }

    Ok(())
}
