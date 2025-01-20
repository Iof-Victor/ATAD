use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};


fn divisibleSumPairs(n: i32, k: i32, ar: &Vec<i32>) -> i32 {
    let mut remainder_count = vec![0; k as usize]; // To count occurrences of each remainder
    let mut pair_count = 0; // To store the number of valid pairs

    for &num in ar.iter() {
        let remainder = ((num % k) + k) % k;  
        let complement = (k - remainder) % k;

        // Add the count of previous elements that complement the current element
        pair_count += remainder_count[complement as usize];
        remainder_count[remainder as usize] += 1;
    }
    pair_count 
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

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisibleSumPairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}