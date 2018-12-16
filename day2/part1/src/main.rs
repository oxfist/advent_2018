use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("A valid string");

    inventory_management_system(filename.to_string());
}

fn inventory_management_system(filename: String) {
    let file = File::open(filename).expect("Existing file");

    for line in BufReader::new(file).lines() {
        println!("{}", line.expect("Valid string"));
    }
}
