use std::io;

// Function to compute the sum of two integers
fn solve_me_first(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Variable declarations
    let mut num_str_1 = String::new();
    let mut num_str_2 = String::new();

    // Read input values
    io::stdin().read_line(&mut num_str_1).expect("Failed to read input");
    io::stdin().read_line(&mut num_str_2).expect("Failed to read input");

    // Parse the input strings to integers
    let num_1: i32 = num_str_1.trim().parse().expect("Failed to parse integer");
    let num_2: i32 = num_str_2.trim().parse().expect("Failed to parse integer");

    // Compute and print the sum
    println!("{}", solve_me_first(num_1, num_2));
}
