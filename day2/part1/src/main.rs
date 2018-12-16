use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("A valid string");

    inventory_management_system(filename.to_string());
}

fn inventory_management_system(filename: String) {
    let file: File = File::open(filename).expect("Existing file");

    // Variables for couting the amount of IDs that have exactly two of any
    // letter and three of any letter, respectively.
    let exactly_two_letters_count: i32 = 0;
    let exactly_three_letters_count: i32 = 0;

    for line in BufReader::new(file).lines() {
        let current_id: String = line.expect("Valid string");
        count_letters(current_id);
    }

    println!("{}", exactly_two_letters_count * exactly_three_letters_count);
}

fn count_letters(id: String) -> HashMap<String, i32> {
    let letter_count: HashMap<String, i32> = HashMap::<String, i32>::new();
    let letters: Vec<String> = split_letters(id);

    for letter in letters {
        println!("{}", letter);
    }

    return letter_count;
}

fn split_letters(id: String) -> Vec<String> {
    let mut letters: Vec<String> = id.split("").map(|s| s.to_string())
                                     .collect();

    letters.remove(0);
    letters.pop();

    return letters;
}
