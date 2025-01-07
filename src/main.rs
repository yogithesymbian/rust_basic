mod data_types;
mod numbers;
mod variables;

fn main() {
    println!("Hello, world!");

    println!("\n=========variables=========");
    variables::main();
    println!("\n=========data_types=========");
    data_types::main();
    println!("\n=========numbers=========");
    numbers::main();
}
