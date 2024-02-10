use clap::Parser;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct NasaPhoto {
    copyright: Option<String>,
    data: Option<String>,
    explanation: String,
    hdurl: Option<String>,
    media_type: String,
    service_version: String,
    title: String,
    url: String,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    date: String,
}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("NASA_API_KEY").expect("NASA_API_KEY must be set in .env file");

        let url = format!(
            "https://api.nasa.gov/planetary/apod?api_key={}&date={}",
            api_key, self.date
        );
        match reqwest::blocking::get(&url)?.json::<NasaPhoto>() {
            Ok(response) => {
                println!("{:#?}", response.hdurl.unwrap_or(response.url));
            }
            Err(error) => {
                println!("Error occurred during deserialization: {}", error);
            }
        }
        Ok(())
    }
}
