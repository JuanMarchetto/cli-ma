use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

#[derive(Serialize, Deserialize)]
struct WeatherDetails {
    day: u8,
    morning_temp: i32,
    morning_id: u32,
    morning_desc: String,
    afternoon_temp: i32,
    afternoon_id: u32,
    afternoon_desc: String,
}

#[derive(Serialize, Deserialize)]
struct WeatherItem {
    _id: String,
    dist: f64,
    lid: u32,
    fid: u32,
    name: String,
    province: String,
    lat: String,
    lon: String,
    zoom: String,
    updated: u32,
    weather: WeatherDetails,
}

fn get_number_from_user(min: i32, max: i32, error_message: &str) -> Option<i32> {
    loop {
        let mut user_input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut user_input).expect(error_message);
        if user_input.trim() == "q" {
            break None;
        }
        if let Ok(number_from_input) = user_input.trim().parse::<i32>() {
            if validate(number_from_input, min, max) {
                break Some(number_from_input);
            }
        }
        println!("{}", error_message);
    }
}

async fn get_data(dia: i32) -> Result<Vec<WeatherItem>, reqwest::Error> {
    let url = format!("https://ws.smn.gob.ar/map_items/forecast/{}", dia);
    let parsed_url = Url::parse(&*url).unwrap();
    reqwest::get(parsed_url)
        .await
        .unwrap()
        .json::<Vec<WeatherItem>>()
        .await
}

fn ask_for_day() -> Option<i32> {
    println!("Buscamos el pronóstico a 1, 2 o 3 días?");
    let dias = get_number_from_user(
        1,
        3,
        "Por favor ingrese un número entre 1 y 3. Para salir presione q",
    );
    if let Some(dias_number) = dias {
        println!("Buscando el pronóstico a {} día/s...", dias_number);
        Some(dias_number)
    } else {
        None
    }
}

fn validate(dia: i32, min: i32, max: i32) -> bool {
    (min..=max).contains(&dia)
}

fn get_city(provincias: Vec<String>, weather_data: &Vec<WeatherItem>) -> Option<Vec<String>> {
    println!("Seleccione una provincia (ingrese el número):");
    for (index, provincia) in provincias.iter().enumerate() {
        println!("[{}] {}", index, provincia);
    }
    let provincia = get_number_from_user(
        0,
        provincias.len() as i32,
        "Por favor ingrese un número de provincia valido. Para salir presione q",
    );
    if let Some(provincia_number) = provincia {
        let provincia_selected = provincias[provincia_number as usize].clone();
        println!("Seleccionada la provincia: {}", provincia_selected);

        println!("Provincia seleccionada: {}...", provincia_selected);
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

fn show_weather(cities: Vec<String>, city_number: i32, weather_data: Vec<WeatherItem>) {
    let city_selected = cities[city_number as usize].clone();
    println!("Ciudad seleccionada: {}", city_selected);
    println!();
    println!("Mañana:");
    println!(
        "Temperatura: {}°C",
        &weather_data[city_number as usize].weather.morning_temp
    );
    println!(
        "Pronóstico: {}",
        &weather_data[city_number as usize].weather.morning_desc
    );
    println!();
    println!("Tarde:");
    println!(
        "Temperatura: {}°C",
        weather_data[city_number as usize].weather.afternoon_temp
    );
    println!(
        "Pronóstico: {}",
        weather_data[city_number as usize].weather.afternoon_desc
    );
}


fn ask_for_city(cities: Vec<String>, weather_data: Vec<WeatherItem>) {
    println!("Seleccione una ciudad (ingrese el número):");
    for (index, city) in cities.iter().enumerate() {
        println!("[{}] {}", index, city);
    }
    let city = get_number_from_user(
        0,
        cities.len() as i32,
        "Por favor ingrese un número de provincia valido. Para salir presione q",
    );
    if let Some(city_number) = city {
        show_weather(cities, city_number, weather_data);
    }
}

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
