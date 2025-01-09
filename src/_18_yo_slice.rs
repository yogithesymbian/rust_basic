fn calculate_sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
pub fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let slice = &numbers[1..4]; //slice includes [20, 30, 40]
    println!("Sum of slice : {}", calculate_sum(slice)) // expected result : 90
}
