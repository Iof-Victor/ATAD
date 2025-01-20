use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn equalizeArray(arr: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();

    // Count the frequency of each element in the array
    for &elem in arr.iter() {
       *frequency_map.entry(elem).or_insert(0) += 1;
    }

    let max_frequency =frequency_map.values().max().unwrap();

    // The minimum deletions needed is the total number of elements minus the maximum frequency
    arr.len() as i32 - max_frequency
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = equalizeArray(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}