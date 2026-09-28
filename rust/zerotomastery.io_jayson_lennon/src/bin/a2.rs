// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = sum(2, 3);
    display_result(result);
}

// Things I've learned:
// - Difference between statements and expressions.
// - Statements perform an action and do not return a value.
// - Expressions evaluate to a value and return it.
// - Return is reserved for early returns; avoiding using the return keyword is idiomatic Rust.
// - You can return multiple values from one function using tuples or custom structs.
