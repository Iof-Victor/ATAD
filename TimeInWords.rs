use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeInWords(h: i32, m: i32) -> String {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "quarter", "sixteen", "seventeen",
        "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three",
        "twenty four", "twenty five", "twenty six", "twenty seven", "twenty eight",
        "twenty nine", "half"
    ];

    match m {
        0 => format!("{} o' clock", numbers[h as usize]),
        1 => format!("one minute past {}", numbers[h as usize]),
        15 => format!("quarter past {}", numbers[h as usize]),
        30 => format!("half past {}", numbers[h as usize]),
        45 => format!("quarter to {}", numbers[(h % 12 + 1) as usize]),
        59 => format!("one minute to {}", numbers[(h % 12 + 1) as usize]),
        1..=29 => format!("{} minutes past {}", numbers[m as usize], numbers[h as usize]),
        31..=59 => format!("{} minutes to {}", numbers[(60 - m) as usize], numbers[(h % 12 + 1) as usize]),
        _ => String::from("Invalid input"),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = timeInWords(h, m);

    writeln!(&mut fptr, "{}", result).ok();
}
