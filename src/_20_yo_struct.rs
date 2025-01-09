struct YoUser {
    username: String,
    age: u8,
    active: bool,
}

pub fn main() {
    let user = YoUser {
        username: String::from("yo_rustacean"),
        age: 28,
        active: true,
    };

    println!(
        "User : {}, Age : {}, Active : {}",
        user.username, user.age, user.active
    ); // expected : user details printed
}

// Real Scenario: Represent entities like users, products, or accounts.
