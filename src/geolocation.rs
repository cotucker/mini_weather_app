use reqwest::Error;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Location {
    status: String,
    city: String,
}

#[derive(Deserialize, Debug)]
struct PossibleLocation {
    name: String,
    country: String,
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

pub fn autocomplition (api_key: &str, city: &str) -> Result<Vec<String>, Error> {
    let response = match reqwest::blocking::get(format!("http://api.weatherapi.com/v1/search.json?key={}&q={}", api_key, city)) {
        Ok(response) => response,
        Err(_) => panic!("Не удалось получить геопозицию"),
    };
    let vec: Vec<PossibleLocation> = vec![
        PossibleLocation {
            name: "error".to_string(),
            country: "error".to_string(),
        }
    ];
    let response = match response.json::<Vec<PossibleLocation>>(){
        Ok(response) => response,
        Err(_) => vec,
    };
    let vec = response.iter().map(|s1| format!("{}, {}", s1.name, s1.country)).collect();
    Ok(vec)
}


fn get_error() -> Location {
    Location {
        status: "error".to_string(),
        city: "error".to_string(),
    }
}
