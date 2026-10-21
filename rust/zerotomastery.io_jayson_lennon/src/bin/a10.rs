// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_variable_size(result: bool) {
    match result {
        true => println!("its big"),
        _ => println!("its small"),
    }
}

fn main() {
    let x = 100;
    let result = x > 100;
    print_variable_size(result);
}

// I've learned that:
// - Rust is an expression-based language.
// - Naturally, comparisons evaluate to something, therefore they are expressions, so we can skip verbose and redundant code.
