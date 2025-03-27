// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod weather;
mod config;

use std::error::Error;

slint::include_modules!();

static API_KEY: &str = "e756ac9e718447a0ae9133510252303";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let weather_responce = weather::get_weather(API_KEY, "Minsk").await?;
    weather::print_weather(&weather_responce);

    let ui = MainWindow::new().unwrap();

    ui.window().set_position( slint::PhysicalPosition::new(0, 0));
    let ui_clone = ui.as_weak();
    ui.on_mouse_move(
            move |delta_x, delta_y| {
            let ui_clone = ui_clone.unwrap();
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
            let ui_clone = ui_clone.unwrap();
            ui_clone.window().set_minimized(true);
        }
    );
    let ui_clone = ui.as_weak();
    let weather_responce_clone = weather_responce.clone();
    ui.on_reload_window(
        move || {
            set_all_params(&weather_responce_clone, &ui_clone.unwrap());
        }
    );
    set_all_params(&weather_responce, &ui);

    ui.run()?;
    Ok(())
}

fn split_date_time(date_time: &String) -> (String, String) {
    let vec: Vec<&str> = date_time.split(' ').collect();
    (format!("{} {} {}", vec[0], vec[1], vec[2]), format!("{} {} {}", vec[3], vec[4], vec[5]))
}

fn set_all_params(weather_responce: &weather::WeatherResponse, ui: &MainWindow) {
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
