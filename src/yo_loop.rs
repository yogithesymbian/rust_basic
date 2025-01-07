pub fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Loop iteration: {}", counter);

        if counter > 5 {
            break;
        }
    }
    println!("loop finished");
}
