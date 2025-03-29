use reqwest::Error;
use reqwest;
use serde::Deserialize;
// use


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
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Condition {
    // text: String,
    code: i32,
}

pub(crate) fn get_forecast(api_key: &str, city: &str) -> Result<ForecastResponse, Error> {

    let response = match reqwest::blocking::get(format!("http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=2&aqi=no&alerts=no", api_key, city)) {
        Ok(response) => response,
        Err(_) => panic!("Не удалось получить данные о погоде"),
    };
    let response = match response.json::<ForecastResponse>(){
        Ok(response) => response,
        Err(_) => get_error(),
    };
    // println!("{:?}", response);
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

pub(crate) fn get_weather_for_time(forecast: &ForecastResponse) -> Vec<(String, String, String)> {
    if forecast.forecast.forecastday.is_empty() {
        return vec![
        ("Err Err".to_string(), "0".to_string(), "Err".to_string()),
        ("Err Err".to_string(), "0".to_string(), "Err".to_string()),
        ("Err Err".to_string(), "0".to_string(), "Err".to_string())
        ];
    }
    let forecast_vec = forecast.forecast.forecastday[1].hour.clone();
    let time_stamp_lenth = forecast_vec.len();
    let mut test: Vec<(String, String, String)> = vec![];
    test.push((forecast_vec[0].time.clone(), forecast_vec[0].condition.code.to_string(), forecast_vec[0].temp_c.to_string()));
    test.push((forecast_vec[time_stamp_lenth/2].time.clone(), forecast_vec[time_stamp_lenth/2].condition.code.to_string(), forecast_vec[time_stamp_lenth/2].temp_c.to_string()));
    test.push((forecast_vec[time_stamp_lenth - 1].time.clone(), forecast_vec[time_stamp_lenth - 1].condition.code.to_string(), forecast_vec[time_stamp_lenth - 1].temp_c.to_string()));
    // println!("{:?}", test);
    // println!("{}", test.len());
    test
}
