use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i32,
    pub cloud: i32,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub windchill_c: f64,
    pub windchill_f: f64,
    pub heatindex_c: f64,
    pub heatindex_f: f64,
    pub dewpoint_c: f64,
    pub dewpoint_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}



pub async fn get_weather(api_key: String, latitude: f64, longitude: f64) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={},{}", api_key, latitude, longitude);
    let response = reqwest::get(&url).await?;
    let blabla = response.text().await?;
    let weather: WeatherResponse = serde_json::from_str(&blabla)?;
    Ok(weather)
}