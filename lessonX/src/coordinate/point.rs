use std::{fmt::Display, str::FromStr};

use super::distance::Distance;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Point {
    pub name: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    pub fn new(name: Option<&str>, latitude: f32, longitude: f32) -> Self {
        Self {
            name: name.map(|s| s.to_string()),
            latitude,
            longitude,
        }
    }

    pub fn _random() -> Self {
        let mut rng = thread_rng();
        Self {
            name: None,
            latitude: rng.gen_range(-90.0..90.0),
            longitude: rng.gen_range(-180.0..180.0),
        }
    }

    pub fn distance(&self, other: &Point) -> Distance {
        // haversine formula
        const KILOMETERS: f32 = 6371.0;

        let d_lat: f32 = (other.latitude - self.latitude).to_radians();
        let d_lon: f32 = (other.longitude - self.longitude).to_radians();
        let lat1: f32 = (self.latitude).to_radians();
        let lat2: f32 = (other.latitude).to_radians();

        let a: f32 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
            + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
        let c: f32 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

        Distance::Kilometers(KILOMETERS * c)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name {
            Some(ref name) => write!(
                f,
                "{}: lat: {}, lon:{}",
                name, self.latitude, self.longitude
            ),
            None => write!(f, "lat: {}, lon: {}", self.latitude, self.longitude),
        }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let name = parts.next().map(|s| s.to_string());
        let latitude = parts
            .next()
            .ok_or_else(|| "Missing latitude".to_string())?
            .parse::<f32>()
            .map_err(|e| format!("Error parsing latitude: {}", e))?;
        let longitude = parts
            .next()
            .ok_or_else(|| "Missing longitude".to_string())?
            .parse::<f32>()
            .map_err(|e| format!("Error parsing longitude: {}", e))?;

        Ok(Self {
            name,
            latitude,
            longitude,
        })
    }
}
