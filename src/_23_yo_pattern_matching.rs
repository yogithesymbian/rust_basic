pub fn main() {
    let value = Some(42);
    match value {
        Some(num) if num > 10 => println!("Values is greater than 10: {}", num),
        Some(num) => println!("Value is 10 or less: {}", num),
        None => println!("No value"),
    }
}
// Real Scenario: Handle optional data, like parsing user inputs.
