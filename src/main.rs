use exitfailure::ExitFailure;

mod ask_for_city;
mod ask_for_day;
mod constants;
mod get_city;
mod get_data;
mod helpers;
mod show_weather;
mod structs;

use ask_for_city::ask_for_city;
use ask_for_day::ask_for_day;
use get_city::get_city;
use get_data::get_data;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let weather_data = get_data(ask_for_day().unwrap()).await?;

    let provincias: Vec<String> = weather_data.iter().fold(Vec::new(), |mut acc, item| {
        if !acc.contains(&item.province) {
            acc.push(item.province.clone());
        }
        acc
    });
    let cities = get_city(provincias, &weather_data).unwrap();

    ask_for_city(cities, weather_data);

    Ok(())
}
