use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn appendAndDelete(s: &str, t: &str, k: i32) -> String {
    // Find the length of the longest common prefix
    let mut common_length = 0;
    let min_len = s.len().min(t.len());

    for i in 0..min_len {
        if s.chars().nth(i) == t.chars().nth(i) {
            common_length += 1;
        } else {
            break;
        }
    }

    let deletions = s.len() - common_length;
    let additions = t.len() - common_length;
    let total_operations = deletions + additions;

    let k_usize = k as usize;

    // Check if we can perform exactly k operations
    if total_operations > k_usize {
        return "No".to_string();
    }

    if (k_usize - total_operations) % 2 == 0 || k_usize >= s.len() + t.len() {
        return "Yes".to_string();
    }

    "No".to_string()
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let t = stdin_iterator.next().unwrap().unwrap();

    let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = appendAndDelete(&s, &t, k);

    writeln!(&mut fptr, "{}", result).ok();
}
