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
    let mut provincias: Vec<String> = Vec::new();
    let mut ciudades: Vec<String> = Vec::new();
    let mut weather_data: Vec<WeatherItem> = Vec::new();
    println!("Buscamos el pronóstico a 1, 2 o 3 días?");
    let dias = get_number_from_user();
    if let Ok(dias_number) = dias {
        if (1..=3).contains(&dias_number) {
            println!("Buscando el pronóstico a {} día/s...", dias_number);
            weather_data = get_data(dias_number).await?;
            for weather_item in &weather_data {
                let found = provincias
                    .iter()
                    .find(|provincia| weather_item.province == **provincia);
                if found.is_none() {
                    provincias.push(weather_item.province.clone());
                }
            }
        } else {
            panic!("El valor ingresado no es válido");
        }
    } else {
        panic!("{}", dias.unwrap_err());
    }

    println!("Seleccione una provincia (ingrese el número):");
    for (index, provincia) in provincias.iter().enumerate() {
        println!("[{}] {}", index, provincia);
    }
    let provincia = get_number_from_user();
    if let Ok(provincia_number) = provincia {
        if provincia_number >= 0 && provincia_number < provincias.len() as i32 {
            let provincia_selected = provincias[provincia_number as usize].clone();
            println!("Seleccionada la provincia: {}", provincia_selected);
            for weather_item in &weather_data {
                if weather_item.province == provincia_selected {
                    let found = ciudades.iter().find(|ciudad| weather_item.name == **ciudad);
                    if found.is_none() {
                        ciudades.push(weather_item.name.clone());
                    }
                }
            }
            println!("Provincia seleccionada: {}...", provincia_selected);
        } else {
            panic!("El valor ingresado no es válido");
        }
        println!("provincia: {}", provincia_number);
    }

    println!("Seleccione una ciudad (ingrese el número):");
    for (index, ciudad) in ciudades.iter().enumerate() {
        println!("[{}] {}", index, ciudad);
    }
    let ciudad = get_number_from_user();
    if let Ok(ciudad_number) = ciudad {
        if ciudad_number >= 0 && ciudad_number < ciudades.len() as i32 {
            let ciudad_selected = ciudades[ciudad_number as usize].clone();
            println!("Ciudad seleccionada: {}", ciudad_selected);
            println!();
            println!("Mañana:");
            println!(
                "Temperatura: {}°C",
                &weather_data[ciudad_number as usize].weather.morning_temp
            );
            println!(
                "Pronóstico: {}",
                &weather_data[ciudad_number as usize].weather.morning_desc
            );
            println!();
            println!("Tarde:");
            println!(
                "Temperatura: {}°C",
                weather_data[ciudad_number as usize].weather.afternoon_temp
            );
            println!(
                "Pronóstico: {}",
                weather_data[ciudad_number as usize].weather.afternoon_desc
            );
        } else {
            panic!("El valor ingresado no es válido");
        }
    }
    Ok(())
}
