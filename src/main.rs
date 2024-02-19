use chrono::prelude::Utc;
use clap::Parser;
use dateparser;
use std::fs;
use std::process;

/// Print text progress for character count, start and due date
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Article file
    file: String,
    /// Character count
    count: u32,
    /// Start date ISO format (yyyy-mm-dd)
    start: String,
    /// Due date in ISO format (yyyy-mm-dd)
    date: String,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // load text
    let result = fs::read_to_string(args.file);
    if result.is_err() {
        error(format!("reading text file: {}", result.as_ref().err().unwrap()).as_str());
    }
    let text = result.unwrap();
    // count characters
    let text_size = text.chars().count();
    // start date
    let result = dateparser::parse(&args.start);
    if result.is_err() {
        error(format!("parsing start date: {}", result.as_ref().err().unwrap()).as_str());
    }
    let date_start = result.unwrap();
    // due date
    let result = dateparser::parse(&args.date);
    if result.is_err() {
        error(format!("parsing due date: {}", result.as_ref().err().unwrap()).as_str());
    }
    let date_due = result.unwrap();
    // compute number of days to due date
    let days_left = date_due.signed_duration_since(Utc::now()).num_days();
    // total number of days
    let days_total = date_due.signed_duration_since(date_start).num_days();
    // days elapsed
    let days_elapsed = Utc::now().signed_duration_since(date_start).num_days();
    // progress
    let percent_done = (text_size as f64 / args.count as f64) * 100.0;
    // should be
    let percent_should = (days_elapsed as f64 / days_total as f64) * 100.0;
    // delta
    let percent_delta = percent_done - percent_should;
    // print progress
    println!(
        "text size: {}, due size: {}, due for: {}, days left: {}",
        text_size, args.count, args.date, days_left
    );
    println!(
        "progress: {:.0}%, should be: {:.0}%, delta: {:.0}%",
        percent_done, percent_should, percent_delta
    );
}

/// Print error message and exit
fn error(msg: &str) {
    eprintln!("ERROR {msg}");
    process::exit(1);
}
