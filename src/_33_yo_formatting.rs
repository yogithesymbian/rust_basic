use std::fmt;

struct Product {
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ${:.2}", self.name, self.price)
    }
}

pub fn main() {
    let product = Product {
        name: "Laptop".to_string(),
        price: 999.99,
    };

    println!("{}", product); // Output: Laptop: $999.99
}
