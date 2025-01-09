enum YoStatus {
    Online,
    Offline,
    Busy,
}

fn print_status(status: YoStatus) {
    match status {
        YoStatus::Online => println!("User is online"),
        YoStatus::Offline => println!("User is offline"),
        YoStatus::Busy => println!("User is busy"),
    }
}

pub fn main() {
    let current_status = YoStatus::Online;
    print_status(current_status); // Expected: "User is online"
}
