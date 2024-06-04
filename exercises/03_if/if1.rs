// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a >= b { a } else { b }
}
