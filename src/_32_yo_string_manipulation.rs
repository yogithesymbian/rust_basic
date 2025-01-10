pub fn main() {
    let first_name = "Yogi";
    let last_name = "Arif Widodo";

    // Concatenate strings
    let full_name = format!("{} {}", first_name, last_name);
    println!("{}", full_name); // Output: Yogi Arif Widodo

    // String manipulation: slicing
    let first_char = &full_name[0..1];
    println!("{}", first_char); // Output: Y
}
