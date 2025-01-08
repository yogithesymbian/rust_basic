fn greet(name: &str) {
    println!("Yo Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn main() {
    greet("Alice");
    let sum = add(5, 3);
    println!("Sum: {}", sum)
}
