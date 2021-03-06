use regex::Regex;
use std::{io, io::prelude::*};

// Example lines:
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
fn parse_line(s: &str) -> Option<(usize, usize, String, String)> {
    let r = Regex::new(r"([0-9]+)-([0-9]+) (.): (.*)").ok()?;
    let caps = r.captures(s)?;
    let min_count = caps.get(1)?.as_str().parse().ok()?;
    let max_count = caps.get(2)?.as_str().parse().ok()?;
    let letter = caps.get(3)?.as_str()[0..1].to_string();
    let password = caps.get(4)?.as_str().to_string();
    Some((min_count, max_count, letter, password))
}

fn count_letters(string: &str, needle: &str) -> usize {
    string.matches(needle).count()
}

fn part2_ok(low: usize, high: usize, letter: &str, password: &str) -> bool {
    let len = password.len();
    if high > len || low > len {
        panic!("index out of range!");
    }
    let match_low = &password[low - 1..low] == letter;
    let match_high = &password[high - 1..high] == letter;
    match_low != match_high
}

fn main() {
    let mut num_valid_passwords = 0;
    let mut num_invalid_passwords = 0;
    let mut num_part2_valid_passwords = 0;
    for line in io::stdin().lock().lines() {
        let (low, high, letter, password) = parse_line(&line.unwrap()).unwrap();
        let num_special_letters = count_letters(&password, &letter);
        if num_special_letters < low || num_special_letters > high {
            num_invalid_passwords += 1;
        } else {
            num_valid_passwords += 1;
        }
        if part2_ok(low, high, &letter, &password) {
            num_part2_valid_passwords += 1;
        }
    }
    println!("Number of invalid passwords: {}", num_invalid_passwords);
    println!("Number of valid passwords: {}", num_valid_passwords);
    println!(
        "Number of valid passwords for part 2: {}",
        num_part2_valid_passwords
    );
}
