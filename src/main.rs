use clap::Parser;
use std::{
    fs::{File, OpenOptions, Permissions},
    io::Write,
    path::Path,
};

use chrono::Timelike;
use datetime::{self, LocalDateTime, LocalTime};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the event
    #[arg(short, long)]
    event: String,

    /// Date of the Event
    #[arg(short, long)]
    date: i8,

    /// Month of the Event
    #[arg(short, long)]
    month: i8,

    /// Year of the event
    #[arg(short, long)]
    year: i64,
}

struct EventCountdown {
    event_name: String,
    date: i8,
    month: i8,
    year: i64,

    days_left: i64,
    hours_left: u32,
    minutes_left: u32,
}

impl EventCountdown {
    fn new(event_name: &str, date: i8, month: i8, year: i64) -> EventCountdown {
        let current_time = chrono::Local::now(); // seconds

        let datetime_month =
            datetime::Month::from_one(month).expect("Invalid month, should be between 1-12");
        let target_date: datetime::LocalDate =
            datetime::LocalDate::ymd(year, datetime_month, date).unwrap();
        let target_instant = LocalDateTime::new(target_date, LocalTime::hm(0, 0).unwrap())
            .to_instant()
            .seconds();

        // countdown computation happens here

        let time_diff = target_instant - current_time.timestamp();
        if time_diff < 0 {
            panic!("You're already past the date");
        }

        let diff_days = time_diff / (3600 * 24);
        let hour_diff = 24 - current_time.hour() - 1;
        let minute_diff = 60 - current_time.minute();

        EventCountdown {
            event_name: event_name.to_string(),
            date,
            month,
            year,

            days_left: diff_days,
            hours_left: hour_diff,
            minutes_left: minute_diff,
        }
    }

    fn print(&self) {
        println!(
            "{days_left} Days {hours_left} Hours and {mins_left} Minutes Left for {event_name}.",
            days_left = self.days_left,
            hours_left = self.hours_left,
            mins_left = self.minutes_left,
            event_name = self.event_name
        )
    }

    fn write_event(&self) {
        // check file exist

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("events.txt")
            .unwrap();

        let mut file_content = Vec::new();
        match writeln!(
            &mut file_content,
            "{event} {date} {month} {year}",
            event = self.event_name,
            date = self.date,
            month = self.month,
            year = self.year
        ) {
            Ok(_) => {
                file.write(&file_content).unwrap();
            }
            Err(err) => {
                panic!("Error formatting the date string");
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let ctd = EventCountdown::new(&args.event, args.date, args.month, args.year);
    ctd.print();
    ctd.write_event();
}
