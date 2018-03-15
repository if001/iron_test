use chrono::prelude::*;

pub fn get_time() -> NaiveDateTime{
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let min = now.minute();
    let sec = now.second();

    let d = NaiveDate::from_ymd(year, month, day);
    let t = NaiveTime::from_hms_milli(hour, min, sec, 000);
    NaiveDateTime::new(d, t)
}