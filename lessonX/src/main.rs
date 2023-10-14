mod coordinate;
use coordinate::point;
use log::{debug, error, info, LevelFilter};
use simple_logger::SimpleLogger;
use std::{fs, io, str::FromStr};

fn main() {
    /* See
    https://doc.rust-lang.org/stable/rust-by-example/attribute/cfg.html
    https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions
    */

    let level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    /* https://crates.io/crates/simple_logger */
    SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("Failed to initialize logger");

    /* https://doc.rust-lang.org/cargo/reference/environment-variables.html */
    info!("Where am I v{}", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = std::env::args().collect();
    let mut landmarks: Vec<point::Point> = Vec::new();
    if args.len() > 1 {
        debug!("{:?}", args);
        let path = std::path::Path::new(&args[1]);
        debug!("path: {:?}", path);
        fs::read_to_string(path)
            .expect("Error reading file")
            .lines()
            .for_each(|line| {
                landmarks.push(point::Point::from_str(line).expect("Error parsing line"));
            });
    }

    /* https://doc.rust-lang.org/rust-by-example/flow_control/match.html */
    let my_location = match get_my_location() {
        Ok(loc) => loc,
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };
    debug!("{}", my_location);
    landmarks.push(my_location);

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
            name: Some("Me".to_string()),
            latitude: lat,
            longitude: lon,
        };

        landmarks.sort_by(|a, b| {
            a.distance(&point2)
                .into_inner()
                .total_cmp(&b.distance(&point2).into_inner())
        });

        let mut distance = landmarks[0].distance(&point2).into_kilometers();

        /* https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html */
        if let coordinate::distance::Distance::Kilometers(km) = distance {
            debug!("You are {} kilometers from {}", km, landmarks[0]);
        }
        if let coordinate::distance::Distance::Miles(mi) = distance {
            debug!("You are {} miles from {}", mi, landmarks[0]);
        }

        if distance.into_inner() < 500.00 {
            println!("You found me, I was hiding at {}!", landmarks[0]);
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
    /* https://doc.rust-lang.org/std/macro.option_env.html */
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

    Ok(point::Point::new(None, lat, lon))
}
