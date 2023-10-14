use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Distance {
    Miles(f32),
    Kilometers(f32),
}

impl Distance {
    const MILES_TO_KM: f32 = 1.60934;
    const KM_TO_MILES: f32 = 0.621371;

    pub fn into_miles(self) -> Self {
        match self {
            Self::Miles(_) => self,
            Self::Kilometers(km) => Self::Miles(km * Self::KM_TO_MILES),
        }
    }

    pub fn into_kilometers(self) -> Self {
        match self {
            Self::Miles(miles) => Self::Kilometers(miles * Self::MILES_TO_KM),
            Self::Kilometers(_) => self,
        }
    }

    pub fn into_inner(self) -> f32 {
        match self {
            Self::Miles(miles) => miles,
            Self::Kilometers(km) => km,
        }
    }

    pub fn temperature(&self) -> String {
        let distance: u32 = (self.into_kilometers().into_inner() * 100.0) as u32;
        match distance {
            0..=500 => "You found me".to_string(),
            501..=2000 => "Super Heated".to_string(),
            2001..=10000 => "Hot".to_string(),
            10001..=100000 => "Warm".to_string(),
            100001..=1000000 => "Cool".to_string(),
            1000001..=10000000 => "Cold".to_string(),
            _ => "Frozen".to_string(),
        }
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Miles(miles) => write!(f, "{:.2} miles", miles),
            Self::Kilometers(km) => write!(f, "{:.2} km", km),
        }
    }
}
