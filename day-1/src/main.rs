use std::env;
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Opening file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for (i, left) in numbers.iter().enumerate() {
        for (j, right) in numbers[i..].iter().enumerate() {
            let sum = left + right;
            if sum == 2020 {
                println!("left[{}] * right[{}] = {}", i, j, left * right);
            }
        }
    }
}
