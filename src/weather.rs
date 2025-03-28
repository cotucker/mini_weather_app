use reqwest::Error;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Debug, Deserialize, Clone)]
struct Location {
    name: String,
    country: String,
    localtime: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Current {
    temp_c: f64,
    condition: Condition,
    humidity: f64,
    feelslike_c: f64,
    wind_kph: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Condition {
    text: String,
    code: i32,
}

pub fn get_weather(api_key: &str, city: &str) -> Result<WeatherResponse, Error> {

    let response = match reqwest::blocking::get(format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city)) {
        Ok(response) => response,
        Err(_) => panic!("Не удалось получить данные о погоде"),
    };
    let header = &response.headers().clone();
    let mut response = match response.json::<WeatherResponse>(){
        Ok(response) => response,
        Err(_) => get_error(),
    };
    response.location.localtime = String::from(header.get("date").unwrap().to_str().unwrap());
    Ok(response)
}
pub(crate) fn get_location(weather: &WeatherResponse) -> (String, String)  {
    (weather.location.name.clone(), weather.location.country.clone())

}
pub(crate) fn get_temp(weather: &WeatherResponse) -> String {
    format!("{}", weather.current.temp_c)
}
pub(crate) fn get_condition(weather: &WeatherResponse) -> String {
    format!("{}", weather.current.condition.text)
}
pub(crate) fn get_date(weather: &WeatherResponse) -> String {
    format!("{}", weather.location.localtime)
}
pub(crate) fn get_code(weather: &WeatherResponse) -> String {
    format!("{}", weather.current.condition.code)
}
pub(crate) fn get_humidity(weather: &WeatherResponse) -> String {
    format!("{}", weather.current.humidity)
}
pub(crate) fn get_feels_like(weather: &WeatherResponse) -> String {
    format!("{}", weather.current.feelslike_c)
}
pub(crate) fn get_wind(weather: &WeatherResponse) -> String {
    format!("{}", weather.current.wind_kph)
}


pub(crate) fn print_weather(weather: &WeatherResponse) {
    println!("Город: {}, {}", weather.location.name, weather.location.country);
    println!("Температура: {}°C", weather.current.temp_c);
    println!("Состояние: {}", weather.current.condition.text);
    println!("Время: {}", weather.location.localtime);
    println!("Code: {}", weather.current.condition.code)
}

fn get_error() -> WeatherResponse {
    WeatherResponse {
        location: Location {
            name: "Error".to_string(),
            country: "Error".to_string(),
            localtime: "Error".to_string(),
        },
        current: Current {
           condition: Condition {
            text: "Error".to_string(),
            code: 0,
            },
           temp_c: 0.0,
           humidity: 0.0,
           feelslike_c: 0.0,
           wind_kph: 0.0,
        },
    }
}
