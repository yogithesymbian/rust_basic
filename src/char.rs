pub fn main() {
    let char1: char = 'R';
    let char2: char = 'u';
    let char3: char = 's';
    let char4: char = 't';

    println!("Character 1: {}", char1);
    println!("Character 2: {}", char2);
    println!("Character 3: {}", char3);
    println!("Character 4: {}", char4);

    // Combining characters into a string
    let word = format!("{}{}{}{}", char1, char2, char3, char4);
    println!("Word formed: {}", word);
}
