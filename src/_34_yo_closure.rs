// A closure is an anonymous function that can capture its environment. They are useful for things like callbacks or operations that require an external context.

// Example: Using Closure for Summation
pub fn main() {
    let add = |x, y| x + y;

    let result = add(5, 10);
    println!("{}", result); // Output: 15
}
