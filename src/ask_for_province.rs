use crate::constants::{PROVINCE_ERROR_MESSAGE, PROVINCE_SELECTED, SELECT_PROVINCE};
use crate::helpers::get_number_from_user;
use crate::structs::WeatherItem;

pub fn ask_for_province(
    provinces: Vec<String>,
    weather_data: &[WeatherItem],
) -> Option<Vec<String>> {
    println!("{}", SELECT_PROVINCE);
    for (index, province) in provinces.iter().enumerate() {
        println!("[{}] {}", index, province);
    }
    let province = get_number_from_user(0, provinces.len() as i32, PROVINCE_ERROR_MESSAGE);
    if let Some(province_number) = province {
        let province_selected = provinces[province_number as usize].clone();
        println!();
        println!("{}{}...", PROVINCE_SELECTED, province_selected);
        println!();
        Some(weather_data.iter().fold(Vec::new(), |mut acc, item| {
            if item.province == province_selected && !acc.contains(&item.name) {
                acc.push(item.name.clone());
            }
            acc
        }))
    } else {
        None
    }
}
