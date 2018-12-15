use std::env;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    chronal_calibration(filename.to_string());
}

fn chronal_calibration(filename: String) {
    let mut current_frequency: i32 = 0;
    let mut frequencies = HashSet::new();
    let mut unfinished: bool = true;

    while unfinished {
        let file = File::open(filename.clone()).expect("Existing file");

        unfinished = read_frequencies(&mut current_frequency, &mut frequencies,
                                       file);
    }

    println!("{}", current_frequency);
}

fn read_frequencies(current_frequency: &mut i32,
                    frequencies: &mut HashSet<String>, file: File) -> bool {
    for line in BufReader::new(file).lines() {
        let new_frequency: i32 = line.unwrap().parse().expect("Valid integer");
        *current_frequency += new_frequency;

        if !(*frequencies).insert((*current_frequency).to_string()) {
            return false;
        }
    }

    return true;
}
