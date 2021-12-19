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
use get_data::get_data;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // We get the weather forecast by asking to the user for the number of days
    // and calling the get_data function with the number of days.
    let weather_data = get_data(ask_for_day().unwrap()).await?;

    // We create a vector of strings with the names of the provinces.
    let provinces: Vec<String> = weather_data.iter().fold(Vec::new(), |mut acc, item| {
        if !acc.contains(&item.province) {
            acc.push(item.province.clone());
        }
        acc
    });

    // We ask the user to select a province and we get a Vec with the cities of that province.
    let cities = ask_for_province(provinces, &weather_data).unwrap();

    // We ask the user to select a city and we display the weather of that city.
    ask_for_city(cities, weather_data);

    Ok(())
}
