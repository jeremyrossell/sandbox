// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colour {
    Red,
    Green,
    Blue,
}

fn print_colour_name(colour_name: Colour) {
    match colour_name {
        Colour::Red => println!("Red"),
        Colour::Green => println!("Green"),
        Colour::Blue => println!("Blue"),
    }
}

fn main() {
    let colour_name = Colour::Blue;
    print_colour_name(colour_name);
}

// Things I've learned:
// - Rust warns you about unused or dead code in case you forgot to clean up after changing how the program works, or incomplete features.
// - Rust's compiler is designed to be strict to help you write cleaner, bug-free software. By warning you about dead code, Rust helps you:
//      + Reduce code bloat: Prevent dead structures and unused states from cluttering your binaries.
//      + Catch logic bugs early: If you thought your program was using a specific enum variant, but the compiler says it's dead code,
//                                it immediately reveals a bug where you forgot to hook up that part of your logic.
