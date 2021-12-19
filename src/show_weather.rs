use crate::constants::{AFTERNON, CITY_SELECTED, FORECAST, MORNING, TEMPERATURE};

use crate::structs::WeatherItem;

pub fn show_weather(cities: Vec<String>, city_number: i32, weather_data: Vec<WeatherItem>) {
    let city_selected = cities[city_number as usize].clone();
    println!();
    println!("{}{}", CITY_SELECTED, city_selected);
    println!();
    println!("{}", MORNING);
    println!(
        "{}{}",
        TEMPERATURE, &weather_data[city_number as usize].weather.morning_temp
    );
    println!(
        "{}{}",
        FORECAST, &weather_data[city_number as usize].weather.morning_desc
    );
    println!();
    println!("{}", AFTERNON);
    println!(
        "{}{}",
        TEMPERATURE, weather_data[city_number as usize].weather.afternoon_temp
    );
    println!(
        "{}{}",
        FORECAST, weather_data[city_number as usize].weather.afternoon_desc
    );
}
