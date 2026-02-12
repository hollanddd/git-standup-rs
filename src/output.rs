use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::cli::Args;
use crate::git::BranchCommits;

pub fn print_output(
    repo_path: &Path,
    branch_commits: &[BranchCommits],
    author: &str,
    args: &Args,
) -> Result<()> {
    let display_path = repo_path.display().to_string();

    if !branch_commits.is_empty() {
        println!("{}", display_path.yellow().bold().underline());
        for bc in branch_commits {
            println!("  {}", bc.branch.green());
            for line in bc.output.lines() {
                println!("  {}", line);
            }
        }
    } else if !args.silent {
        println!("{}", display_path.yellow().bold().underline());
        if author == ".*" {
            println!("{}", "No commits found during this period.".yellow());
        } else {
            println!(
                "{}",
                format!("No commits from {} during this period.", author).yellow()
            );
        }
    }

    Ok(())
}

pub fn write_report(
    report_path: &Path,
    repo_path: &Path,
    branch_commits: &[BranchCommits],
    _author: &str,
    args: &Args,
) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(report_path)
        .context("Failed to open report file")?;

    let display_path = repo_path.display().to_string();

    if !branch_commits.is_empty() {
        writeln!(file, "{}", display_path)?;
        for bc in branch_commits {
            writeln!(file, "  {}", bc.branch)?;
            let clean_output = strip_ansi_codes(&bc.output);
            for line in clean_output.lines() {
                writeln!(file, "  {}", line)?;
            }
        }
    } else if !args.silent {
        writeln!(file, "{}\nNo activity found!\n", display_path)?;
    }

    Ok(())
}

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }

    result
}
