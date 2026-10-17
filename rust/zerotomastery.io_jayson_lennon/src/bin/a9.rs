// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn return_coordinate() -> (i32, i32, i32) {
    (2, 5, 8)
}

fn main() {
    let (x, y, z) = return_coordinate();

    if y > 5 {
        println!("greater than 5");
    } else if y < 5 {
        println!("lesser than 5");
    } else {
        println!("equal to 5");
    }

}

// Things I've learned:
// - To destructure tuples. Either by accessing their fields using a dot, or by assigning their value(s) to a tuple variable.
// - Tuples can store any data type, even mix many in a single tuple.
