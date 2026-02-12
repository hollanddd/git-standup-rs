use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

use crate::cli::Args;
use crate::date::calculate_since;
use crate::output::{print_output, write_report};

/// A branch name paired with its git log output.
pub struct BranchCommits {
    pub branch: String,
    pub output: String,
}

pub fn run_standup(repo_path: &Path, args: &Args, report_path: &Path) -> Result<()> {
    let repo_name = repo_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Fetch if requested
    if args.fetch {
        println!(
            "{}",
            format!("Fetching commits in {}", repo_name.underline()).green()
        );
        let _ = Command::new("git")
            .args(["fetch", "--all"])
            .current_dir(repo_path)
            .output();
    }

    // Get the author (default to git config user.name)
    let author = get_author(repo_path, &args.author)?;

    // Calculate the since date
    let since = calculate_since(args)?;

    // Get branches to query
    let branches = match &args.branch {
        Some(branch) => vec![branch.clone()],
        None => get_branches(repo_path)?,
    };

    // Collect commits per branch
    let mut branch_commits = Vec::new();
    for branch in &branches {
        let output = run_git_log(repo_path, branch, &author, &since, args)?;
        if !output.trim().is_empty() {
            branch_commits.push(BranchCommits {
                branch: branch.clone(),
                output,
            });
        }
    }

    // Output results
    if args.report {
        println!("Generating report for: {}", repo_path.display());
        write_report(report_path, repo_path, &branch_commits, &author, args)?;
    } else {
        print_output(repo_path, &branch_commits, &author, args)?;
    }

    Ok(())
}

fn get_branches(repo_path: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["for-each-ref", "--format=%(refname:short)", "refs/heads/"])
        .current_dir(repo_path)
        .output()
        .context("Failed to list branches")?;

    let branches: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect();

    Ok(branches)
}

fn get_author(repo_path: &Path, author_arg: &Option<String>) -> Result<String> {
    match author_arg {
        Some(a) if a == "all" => Ok(".*".to_string()),
        Some(a) => Ok(a.clone()),
        None => {
            let output = Command::new("git")
                .args(["config", "user.name"])
                .current_dir(repo_path)
                .output()
                .context("Failed to get git user.name")?;
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
    }
}

fn run_git_log(
    repo_path: &Path,
    branch: &str,
    author: &str,
    since: &str,
    args: &Args,
) -> Result<String> {
    let mut cmd_args = vec!["--no-pager".to_string(), "log".to_string()];

    cmd_args.push(branch.to_string());
    cmd_args.push("--no-merges".to_string());
    cmd_args.push(format!("--since={}", since));

    // Until filter
    if let Some(until_days) = args.until_days {
        cmd_args.push(format!("--until={} days ago", until_days));
    } else if let Some(before) = &args.before {
        cmd_args.push(format!("--until={}", before));
    }

    // After filter
    if let Some(after) = &args.after {
        cmd_args.push(format!("--after={}", after));
    }

    cmd_args.push(format!("--author={}", author));
    cmd_args.push("--abbrev-commit".to_string());
    cmd_args.push("--oneline".to_string());

    // Pretty format
    let date_placeholder = if args.author_date { "%ad" } else { "%cd" };
    let mut format = format!(
        "%Cred%h%Creset - %s %Cgreen({}) %C(bold blue)<%an>%Creset",
        date_placeholder
    );
    if args.gpg_sign {
        format.push_str(" %C(yellow)gpg: %G?%Creset");
    }
    cmd_args.push(format!("--pretty=format:{}", format));

    cmd_args.push(format!("--date={}", args.date_format));
    cmd_args.push("--color=always".to_string());

    // Diff stat
    if args.diff_stat {
        cmd_args.push("--stat".to_string());
    }

    let output = Command::new("git")
        .args(&cmd_args)
        .current_dir(repo_path)
        .output()
        .context("Failed to run git log")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
