mod coordinate;
use coordinate::point::Point;

fn main() {
    let point = Point::new();
    let other = Point::new();

    let distance = point.distance(&other);

    println!(
        "Point: {}, {} is {distance} units from {}, {}",
        point.x, point.y, other.x, other.y
    )
}
