use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use cron_descriptor::cronparser::cron_expression_descriptor;
use time::format_description::well_known::Rfc3339;
use time::macros;
use time::OffsetDateTime;

use crate::format_utils;

pub fn unix_timestamp(milli: bool, dt: Option<String>) {
    let now = match dt {
        Some(ref datetime_str) => {
            // Try parsing with multiple formats
            match DateTime::parse_from_rfc3339(datetime_str)
                .or_else(|_| DateTime::parse_from_rfc2822(datetime_str))
                .or_else(|_| DateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S %z"))
                .or_else(|_| DateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S"))
            {
                Ok(parsed_dt) => parsed_dt,
                Err(_) => {
                    println!("Could not parse the provided datetime string.");
                    return;
                }
            }
        }
        None => Utc::now().into(),
    };

    if milli {
        let timestamp_milliseconds = now.timestamp_millis();
        println!("{}", timestamp_milliseconds);
    } else {
        let timestamp_seconds = now.timestamp();
        println!("{}", timestamp_seconds);
    }
}


pub fn date_time(local: bool, rfc: bool, unix: Option<u64>) {

    // If a Unix timestamp is provided, parse it
    let date_time: DateTime<Utc> = if let Some(unix_time) = unix {
        DateTime::from_timestamp(unix_time as i64, 0).unwrap()
    } else {
        Utc::now() // Use the current time if no Unix timestamp is provided
    };

    if local {
         // Convert UTC time to local time
        if rfc {
            let formatted = date_time.with_timezone(&Local).to_rfc3339(); // Use chrono's RFC 3339 method
            println!("{}", formatted);
        } else {
            let formatted = date_time.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(); // Custom format
            println!("{}", formatted);
        }
    } else {
        if rfc {
            let formatted = date_time.to_rfc3339(); // Use chrono's RFC 3339 method
            println!("{}", formatted);
        } else {
            let formatted = date_time.format("%Y-%m-%d %H:%M:%S").to_string(); // Custom format
            println!("{}", formatted);
        }
    };
}


pub fn chron_tostring(cron_expr: String) {
    let description = cron_expression_descriptor::get_description_cron(&cron_expr);
    println!("{}", description);
}

pub fn countdown(input: String) {
    // Parse input into minutes and seconds
    let countdown_time = match parse_duration(&input) {
        Some(duration) => duration,
        None => {
            println!("Please provide a valid time in the format MM:SS or just seconds.");
            return;
        }
    };

    // Countdown loop
    let mut total_seconds = countdown_time.as_secs();
    while total_seconds > 0 {
        format_utils::clear_terminal(); // Assume you have this utility for clearing the terminal
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        // Display the remaining time in a readable format
        if minutes > 1 {
            if seconds == 1 {
                println!("Time left: {} minutes 1 second", minutes);
            } else if seconds == 0 {
                println!("Time left: {} minutes", minutes);
            } else {
                println!("Time left: {} minutes {} seconds", minutes, seconds);
            }
        } else if minutes == 1 {
            if seconds == 1 {
                println!("Time left: 1 minute 1 second");
            } else if seconds == 0 {
                println!("Time left: 1 minute");
            } else {
                println!("Time left: 1 minute {} seconds", seconds);
            }
        } else {
            if seconds == 1 {
                println!("Time left: 1 second");
            } else if seconds == 0 {
                println!("Time left: 0 seconds");
            } else {
                println!("Time left: {} seconds", seconds);
            }
        }

        sleep(Duration::from_secs(1));
        total_seconds -= 1;
    }

    // Flashing "Time's up" message
    let mut flash = true;
    loop {
        format_utils::clear_terminal();
        if flash {
            println!("Time's up!");
        } else {
            println!(" ");
        }
        flash = !flash;
        sleep(Duration::from_millis(500));
    }
}

// Function to parse input string into a Duration
fn parse_duration(input: &str) -> Option<Duration> {
    // Try to parse in "MM:SS" format
    if let Some((minutes, seconds)) = input.split_once(':') {
        if let (Ok(min), Ok(sec)) = (minutes.parse::<u64>(), seconds.parse::<u64>()) {
            return Some(Duration::from_secs(min * 60 + sec));
        }
    }

    // If no ":" found, treat the input as just seconds
    if let Ok(seconds) = input.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    // Invalid input
    None
}
