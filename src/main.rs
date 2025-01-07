mod array;
mod char;
mod constant;
mod data_types;
mod numbers;
mod operator_boolean;
mod operators_comparison;
mod variable_scope;
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
    println!("\n=========char=========");
    char::main();
    println!("\n=========array=========");
    array::main();
    println!("\n=========constant=========");
    constant::main();
    println!("\n=========variable scope=========");
    variable_scope::main();
}
