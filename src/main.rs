use clap::Parser;
use std::fs;
use dateparser;
use chrono::prelude::Utc;

/// Print text progress for character count and due date
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
    let text = fs::read_to_string(args.file).unwrap();
    // count characters
    let text_size = text.chars().count();
    // start date
    let date_start = dateparser::parse(&args.start).unwrap();
    // due date
    let date_due = dateparser::parse(&args.date).unwrap();
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
    // FIXME
    println!("text size: {}, due size: {}, due for: {}, days left: {}", text_size, args.count, args.date, days_left);
    println!("progress: {:.0}%, should be: {:.0}%, delta: {:.0}%", percent_done, percent_should, percent_delta);
}
