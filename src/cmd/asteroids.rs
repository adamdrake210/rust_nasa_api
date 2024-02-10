use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Links {
    #[serde(rename = "self")]
    self_link: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EstimatedDiameter {
    kilometers: Diameter,
    meters: Diameter,
    miles: Diameter,
    feet: Diameter,
}

#[derive(Serialize, Deserialize, Debug)]
struct Diameter {
    estimated_diameter_min: f64,
    estimated_diameter_max: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct CloseApproachData {
    close_approach_date: String,
    close_approach_date_full: String,
    epoch_date_close_approach: i64,
    relative_velocity: RelativeVelocity,
    miss_distance: MissDistance,
    orbiting_body: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelativeVelocity {
    kilometers_per_second: String,
    kilometers_per_hour: String,
    miles_per_hour: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MissDistance {
    kilometers: String,
    astronomical: String,
    lunar: String,
    miles: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NearEarthObject {
    links: Links,
    id: String,
    neo_reference_id: String,
    name: String,
    nasa_jpl_url: String,
    absolute_magnitude_h: f64,
    estimated_diameter: EstimatedDiameter,
    is_potentially_hazardous_asteroid: bool,
    close_approach_data: Vec<CloseApproachData>,
    is_sentry_object: bool,
}

#[derive(Debug, Deserialize)]
struct Asteroid {
    element_count: i32,
    near_earth_objects: HashMap<String, Vec<NearEarthObject>>,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    start_date: String,
    end_date: String,
}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("NASA_API_KEY").expect("NASA_API_KEY must be set in .env file");

        let url = format!(
            "https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}",
            self.start_date, self.end_date, api_key
        );
        println!("URL: {}", url);
        match reqwest::blocking::get(&url)?.json::<Asteroid>() {
            Ok(response) => {
                println!("{:#?}", response);
                let mut file = std::fs::File::create("data/asteroids.txt")?;
                for (_date, asteroids) in response.near_earth_objects {
                    for asteroid in asteroids {
                        let data = format!(
                            "Name: {}\nEstimated Diameter: {} - {}\nMiss Distance (km): {}\n\n",
                            asteroid.name,
                            asteroid
                                .estimated_diameter
                                .kilometers
                                .estimated_diameter_min,
                            asteroid
                                .estimated_diameter
                                .kilometers
                                .estimated_diameter_max,
                            asteroid.close_approach_data[0].miss_distance.kilometers
                        );
                        file.write_all(data.as_bytes())?;
                    }
                }
            }
            Err(error) => {
                println!("Error occurred during deserialization: {}", error);
            }
        }
        Ok(())
    }
}
