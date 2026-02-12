# git-standup-rs

Recall what you did on the last working day — or be nosy and find what someone else did. A Rust port of [git-standup](https://github.com/kamranahmedse/git-standup).

## Install

### Homebrew

```bash
brew install hollanddd/homebrew-tap/git-standup-rs
```

### Cargo

```bash
cargo install git-standup
```

### From source

```bash
git clone https://github.com/hollanddd/git-standup-rs.git
cd git-standup-rs
cargo install --path .
```

## Usage

Run inside a git repository (or a directory containing git repos):

```bash
git-standup
```

### Options

```
-a, --author <AUTHOR>        Specify author to restrict search to (use "all" to see all authors)
-b, --branch <BRANCH>        Specify branch to restrict search to (unset: all branches)
-w, --weekdays <WEEKDAYS>    Specify weekday range [default: MON-FRI]
-m, --max-depth <MAX_DEPTH>  Depth of recursive directory search [default: 2]
-F, --force-recursion        Force recursion even when a git repo is found earlier
-L, --follow-links           Follow symbolic links during directory search
-d, --days <DAYS>            Number of days back to include
-u, --until-days <DAYS>      Number of days back until which standup should run
-D, --date-format <FORMAT>   Date format for git log [default: relative]
-A, --after <DATE>           Show commits after this date (YYYY-MM-DD)
-B, --before <DATE>          Show commits before this date (YYYY-MM-DD)
-g, --gpg-sign               Show if commit is GPG signed (G) or not (N)
-f, --fetch                  Fetch the latest commits beforehand
-s, --silent                 Silence the no activity message
-c, --diff-stat              Show diff-stat for every matched commit
-r, --report                 Generate a report file (git-standup-report.txt)
-R, --author-date            Display the author date instead of the committer date
```

### Examples

```bash
# See what you did last working day
git-standup

# See what a specific author did
git-standup -a "John Doe"

# See what everyone did in the last 7 days
git-standup -a all -d 7

# Check a specific branch
git-standup -b main

# Generate a report file
git-standup -r
```

## License

MIT
