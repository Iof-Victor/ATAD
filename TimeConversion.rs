use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    // Extract the AM/PM part
    let period = &s[8..10];
    let mut hours: i32 = s[0..2].parse().unwrap();
    let minutes_seconds = &s[2..8];

    if period == "AM" {
        if hours == 12 {
            hours = 0;  
        }
    } else if period == "PM" {
        if hours != 12 {
            hours += 12;
        }
    }

    format!("{:02}{}", hours, minutes_seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
