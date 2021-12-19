use crate::constants::{PROVINCIA_ERROR_MESSAGE, PROVINCIA_SELECTED, SELECT_PROVINCIA};
use crate::helpers::get_number_from_user;
use crate::structs::WeatherItem;

pub fn ask_for_provincia(
    provincias: Vec<String>,
    weather_data: &[WeatherItem],
) -> Option<Vec<String>> {
    println!("{}", SELECT_PROVINCIA);
    for (index, provincia) in provincias.iter().enumerate() {
        println!("[{}] {}", index, provincia);
    }
    let provincia = get_number_from_user(0, provincias.len() as i32, PROVINCIA_ERROR_MESSAGE);
    if let Some(provincia_number) = provincia {
        let provincia_selected = provincias[provincia_number as usize].clone();
        println!("{}{}...", PROVINCIA_SELECTED, provincia_selected);
        Some(weather_data.iter().fold(Vec::new(), |mut acc, item| {
            if item.province == provincia_selected && !acc.contains(&item.name) {
                acc.push(item.name.clone());
            }
            acc
        }))
    } else {
        None
    }
}
