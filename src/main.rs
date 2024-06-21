use std::env;
use dotenv::dotenv;
use ipgeolocate::{Locator, Service};
use tokio;

mod weather;
use weather::weather::get_weather;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");

    if let Some(ip) = public_ip::addr().await {
        println!("Found IP address: {:?}, trying to locate it...", ip);
        let service = Service::IpApi;
        match Locator::get(ip.to_string().as_str(), service).await {
            Ok(ip) => {
                println!("Found latitude/longitude for {}: {}, {}", ip.ip, ip.latitude, ip.longitude);
                println!("Trying to get weather for latitude: {}, longitude: {}", ip.latitude, ip.longitude);
                match get_weather(api_key, ip.latitude.parse().unwrap(), ip.longitude.parse().unwrap()).await {
                    Ok(weather) => {
                        println!("Current temperature: {}°C ({}°F)", weather.current.temp_c, weather.current.temp_f);
                    },
                    Err(e) => {
                        println!("Failed to get weather data: {}", e);
                    },
                }
            },
            Err(error) => println!("Error: {}", error),
        };
    } else {
        println!("Failed to get IP address");
    }
}