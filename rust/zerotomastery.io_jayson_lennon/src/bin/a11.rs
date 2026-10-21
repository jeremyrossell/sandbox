// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_groceryitem_quantity(item: &GroceryItem) {
    println!("quantity: {}", item.quantity);
}

fn display_groceryitem_id(item: &GroceryItem) {
    println!("id: {}", item.id);
}

fn main() {
    let milk = GroceryItem {
        quantity: 5,
        id: 12345,
    };
    display_groceryitem_quantity(&milk);
    display_groceryitem_id(&milk);
}

// I've learned that:
// - Rust uses the borrow-checker model to manage memory, where only one function at a time owns a piece of memory, and drops it when the scope ends.
// - Memory (value) can be moved to another function.
// - Memory (value) can be borrowed, which is read-only, by other functions.
// - The ampersand (&) goes on the struct data type name of the parameter instead of the value's name.
//      Types define what the memory looks like (GroceryItem vs &GroceryItem).
//      Names (item, milk) are just labels for human brains to keep track of variables.
