use colored::*;
use reqwest::Response;
use serde::Deserialize;
use std::{fmt::format, io};

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}
// fcdb4c5cccb0b13a45b74cd9d2f6c1f1
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;

    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C
        > Humidity: {:.1}%
        > Pressure: {:.1} hPa
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
    );

    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcase clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstrom" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);

    fn get_emoji(temp: f64) -> &'static str {
        if temp < 0.0 {
            "Freeze"
        } else if temp >= 0.0 && temp < 10.0 {
            "Cloudy"
        } else if temp >= 10.0 && temp < 20.0 {
            "Mostly Cloudy"
        } else if temp >= 20.0 && temp < 30.0 {
            "Partially Cloudy"
        } else {
            "Sunny"
        }
    }
}

fn main() {
    println!("{}", "Welcome to the Weather Station!".bright_yellow());
    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input!");
        let city = city.trim();

        println!(
            "{}",
            "Please enter the country code(e.g. US for United States):".bright_green()
        );
        let mut country = String::new();
        io::stdin()
            .read_line(&mut country)
            .expect("Failed to read input!");
        let country = country.trim();

        let api_key = "test";

        match get_weather_info(city, country, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
        println!(
            "{}",
            "Do you want to see the weather for another city?(yes/no)"
        );
        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
            .expect("Failed to read input");
        let ans = ans.trim();

        if ans != "yes" {
            println!("Thank you for using our service!");
            break;
        }
    }
}
