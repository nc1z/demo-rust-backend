use chrono::Datelike;
use rocket::serde::json::Json;

use crate::routes::date::Date;

pub fn get_current_date() -> Date {
    let current_utc = chrono::Utc::now();
    let year = current_utc.year();
    let month = current_utc.month();
    let day = current_utc.day();
    let current_date = Date { day, month, year };
    current_date
}

pub fn date_plus_month(mut date: Json<Date>) -> Date {
    let mut new_month = date.month + 1;
    if new_month > 12 {
        new_month = 1;
        date.year = date.year + 1;
    }
    let new_date = Date {
        day: date.day,
        month: new_month,
        year: date.year,
    };
    new_date
}
