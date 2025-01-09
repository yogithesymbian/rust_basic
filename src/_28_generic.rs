fn yo_swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

pub fn main() {
    let mut x = 5;
    let mut y = 10;
    yo_swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y); // Expected: "x: 10, y: 5"
}

// Real Scenario: Write functions that operate on multiple data types.
