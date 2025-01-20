use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};


fn nonDivisibleSubset(k: i32, s: &Vec<i32>) -> i32 {
    let mut remainder_count = vec![0; k as usize];

    for &num in s {
        let remainder = ((num % k) + k) % k;
        remainder_count[remainder as usize] += 1;
    }

    // Initialize result with at most one element from the remainder 0 group
    let mut result = if remainder_count[0] > 0 { 1 } else { 0 };

    for i in 1..=(k / 2) {
        if i == k - i {
            // If i == k-i, we can only include one element from this group
            result += 1;
        } else {
            // Otherwise, choose the maximum count from the two complementary remainders
            result += std::cmp::max(remainder_count[i as usize], remainder_count[(k - i) as usize]);
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = nonDivisibleSubset(k, &s);

    writeln!(&mut fptr, "{}", result).ok();
}
