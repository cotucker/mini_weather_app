use reqwest::Error;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Location {
    status: String,
    city: String,
}

pub fn get_location() -> Result<String, Error> {
    let response = match reqwest::blocking::get(format!("http://ip-api.com/json/")) {
        Ok(response) => response,
        Err(_) => panic!("Не удалось получить геопозицию"),
    };
    let response = match response.json::<Location>(){
        Ok(response) => response,
        Err(_) => get_error(),
    };
    if response.status == "fail" {
        return Ok("error".to_string());
    } else {
        return Ok(response.city);
    }
}

fn get_error() -> Location {
    Location {
        status: "error".to_string(),
        city: "error".to_string(),
    }
}
