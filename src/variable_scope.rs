pub fn main() {
    let x: i32 = 5; // x is valid within the main function's scope
    println!("x inside main: {}", x);

    {
        let y: i32 = 10; // y is valid inside this inner scope
        println!("y inside inner scope: {}", y);
    }

    // println!("y outside inner scope: {}", y); // This will cause a compile-time error

    // x is still accessible here
    println!("x outside inner scope: {}", x);
}
