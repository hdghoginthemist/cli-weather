use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Day {
    pub maxtemp_c: f32,
    pub mintemp_c: f32,
    pub daily_chance_of_rain: f32,
    pub daily_chance_of_snow: f32,
    pub condition: Condition,
}

#[derive(Deserialize, Debug)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Deserialize, Debug)]
pub struct Forecastday {
   pub date: String,
   pub day: Day,
   pub astro: Astro,
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
   pub forecastday: Vec<Forecastday>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
   pub lat: f32, 
   pub lon: f32,
   pub name: String,
   pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub text: String
}


#[derive(Deserialize, Debug)]
pub struct Current {
   pub temp_c: f32,
   pub condition: Condition,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
   pub location: Location,
   pub current: Current,
   pub forecast: Forecast,
}