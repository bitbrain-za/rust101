use rand::{thread_rng, Rng};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..100),
            y: rng.gen_range(0..100),
        }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let x = (self.x - other.x).pow(2);
        let y = (self.y - other.y).pow(2);

        println!("x: {}, y: {}", x, y);

        ((x + y) as f64).sqrt()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_distance() {
        let point = Point { x: 2, y: 3 };
        let other = Point { x: 6, y: 6 };

        let distance = point.distance(&other);

        assert!(distance == 5.0);
    }
}
