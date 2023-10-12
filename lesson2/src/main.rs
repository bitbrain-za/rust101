use rand::random as randomizer;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Self {
            x: randomizer(),
            y: randomizer(),
        }
    }

    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let point = Point::new();
    let other = Point::new();

    let third_point = point.add(other);

    // println!("Point: {}, {}", point.x, point.y);
    println!("New Point: {}, {}", third_point.x, third_point.y);
}
