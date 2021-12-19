use exitfailure::ExitFailure;

mod ask_for_city;
mod ask_for_day;
mod ask_for_province;
mod constants;
mod get_data;
mod helpers;
mod show_weather;
mod structs;

use ask_for_city::ask_for_city;
use ask_for_day::ask_for_day;
use ask_for_province::ask_for_province;
use constants::{API_ERROR_MESSAGE, EXIT_MESSAGE};
use get_data::get_data;
use structs::WeatherItem;

fn print_exit_message() {
    println!("{}", EXIT_MESSAGE)
}

fn get_provinces(weather_data: &[WeatherItem]) -> Vec<String> {
    weather_data.iter().fold(Vec::new(), |mut acc, item| {
        if !acc.contains(&item.province) {
            acc.push(item.province.clone());
        }
        acc
    })
}

fn run_province(weather_data: Vec<WeatherItem>) -> Option<()> {
    let provinces = get_provinces(&weather_data);
    let cities = ask_for_province(provinces, &weather_data);
    match cities {
        Some(cities) => ask_for_city(cities, weather_data),
        _ => None,
    }
}

async fn run() -> Option<()> {
    // We ask to the user for the day
    // If we got a day, we run the rest of the program
    if let Some(day) = ask_for_day() {
        match get_data(day).await {
            Ok(weather_data) => run_province(weather_data),
            _ => {
                println!("{}", API_ERROR_MESSAGE);
                None
            }
        }
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    if run().await.is_none() {
        print_exit_message()
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use structs::WeatherDetails;
    #[test]
    fn test_handle_orders() {
        let weather_data = vec![
                WeatherItem {
                    _id: "609d947e818e15902c74d831".to_string(),
                    dist: 20.55,
                    lid: 9539,
                    fid: 9539,
                    name: "Tunuyan".to_string(),
                    province: "Mendoza".to_string(),
                    lat: "-33.58210373".to_string(),
                    lon: "-69.02268982".to_string(),
                    zoom: "2".to_string(),
                    updated: 1557349200,
                    weather:
                        WeatherDetails {
                        day: 1,
                        morning_temp: 10,
                        morning_id: 2,
                        morning_desc: "Cielo parcialmente nublado. Vientos leves del sector sur.".to_string(),
                        afternoon_temp: 22,
                        afternoon_id: 14,
                        afternoon_desc: "Nubosidad variable. Probabilidad de tormentas aisladas. Vientos leves del sector sur.".to_string(),
                }
            }
        ];
        let provinces = get_provinces(&weather_data);
        assert_eq!(provinces.len(), 1);
        assert_eq!(provinces[0], "Mendoza");
    }
}
