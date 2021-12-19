use crate::constants::{CITY_ERROR_MESSAGE, SELECT_CITY};
use crate::helpers::get_number_from_user;
use crate::show_weather::show_weather;
use crate::structs::WeatherItem;

pub fn ask_for_city(cities: Vec<String>, weather_data: Vec<WeatherItem>) -> Option<()> {
    // We ask to the user for the city
    println!("{}", SELECT_CITY);
    for (index, city) in cities.iter().enumerate() {
        println!("[{}] {}", index, city);
    }
    let city = get_number_from_user(0, cities.len() as i32 - 1, CITY_ERROR_MESSAGE);
    // If we got a city, we show the weather and we return Some(())
    // Else we return None
    if let Some(city_number) = city {
        show_weather(cities, city_number, weather_data);
        Some(())
    } else {
        None
    }
}
