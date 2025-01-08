fn take_ownerhsip(s: String) {
    println!("Yo, data ownership taken: {}", s)
}
pub fn main() {
    let s = String::from("Hello Rust");
    take_ownerhsip(s);
    // println!("{}", s); // This will cause an error because ownership is moved.
}
