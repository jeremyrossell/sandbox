// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Lemon,
    Orange,
    Apple,
}

struct Drink {
    flavour: Flavour,
    ounces: f64,
}

fn print_drink_description(drink: Drink) {
    match drink.flavour {
        Flavour::Lemon => println!("Lemon"),
        Flavour::Orange => println!("Orange"),
        Flavour::Apple => println!("Apple"),
    }
    println!("{}", drink.ounces);
}

fn main() {
    let lemonade = Drink {
        flavour: Flavour::Lemon,
        ounces: 2.99,
    };
    print_drink_description(lemonade);
}

// Things I've learned:
// - To not forget variable declaration is a statement (do not forget the semicolon), even if it's an struct.
// - Enums are a set of mutually exclusive options, meanwhile Structs are a collection of related fields bundled together.
// - What is inside Enums are variants; what is inside Structs are fields.
