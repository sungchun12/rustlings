// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(()) // I had to add this line because the compiler was complaining that the main function was not returning a value.
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee: i32 = 1;
    let cost_per_item: i32 = 5;
    let qty: i32 = item_quantity.parse::<i32>()?; // the "?" operator is shorthand for the match statement in the previous exercise, it make this code more concise. without having to write a full match statement, it will return the error if the string is not a valid integer. In plain terms, "Is this an integer? If not, return the error. If it is, continue."

    Ok(qty * cost_per_item + processing_fee)
}
