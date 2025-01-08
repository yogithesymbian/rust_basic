pub fn main() {
    // Fixed-size array with 5 elements of type i32
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Iterating over the array
    for number in numbers.iter() {
        println!("Array element: {}", number);
    }
}
