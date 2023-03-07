use clap::Parser;
use chrono::{Local, NaiveDateTime, NaiveTime, Utc};

mod font;
mod countdown;

#[derive(Parser, Debug)]
struct Args {
    /// the duration to count down fro
    duration: String,

    /// count up instead of down
    #[arg(long)]
    up: bool,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let target_duration = if let Ok(val) = parse_time(&args.duration) {
        val as usize
    } else {
        parse_duration(&args.duration).unwrap() as usize
    };

    countdown::Countdown::new(target_duration, args.up).run().await;
}

// Parse a string like "1:30" or "1:30 pm" into a number of seconds.
fn parse_time(date: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let target_time = match NaiveTime::parse_from_str(date, "%I:%M %p") {
        Ok(dt) => dt,
        Err(_) => NaiveTime::parse_from_str(date, "%H:%M")?,
    };

    let mut target_datetime = NaiveDateTime::new(Utc::now().naive_local().date(), target_time);

    // The time of day has already passed, so target tomorrow.
    if target_datetime < Local::now().naive_local() {
        target_datetime = target_datetime + chrono::Duration::days(1);
    }

    let duration = target_datetime - Local::now().naive_local();

    Ok(duration.num_seconds())
}

// Parse a string like "1h30m" or "1h30m10s" into a number of seconds.
fn parse_duration(duration_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut total_seconds = 0;

    let mut num_buffer = String::new();
    for c in duration_str.chars() {
        match c {
            'h' => {
                let hours = num_buffer.parse::<u64>()?;
                total_seconds += hours * 60 * 60;
                num_buffer.clear();
            }
            'm' => {
                let minutes = num_buffer.parse::<u64>()?;
                total_seconds += minutes * 60;
                num_buffer.clear();
            }
            's' => {
                let seconds = num_buffer.parse::<u64>()?;
                total_seconds += seconds;
                num_buffer.clear();
            }
            _ => {
                if c.is_ascii_digit() {
                    num_buffer.push(c);
                } else {
                    return Err("failed to parse".into());
                }
            }
        }
    }

    // Handle the case where the string ends with a number (without a trailing letter)
    if !num_buffer.is_empty() {
        let seconds = num_buffer.parse::<u64>()?;
        total_seconds += seconds;
    }

    Ok(total_seconds)
}
