use clap::Parser;

/// Recall what you did on the last working day - or be nosy and find what someone else did.
#[derive(Parser, Debug)]
#[command(name = "git-standup")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Specify author to restrict search to (use "all" to see all authors)
    #[arg(short = 'a', long)]
    pub author: Option<String>,

    /// Specify branch to restrict search to (unset: all branches)
    #[arg(short = 'b', long)]
    pub branch: Option<String>,

    /// Specify weekday range (e.g., "MON-FRI" or "SUN-THU")
    #[arg(short = 'w', long, default_value = "MON-FRI")]
    pub weekdays: String,

    /// Specify the depth of recursive directory search
    #[arg(short = 'm', long, default_value = "2")]
    pub max_depth: usize,

    /// Force recursion up to specified depth even when git repository found earlier
    #[arg(short = 'F', long)]
    pub force_recursion: bool,

    /// Toggle inclusion of symbolic links in recursive directory search
    #[arg(short = 'L', long)]
    pub follow_links: bool,

    /// Specify the number of days back to include
    #[arg(short = 'd', long)]
    pub days: Option<u32>,

    /// Specify the number of days back until which standup should run
    #[arg(short = 'u', long)]
    pub until_days: Option<u32>,

    /// Specify the date format for git log (relative, local, default, iso, rfc, short, raw)
    #[arg(short = 'D', long, default_value = "relative")]
    pub date_format: String,

    /// Show commits after this date (YYYY-MM-DD)
    #[arg(short = 'A', long)]
    pub after: Option<String>,

    /// Show commits before this date (YYYY-MM-DD)
    #[arg(short = 'B', long)]
    pub before: Option<String>,

    /// Show if commit is GPG signed (G) or not (N)
    #[arg(short = 'g', long)]
    pub gpg_sign: bool,

    /// Fetch the latest commits beforehand
    #[arg(short = 'f', long)]
    pub fetch: bool,

    /// Silence the no activity message
    #[arg(short = 's', long)]
    pub silent: bool,

    /// Show diff-stat for every matched commit
    #[arg(short = 'c', long)]
    pub diff_stat: bool,

    /// Generate a report file (git-standup-report.txt)
    #[arg(short = 'r', long)]
    pub report: bool,

    /// Display the author date instead of the committer date
    #[arg(short = 'R', long)]
    pub author_date: bool,
}
