// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut i = 1;

    loop {
        if i < 0 || i > 4 {
            break;
        }

        println!("{:?}", i);
        i = i + 1;
    }
}

// Things I've learned:
// - Always check conditionals at the beginning, and look at the sad case instead of the happy case, this is called "guard clause or early return",
//   this way we avoid burying the actual logic inside nested code. Funnily known as "pyramid of doom".
// - Keeping the conditional at the top guarantees the code to run 0 or more times, this is the standard practice.
// - Keeping the conditional at the bottom guarantees the code to run 1 or more times, used only when we need the code to run at least once.
