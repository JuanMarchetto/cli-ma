use crate::constants::BASE_URL;
use crate::structs::WeatherItem;
use reqwest::Url;

pub async fn get_data(dia: i32) -> Result<Vec<WeatherItem>, reqwest::Error> {
    let url = format!("{}{}", BASE_URL, dia);
    let parsed_url = Url::parse(&*url).unwrap();
    reqwest::get(parsed_url)
        .await
        .unwrap()
        .json::<Vec<WeatherItem>>()
        .await
}
