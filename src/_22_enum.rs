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
    print_status(current_status); // Expected: "User is Online"
    let current_status2 = YoStatus::Offline;
    print_status(current_status2); // Expected: "User is Offline"
    let current_status3 = YoStatus::Busy;
    print_status(current_status3); // Expected: "User is Busy"
}
// Real Scenario: Represent states or configurations, such as a network status.
