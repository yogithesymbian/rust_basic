pub fn main() {
    let mut _counter = 0;
    loop {
        _counter += 1;
        println!("Loop iteration: {}", _counter);

        if _counter > 5 {
            break;
        }
    }
    println!("loop finished");
}
