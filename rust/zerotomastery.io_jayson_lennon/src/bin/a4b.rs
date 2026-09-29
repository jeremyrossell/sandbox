// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let number: i32 = 1;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

// Things I've learned:
// - To not declare data type if it is assumed by inference, like local variables.
// - Declare data type when Rust forces it to, such as function signatures, structs, and constants.
// - Declare data type to disambiguate ambiguous methods, the compiler needs to know your target type.
// - Using turbofish ::<T>: syntax makes the variable name more clean, example:
//      let count = "42".parse::<i32>().expect("Not a number");
