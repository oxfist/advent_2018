use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    chronal_calibration(filename.to_string());
}

fn chronal_calibration(filename: String) {
    let file = File::open(filename).expect("Existing file");
    let mut current_frequency: i32 = 0;
    let frequencies = HashMap::new();

    read_frequencies(&mut current_frequency, frequencies, file);

    println!("{}", current_frequency);
}

fn read_frequencies(current_frequency: &mut i32,
                    frequencies: HashMap<&str, i32>, file: File) {
    for line in BufReader::new(file).lines() {
        let new_frequency: i32 = line.unwrap().parse().expect("Valid integer");
        *current_frequency += new_frequency;
    }
}
