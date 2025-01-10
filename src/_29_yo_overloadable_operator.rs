use std::ops::Add;

// Define a custom struct to represent a point in 2D space.
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Implement the Add trait to allow adding two Points together.
impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 3, y: 4 };

    let result = point1 + point2; // Overloaded operator
    println!("{:?}", result); // Expected output: Point { x: 4, y: 6 }
}
