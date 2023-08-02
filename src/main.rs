use clap::{Parser};
use reqwest;
use dotenv;
use tabled::{Tabled, Table};
mod api_structs;
use crate::api_structs::CurrentWeather;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]

struct Args {
    /// Number of days for the forecast.
    #[arg(short, default_value_t = 1)]
    days: u8,
    #[arg(short, default_value_t = String::from("Amsterdam"))]
    location: String,
}

#[derive(Tabled)]
struct Result {
    date: String,
    mintemp_c: f32,
    maxtemp_c: f32,
    condition: String,
    sunrise: String,
    sunset: String,
}

fn main() {
    dotenv::dotenv().ok();
 
    let mut api_key = None;
    for (key, value) in dotenv::vars() {
        if key != "API_KEY" {
            continue;
        }
        api_key = Some(value)
    }
    if api_key.is_none() {
        panic!("need API key")
    }

    let api_key = api_key.unwrap();

    let args: Args = Args::parse();

    let url = format!("http://api.weatherapi.com/v1/forecast.json?key={api_key}&q={}&days={}",args.location, args.days);

    let body: CurrentWeather = reqwest::blocking::get(url).expect("Failed to fetch data").json().expect("Failed to desiarialize");

    println!("{} right now in {} with the temperature at about {} degrees",body.current.condition.text, body.location.name, body.current.temp_c);

    let mut result = vec![];

    for e in body.forecast.forecastday{
        result.push(Result{
            date: e.date,
            mintemp_c: e.day.mintemp_c,
            maxtemp_c: e.day.maxtemp_c,
            condition: e.day.condition.text,
            sunrise: e.astro.sunrise,
            sunset: e.astro.sunset, 
        });
    }

    let table = Table::new(result);
    println!("{}", table);
}
