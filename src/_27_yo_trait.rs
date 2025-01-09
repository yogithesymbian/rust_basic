trait YoGreet {
    fn greet(&self) -> String;
}

struct YoPerson {
    name: String,
}

impl YoGreet for YoPerson {
    fn greet(&self) -> String {
        format!("Yo, {}!", self.name)
    }
}

pub fn main() {
    let person = YoPerson {
        name: String::from("Rustacean"),
    };
    println!("{}", person.greet()); // Expected : "Yo, Rustacean!"
}

// Real Scenario: Define shared behavior for unrelated types.
