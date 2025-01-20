use std::io::{self, BufRead};

fn plusMinus(arr: &[i32]) {
    let total = arr.len() as f64;

    let (mut positive, mut negative, mut zero) = (0, 0, 0);

    for &num in arr {
        match num {
            x if x > 0 => positive += 1,
            x if x < 0 => negative += 1,
            _ => zero += 1,
        }
    }

    println!("{:.6}", positive as f64 / total);
    println!("{:.6}", negative as f64 / total);
    println!("{:.6}", zero as f64 / total);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if n == arr.len() {
        plusMinus(&arr);
    }
}