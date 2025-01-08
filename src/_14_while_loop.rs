pub fn main() {
    let mut counter = 1;

    while counter <= 5 {
        println!("Yo, count: {}", counter); // Expected: Print "Yo, count: 1" to "Yo, count: 5"
        counter += 1;
    }
    println!("Finished loop");
}
