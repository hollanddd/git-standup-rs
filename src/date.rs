use anyhow::Result;
use chrono::{Datelike, Local, Weekday};

use crate::cli::Args;

pub fn calculate_since(args: &Args) -> Result<String> {
    // If -d flag is specified, use that
    if let Some(days) = args.days {
        return Ok(format!("{} days ago", days));
    }

    // If -A (after) is specified, use that directly
    if args.after.is_some() {
        return Ok("1970-01-01".to_string()); // Use epoch as since, after will filter
    }

    // Parse weekday range
    let parts: Vec<&str> = args.weekdays.split('-').collect();
    let week_start = parse_weekday(parts.first().unwrap_or(&"MON"))?;
    let week_end = parse_weekday(parts.get(1).unwrap_or(&"FRI"))?;

    let today = Local::now().weekday();

    // If today is the start of the work week, show commits from last week end
    if today == week_start {
        Ok(format!("last {}", weekday_name(week_end)))
    } else {
        Ok("yesterday".to_string())
    }
}

pub fn parse_weekday(s: &str) -> Result<Weekday> {
    match s.to_uppercase().as_str() {
        "MON" | "MONDAY" => Ok(Weekday::Mon),
        "TUE" | "TUESDAY" => Ok(Weekday::Tue),
        "WED" | "WEDNESDAY" => Ok(Weekday::Wed),
        "THU" | "THURSDAY" => Ok(Weekday::Thu),
        "FRI" | "FRIDAY" => Ok(Weekday::Fri),
        "SAT" | "SATURDAY" => Ok(Weekday::Sat),
        "SUN" | "SUNDAY" => Ok(Weekday::Sun),
        _ => anyhow::bail!("Invalid weekday: {}", s),
    }
}

fn weekday_name(w: Weekday) -> &'static str {
    match w {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}
