use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

#[derive(Serialize, Deserialize, Debug)]
struct WeatherDetails {
    day: u8,
    morning_temp: i32,
    morning_id: u32,
    morning_desc: String,
    afternoon_temp: i32,
    afternoon_id: u32,
    afternoon_desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

fn get_number_from_user() -> Result<i32, &'static str> {
    print!("> ");
    let _ = stdout().flush();
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Error al leer la entrada del usuario");
    if let Ok(number_from_input) = user_input.trim().parse::<i32>() {
        Ok(number_from_input)
    } else {
        Err("El Valor ingresado no es un número")
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

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    println!("Buscamos el pronóstico a 1, 2 o 3 días?");
    let dias = get_number_from_user();
    if let Ok(dias_number) = dias {
        println!("{:#?}"
            , get_data(dias_number).await.unwrap());
    }
    let provincia = get_number_from_user();
    if let Ok(provincia_number) = provincia {
        println!("provincia: {}", provincia_number);
    }
    let ciudad = get_number_from_user();
    if let Ok(ciudad_number) = ciudad {
        println!("ciudad: {}", ciudad_number);
    }
    Ok(())
}
