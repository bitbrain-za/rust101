use std::fmt::Display;

use super::distance::Distance;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub latitude: f32,
    pub longitude: f32,
}

impl Point {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    pub fn _random() -> Self {
        let mut rng = thread_rng();
        Self {
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
        write!(f, "(lat: {}, lon: {})", self.latitude, self.longitude)
    }
}
