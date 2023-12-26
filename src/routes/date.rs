use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

// import services module -> which we exported in mod.rs
use crate::services;

// struct to hold Date data
// serialize/deserialize to convert to/from JSON
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

// route returns a Date object converted to JSON
#[get("/date/now")]
pub fn get_current_date() -> Json<Date> {
    Json(services::date::get_current_date())
}

#[post("/date/add-month", format = "json", data = "<date>")]
pub fn date_plus_month(date: Json<Date>) -> Json<Date> {
    Json(services::date::date_plus_month(date))
}
