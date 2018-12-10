use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut current_frequency: i32 = 0;

    for line in stdin.lock().lines() {
        let new_frequency: i32 = line.unwrap().parse().unwrap();
        current_frequency += new_frequency;
    }

    println!("{}", current_frequency);
}
