use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};


fn findDigits(n: i32) -> i32 {
    let n_str = n.to_string();
    let mut count = 0;

    for digit_char in n_str.chars() {
        let digit = digit_char.to_digit(10).unwrap() as i32;  // Convert char to digit

        if digit != 0 && n % digit == 0 {
            count += 1;
        }
    }

    count
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = findDigits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}