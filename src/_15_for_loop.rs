pub fn main() {
    for i in 1..=5 {
        println!("For loop iteration: {}", i);
    }

    let array = [10, 20, 30];
    for item in array.iter() {
        println!("Yo, Array of item : {}", item);
    }
}
