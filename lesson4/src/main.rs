mod coordinate;
use coordinate::point::Point;

fn main() {
    let point = Point::new();

    loop {
        let lat = match get_f32_from_stdin("Enter latitude:") {
            Ok(f) => f,
            Err(e) => {
                println!("Error getting latitude: {}", e);
                continue;
            }
        };
        let lon = match get_f32_from_stdin("Enter longitude:") {
            Ok(f) => f,
            Err(e) => {
                println!("Error getting longitude: {}", e);
                continue;
            }
        };
        let point2 = point::Point {
            latitude: lat,
            longitude: lon,
        };

        println!("You guessed: {}", point2);
    }
}

fn get_f32_from_stdin(question: &str) -> Result<f32, String> {
    let mut buffer = String::new();
    println!("{}", question);

    let stdin = io::stdin();
    if let Err(e) = stdin.read_line(&mut buffer) {
        return Err(format!("Error getting line: {}", e));
    }

    let buffer = buffer.trim();

    match buffer.parse::<f32>() {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Error parsing float: {}", e)),
    }
}
