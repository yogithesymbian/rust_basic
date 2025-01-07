mod data_types;
mod numbers;
mod operator_boolean;
mod operators_comparison;
mod variables;

fn main() {
    println!("Hello, world!");

    println!("\n=========variables=========");
    variables::main();
    println!("\n=========data_types=========");
    data_types::main();
    println!("\n=========numbers=========");
    numbers::main();
    println!("\n=========comparsion_operator=========");
    operators_comparison::main();
    println!("\n=========boolean_operator=========");
    operator_boolean::main();
}
