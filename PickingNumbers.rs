use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pickingNumbers(a: &Vec<i32>) -> i32 {
    let mut count = vec![0; 101]; // Assuming numbers range from 0 to 100

    // Count occurrences of each number
    for &num in a.iter() {
        count[num as usize] += 1;
    }

    let mut max_length = 0;

    // Find the max length by considering adjacent numbers
    for i in 0..100 {
        max_length = max_length.max(count[i] + count[i + 1]);
    }

    max_length
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pickingNumbers(&a);

    writeln!(&mut fptr, "{}", result).ok();
}