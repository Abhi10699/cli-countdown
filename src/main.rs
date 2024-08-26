use chrono::Timelike;
use datetime::{self, Instant, LocalDateTime, LocalTime};
fn main() {
    let current_time = chrono::Local::now(); // seconds

    let target_date = datetime::LocalDate::ymd(2024, datetime::Month::December, 25).unwrap();
    let target_instant = LocalDateTime::new(target_date, LocalTime::hm(0, 0).unwrap())
        .to_instant()
        .seconds();

    // countdown computation happens here

    let time_diff = target_instant - current_time.timestamp();
    let diff_days = time_diff / (3600 * 24);
    let hour_diff = 24 - current_time.hour() - 1;
    let minute_diff = 60 - current_time.minute();

    println!("Diff Days: {:?}", diff_days);
    println!("Hour Days: {:?}", hour_diff);
    println!("Minute Days: {:?}", minute_diff);

    println!("Chrono Now: {:?}", current_time);
}
