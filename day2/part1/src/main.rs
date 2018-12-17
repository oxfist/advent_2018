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
    let mut two_letters_count: i32 = 0;
    let mut three_letters_count: i32 = 0;

    for line in BufReader::new(file).lines() {
        let current_id: String = line.expect("Valid string");
        count_letters(current_id, &mut two_letters_count,
                      &mut three_letters_count);
    }

    println!("{}", two_letters_count * three_letters_count);
}

fn count_letters(id: String, two_letter_count: &mut i32,
                 three_letter_count: &mut i32) -> HashMap<String, i32> {
    let mut letter_count: HashMap<String, i32> = HashMap::<String, i32>::new();
    let mut found_twice: bool = false;
    let mut found_three_times: bool = false;

    for letter in split_letters(id) {
        let count = letter_count.entry(letter).or_insert(0);
        *count += 1;
    }

    for count in letter_count.values() {
        if !found_twice && *count == 2 {
            *two_letter_count += 1;
            found_twice = true;
        }

        if !found_three_times && *count == 3 {
            *three_letter_count += 1;
            found_three_times = true;
        }

        if found_twice && found_three_times {
            break;
        }
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
