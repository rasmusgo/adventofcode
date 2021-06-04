use std::collections::BTreeSet;
use std::{io, io::prelude::*};

static REQUIRED_FIELDS: [&str; 7] = [
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
           // "cid", // (Country ID) - optional!
];

fn validate_passport(passport: &str) -> Result<(), ()> {
    let mut field_names = BTreeSet::new();
    for field in passport.split_ascii_whitespace() {
        let (field_name, _) = field
            .split_once(":")
            .expect("Failed to split field into name and data");
        field_names.insert(field_name);
    }
    for required_field in REQUIRED_FIELDS.iter() {
        if !field_names.contains(required_field) {
            return Err(());
        }
    }
    Ok(())
}

fn main() {
    let input = {
        let mut buffer = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        buffer
    };

    let mut num_valid_passports = 0;
    let mut num_invalid_passports = 0;
    for passport in input.split("\n\n") {
        match validate_passport(passport) {
            Ok(_) => {
                num_valid_passports += 1;
            }
            Err(_) => {
                num_invalid_passports += 1;
            }
        }
        // println!("Passport {}: {}", i, passport);
    }

    println!("Number of valid passports: {}", num_valid_passports);
    println!("Number of invalid passports: {}", num_invalid_passports);
}
