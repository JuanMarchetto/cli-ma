use crate::constants::{CITY_ERROR_MESSAGE, SELECT_CITY};
use crate::helpers::get_number_from_user;
use crate::show_weather::show_weather;
use crate::structs::WeatherItem;

pub fn ask_for_city(cities: Vec<String>, weather_data: Vec<WeatherItem>) {
    println!("{}", SELECT_CITY);
    for (index, city) in cities.iter().enumerate() {
        println!("[{}] {}", index, city);
    }
    let city = get_number_from_user(0, cities.len() as i32, CITY_ERROR_MESSAGE);
    if let Some(city_number) = city {
        show_weather(cities, city_number, weather_data);
    }
}
