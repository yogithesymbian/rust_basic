mod yo_utils {
    pub fn greet(name: &str) {
        println!("Yo, {}!", name); // Gree the user
    }
}

pub fn main() {
    yo_utils::greet("Rustacean"); // Expected: "Yo, Rustacean!"
}

// Real Scenario: Structure large projects for maintainability.
