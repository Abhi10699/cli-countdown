use std::env::args;

use chrono::Timelike;
use datetime::{self, LocalDateTime, LocalTime};

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
            date: date,
            month: month,
            year: year,

            days_left: diff_days,
            hours_left: hour_diff,
            minutes_left: minute_diff,
        }
    }

    fn print(&self) {
        println!(
            "{days_left} Days, {hours_left} Hours and {mins_left} Minutes Left for {event_name}.",
            days_left = self.days_left,
            hours_left = self.hours_left,
            mins_left = self.minutes_left,
            event_name = self.event_name
        )
    }
}

fn main() {
    let ctd = EventCountdown::new("Christmas", 25, 12, 2024);
    ctd.print();

    println!("{:?}", args());
}