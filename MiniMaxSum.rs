use std::io::{self, BufRead};


fn miniMaxSum(arr: &Vec<i32>) {
    // Calculate the total sum of the array
    let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();

    // Find the minimum and maximum values in the array and dereference them
    let min_value = *arr.iter().min().unwrap();
    let max_value = *arr.iter().max().unwrap();

    // Calculate the minimum and maximum sums
    let min_sum = total_sum - (max_value as i64);
    let max_sum = total_sum - (min_value as i64);

    // Print the result as space-separated integers
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
