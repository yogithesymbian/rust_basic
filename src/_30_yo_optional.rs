fn find_item(value: i32) -> Option<String> {
    if value > 10 {
        Some("Item found!".to_string())
    } else {
        None
    }
}

pub fn main() {
    let result = find_item(15);

    match result {
        Some(message) => println!("{}", message), // Output: Item found !
        None => println!("No item found."),
    }
}
