fn extract_first_word(sentence: &str) -> &str {
    if let Some(index) = sentence.find(' ') {
        &sentence[..index] // return the slice up to the first space
    } else {
        sentence // no space, return the whole string
    }
}

pub fn main() {
    let message = "YoRust programming is fun!";
    println!("First word: {}", extract_first_word(message)); // expected: "YoRust"
}
