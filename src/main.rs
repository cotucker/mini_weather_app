// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod weather;
mod config;
mod forecast;
mod geolocation;

use std::error::Error;
use slint::VecModel;
use std::rc::Rc;

slint::include_modules!();


static API_KEY: &str = "e756ac9e718447a0ae9133510252303";


fn main() -> Result<(), Box<dyn Error>> {

    let ui = MainWindow::new().unwrap();

    ui.window().set_position( slint::PhysicalPosition::new(0, 0));
    let ui_clone = ui.as_weak();
    ui.on_mouse_move(
            move |delta_x, delta_y| {
            let ui_clone = ui_clone.upgrade().unwrap();
            let logical_pos = ui_clone.window().position().to_logical(ui_clone.window().scale_factor());
            ui_clone.window().set_position(slint::LogicalPosition::new(logical_pos.x + delta_x, logical_pos.y + delta_y));
        }
    );

    ui.on_close_window(
        move || {
            std::process::exit(0);
        }
    );
    let ui_clone = ui.as_weak();
    ui.on_window_minimize(
        move || {
            let ui_clone = ui_clone.upgrade().unwrap();
            ui_clone.window().set_minimized(true);
        }
    );
    let ui_clone = ui.as_weak();
    ui.on_reload_window (
        move || {
            let ui_clone = ui_clone.upgrade().unwrap();
            update_params(&ui_clone);
        }
    );
    let ui_clone = ui.as_weak();
    ui.on_city_submitted(
        move |city| {
            let ui_clone = ui_clone.upgrade().unwrap();
            println!("In submite callback: {}", city);
            set_new_city(&ui_clone, &city.as_str());
        }
    );
    update_params(&ui);
    ui.run()?;
    Ok(())
}

fn set_new_city(ui: &MainWindow, city: &str) {
    let weather_responce = weather::get_weather(API_KEY, city.trim()).unwrap();
    let forecast_responce = forecast::get_forecast(API_KEY, city.trim()).unwrap();
    set_all_params(&weather_responce, &forecast_responce, ui);
}

fn split_date_time(date_time: &String) -> (String, String) {
    let vec: Vec<&str> = date_time.split(' ').collect();
    (format!("{} {} {}", vec[0], vec[1], vec[2]), format!("{} {} {}", vec[3], vec[4], vec[5]))
}

fn get_time_from_date_time(date_time: &String) -> String {
    let vec: Vec<&str> = date_time.split(' ').collect();
    format!("{}", vec[1])
}

fn update_params(ui: &MainWindow) {
    // let mut city = String::new();
    // std::io::stdin()
    //    .read_line(&mut city)
    //    .expect("Failed to read line");
    let city = geolocation::get_location().unwrap();
    let weather_responce = weather::get_weather(API_KEY, city.as_str()).unwrap();
    let forecast_responce = forecast::get_forecast(API_KEY, city.as_str()).unwrap();
    set_all_params(&weather_responce, &forecast_responce, ui);
}

fn set_all_params(weather_responce: &weather::WeatherResponse, forecast_response: &forecast::ForecastResponse, ui: &MainWindow) {

    ui.set_country(weather::get_location(&weather_responce).0.into());
    ui.set_city(weather::get_location(&weather_responce).1.into());
    ui.set_condition(weather::get_condition(&weather_responce).into());
    ui.set_temperature(weather::get_temp(&weather_responce).into());
    ui.set_date(split_date_time(&weather::get_date(&weather_responce)).0.into());
    ui.set_time(split_date_time(&weather::get_date(&weather_responce)).1.into());
    let color_pallet = config::color_pallet_map();
    let text_color = color_pallet.get(weather::get_code(&weather_responce).as_str()).unwrap().0;
    let first_grad_color = color_pallet.get(weather::get_code(&weather_responce).as_str()).unwrap().1;
    let secont_grad_color = color_pallet.get(weather::get_code(&weather_responce).as_str()).unwrap().2;
    ui.set_text_color(get_slint_color(text_color));
    ui.set_first_gradient_color(get_slint_color(first_grad_color));
    ui.set_second_gradient_color(get_slint_color(secont_grad_color));
    ui.set_image(slint::Image::load_from_path(std::path::Path::new(config::icon_map(weather::get_code(&weather_responce).as_str()))).unwrap());
    ui.set_humidity(weather::get_humidity(&weather_responce).into());
    ui.set_feels_like_c(weather::get_feels_like(&weather_responce).into());
    ui.set_wind_kph(weather::get_wind(&weather_responce).into());


    let weather_for_time = forecast::get_weather_for_time(forecast_response);
// TODO: add more farecast items
    let slint_data_items: Vec<WeatherForTime> = weather_for_time
        .into_iter()
        .map(|(s1, s2, s3)| WeatherForTime {
           temperature: s3.into(),
           time: get_time_from_date_time(&s1).into(),
           time_condition_icon: slint::Image::load_from_path(std::path::Path::new(config::icon_map(s2.as_str()))).unwrap().into(),
        })
        .collect();
    let new_model = Rc::new(VecModel::from(slint_data_items));
    ui.set_my_vector_data(new_model.into());

}

fn get_color(rgb_color: &'static str) -> (u8, u8, u8) {
    let mut color = (0,0,0);
    let vec: Vec<&str> = rgb_color.split(", ").collect();
    color.0 = vec[0].parse().unwrap();
    color.1 = vec[1].parse().unwrap();
    color.2 = vec[2].parse().unwrap();
    color
}

fn get_slint_color(rgb_color: &'static str) -> slint::Color {
    let color = get_color(rgb_color);
    slint::Color::from_rgb_u8(color.0, color.1, color.2)
}
