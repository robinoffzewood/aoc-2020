use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Opening file {}", filename);

    let start = Instant::now();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for (i, left) in numbers.iter().enumerate() {
        for (j, mid) in numbers[i..].iter().enumerate() {
            for (k, right) in numbers[j..].iter().enumerate() {
                let sum = left + mid + right;
                if sum == 2020 {
                    let duration = start.elapsed();
                    println!("Finished after {:?}", duration);
                    println!("left[{}] * mid[{}] * right[{}] = {}", i, j, k, left * mid * right);
                }
            }
        }
    }
}

