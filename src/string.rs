pub fn main() {
    // String from a literal
    let s1 = "Hello".to_string();
    println!("s1: {}", s1);

    // String from a `String` type
    let s2 = String::from("World");
    println!("s2: {}", s2);

    // String concatenation
    let s3 = s1 + " " + &s2;
    println!("s3: {}", s3);
}
