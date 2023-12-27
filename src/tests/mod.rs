use super::rocket;
use chrono::{Datelike, Local};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::{json, Value};

fn common_setup() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

#[test]
fn get_current_date() {
    let client = common_setup();
    let response = client.get("/api/date/now").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let today = Local::now().date_naive();

    let response_json: Value =
        serde_json::from_str(&response.into_string().unwrap()).expect("valid JSON");

    let expected_json = json!({
        "day": today.day(),
        "month": today.month(),
        "year": today.year(),
    });

    assert_eq!(response_json, expected_json);
}

#[test]
fn date_plus_month() {
    let client = common_setup();

    let request_body = json!({
        "day": 27,
        "month": 10,
        "year": 2023
    });

    let response = client
        .post("/api/date/add-month")
        .header(ContentType::JSON)
        .body(request_body.to_string())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_json: Value =
        serde_json::from_str(&response.into_string().unwrap()).expect("valid JSON");

    let expected_json = json!({
        "day": 27,
        "month": 11,
        "year": 2023
    });

    assert_eq!(response_json, expected_json);
}
