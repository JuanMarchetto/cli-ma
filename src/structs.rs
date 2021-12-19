use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WeatherDetails {
    pub day: u8,
    pub morning_temp: i32,
    pub morning_id: u32,
    pub morning_desc: String,
    pub afternoon_temp: i32,
    pub afternoon_id: u32,
    pub afternoon_desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherItem {
    pub _id: String,
    pub dist: f64,
    pub lid: u32,
    pub fid: u32,
    pub name: String,
    pub province: String,
    pub lat: String,
    pub lon: String,
    pub zoom: String,
    pub updated: u32,
    pub weather: WeatherDetails,
}
