use reqwest::Error;
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
}

#[derive(Debug, Deserialize, Clone)]
struct Condition {
    text: String,
    code: i32,
}

pub(crate) async fn get_weather(api_key: &str, city: &str) -> Result<WeatherResponse, Error> {
    let response = match reqwest::get(format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city))
    .await {
        Ok(response) => response,
        Err(_) => panic!("Не удалось получить данные о погоде"),
    };
    let header = &response.headers().clone();
    let mut response = match response.json::<WeatherResponse>().await{
        Ok(response) => response,
        Err(_) => get_error(),
    };
    println!("{:?}", header);
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
        },
    }
}
