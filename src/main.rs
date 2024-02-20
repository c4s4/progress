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
    let text = match fs::read_to_string(args.file) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("ERROR reading text file: {err}");
            process::exit(1);
        }
    };
    // count characters
    let text_size = text.chars().count();
    // start date
    let date_start = match dateparser::parse(&args.start) {
        Ok(date) => date,
        Err(err) => {
            eprintln!("ERROR parsing start date: {err}");
            process::exit(1);
        }
    };
    // due date
    let date_due = match dateparser::parse(&args.date) {
        Ok(date) => date,
        Err(err) => {
            eprintln!("ERROR parsing due date: {err}");
            process::exit(1);
        }
    };
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
