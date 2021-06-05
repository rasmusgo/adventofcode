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
        let persons: Vec<&str> = group.lines().collect();
        let mut primary_set = BTreeSet::new();
        for letter in persons[0].chars() {
            primary_set.insert(letter);
        }
        for person in persons[1..].iter() {
            let mut secondary_set = BTreeSet::new();
            for letter in person.chars() {
                secondary_set.insert(letter);
            }

            let diff: Vec<_> = primary_set.difference(&secondary_set).cloned().collect();
            for d in diff {
                primary_set.remove(&d);
            }
        }
        println!("{}: {}", group, primary_set.len());
        sum += primary_set.len();
    }
    println!("Sum: {}", sum);
}
