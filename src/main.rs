// import Rocket
#[macro_use]
extern crate rocket;

mod routes;
mod services;

use routes::date::date_plus_month;
use routes::date::get_current_date;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_current_date, date_plus_month])
}

#[cfg(test)]
mod tests;
