use rand::{thread_rng, Rng};

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..100),
            y: rng.gen_range(0..100),
        }
    }

    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn distance(&self, other: &Point) -> f64 {
        let x = (self.x - other.x).pow(2);
        let y = (self.y - other.y).pow(2);

        println!("x: {}, y: {}", x, y);

        ((x + y) as f64).sqrt()
    }
}

fn main() {
    let mut point = Point::new();
    let other = Point::new();

    point.add(&other);

    let distance = point.distance(&other);

    println!(
        "Point: {}, {} is {distance} units from {}, {}",
        point.x, point.y, other.x, other.y
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let point = Point { x: 2, y: 3 };
        let other = Point { x: 6, y: 6 };

        let distance = point.distance(&other);

        assert!(distance == 5.0);
    }
}
