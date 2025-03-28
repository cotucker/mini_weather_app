use reqwest::Error;
use reqwest;
use serde::Deserialize;

use crate::weather::Condition;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ForecastResponse {
    forecast: Forecast,
}

#[derive(Debug, Deserialize, Clone)]
struct Forecast {
    forecastday: Vec<Forecastday>,
}

#[derive(Debug, Deserialize, Clone)]
struct Forecastday {
    hour: Vec<HourForecast>,
}

#[derive(Debug, Deserialize, Clone)]
struct HourForecast {
    time: String,
    temp_c: f32,
    condition: Condition,
    humidity: f32,
}

pub(crate) fn get_forecast(api_key: &str, city: &str) -> Result<ForecastResponse, Error> {

    println!("sdasdas");
    let response = match reqwest::blocking::get(format!("http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=1&aqi=no&alerts=no", api_key, city)) {
        Ok(response) => response,
        Err(_) => panic!("Не удалось получить данные о погоде"),
    };
    let response = match response.json::<ForecastResponse>(){
        Ok(response) => response,
        Err(_) => get_error(),
    };
    println!("{:?}", response);
    Ok(response)
}

fn get_error() -> ForecastResponse {
    println!("ForecastResponse error");
    ForecastResponse {
        forecast: Forecast {
            forecastday: vec![],
        }
    }
}
