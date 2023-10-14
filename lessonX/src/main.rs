mod coordinate;
use coordinate::point;
use log::{debug, error, info, LevelFilter};
use simple_logger::SimpleLogger;
use std::io;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("Failed to initialize logger");

    info!("Where am I v{}", env!("CARGO_PKG_VERSION"));

    let my_location = match get_my_location() {
        Ok(loc) => loc,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };
    debug!("my_location: {:?}", my_location);

    loop {
        let lat = match get_f32_from_stdin("Enter latitude:") {
            Ok(f) => f,
            Err(e) => {
                error!("Error getting latitude: {}", e);
                continue;
            }
        };
        let lon = match get_f32_from_stdin("Enter longitude:") {
            Ok(f) => f,
            Err(e) => {
                error!("Error getting longitude: {}", e);
                continue;
            }
        };
        let point2 = point::Point {
            latitude: lat,
            longitude: lon,
        };

        let mut distance = my_location.distance(&point2).into_kilometers();

        if distance.into_inner() < 500.00 {
            println!("You found me, I was hiding at {}!", my_location);
            break;
        }

        println!("{}", distance.temperature());

        debug!("distance: {}", distance);
        distance = distance.into_miles();
        debug!("distance: {}", distance);
    }
}

fn get_f32_from_stdin(question: &str) -> Result<f32, String> {
    let mut buffer = String::new();
    println!("{}", question);

    let stdin = io::stdin();
    if let Err(e) = stdin.read_line(&mut buffer) {
        error!("Error: {}", e);
        return Err(format!("Error: {}", e));
    }

    let buffer = buffer.trim();

    match buffer.parse::<f32>() {
        Ok(f) => Ok(f),
        Err(e) => {
            error!("Error: {}", e);
            Err(format!("Error: {}", e))
        }
    }
}

fn get_my_location() -> Result<point::Point, Box<dyn std::error::Error>> {
    let lat = match option_env!("MY_LAT") {
        Some(f) => f,
        None => {
            error!("MY_LAT environment variable not set");
            return Err("MY_LAT environment variable not set".into());
        }
    };

    let lon = match option_env!("MY_LON") {
        Some(f) => f,
        None => {
            error!("MY_LON environment variable not set");
            return Err("MY_LON environment variable not set".into());
        }
    };

    let lat: f32 = lat.parse()?;
    let lon: f32 = lon.parse()?;

    Ok(point::Point::new(lat, lon))
}
