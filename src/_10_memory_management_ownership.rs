pub fn main() {
    let s1 = String::from("Hello, Ruest!");
    let s2 = s1; // Ownership is moved to s2, s1 is no longer valid

    // println!("{}", s1); // This would cause a compile-time error
    println!("{}", s2);
}
