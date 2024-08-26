use datetime::{self, Instant, LocalDateTime, LocalTime, TimePiece};
fn main() {
    let current_instant = Instant::now();
    let current_time = LocalDateTime::from_instant(current_instant);

    let target_date = datetime::LocalDate::ymd(2024, datetime::Month::December, 25).unwrap();
    let target_instant = LocalDateTime::new(target_date, LocalTime::hm(0,0).unwrap())
        .to_instant()
        .seconds();

    // countdown computation happens here

    let time_diff = target_instant as f64 - (current_instant.seconds()) as f64;
    let diff_days = (time_diff / (3600.0 * 24.0)).ceil() as i32;
    let hour_diff: i8 = 24 - current_time.hour() - 1;
    let minute_diff = 60 - current_time.minute();

    println!("Diff Days: {:?}", diff_days);
    println!("Hour Days: {:?}", hour_diff);
    println!("Minute Days: {:?}", minute_diff);
    
}
