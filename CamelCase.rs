use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn camelcase(s: &str) -> i32 {
    s.chars().filter(|&c| c.is_uppercase()).count() as i32 + 1
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = camelcase(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
