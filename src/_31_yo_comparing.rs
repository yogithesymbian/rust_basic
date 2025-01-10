use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq)]
struct Product {
    price: f64,
    name: String,
}

pub fn main() {
    let product1 = Product {
        price: 19.99,
        name: "Laptop".to_string(),
    };
    let product2 = Product {
        price: 29.99,
        name: "Smartphone".to_string(),
    };

    if product1 < product2 {
        println!("{} is cheaper than {}", product1.name, product2.name);
    } else {
        println!("{} is more expensive than {}", product1.name, product2.name);
    }
}
