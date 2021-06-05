use std::{io, io::prelude::*};
use std::collections::BTreeSet;

fn main() {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    let input = input;
//     let input = String::from("abc

// a
// b
// c

// ab
// ac

// a
// a
// a
// a

// b
// ");

    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut set_of_letters = BTreeSet::new();
        for letter in group.chars() {
            set_of_letters.insert(letter);
        }
        set_of_letters.remove(&'\n');
        sum += set_of_letters.len();
    }
    println!("Sum: {}", sum);
}
