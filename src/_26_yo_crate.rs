use rand::Rng;

pub fn main() {
    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Yo, random number: {}", random_number);
}

// Real Scenario: Leverage community-provided libraries for common tasks.
