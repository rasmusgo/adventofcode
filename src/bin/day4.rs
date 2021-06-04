use std::collections::BTreeSet;
use std::{io, io::prelude::*};

fn main() {
    let input = {
        let mut buffer = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        buffer
    };

    let required_fields = {
        let mut a = BTreeSet::new();
        a.insert("byr"); // (Birth Year)
        a.insert("iyr"); // (Issue Year)
        a.insert("eyr"); // (Expiration Year)
        a.insert("hgt"); // (Height)
        a.insert("hcl"); // (Hair Color)
        a.insert("ecl"); // (Eye Color)
        a.insert("pid"); // (Passport ID)

        // a.insert("cid"), // (Country ID) - optional!
        a
    };

    let mut num_valid_passports = 0;
    let mut num_invalid_passports = 0;
    let mut i = 0;
    for passport in input.split("\n\n") {
        let mut field_names = BTreeSet::new();
        for field in passport.split_ascii_whitespace() {
            let (field_name, _) = field
                .split_once(":")
                .expect("Failed to split field into name and data");
            field_names.insert(field_name);
        }
        if required_fields.is_subset(&field_names) {
            num_valid_passports += 1;
        } else {
            num_invalid_passports += 1;
        }
        println!("Passport {}: {}", i, passport);
        i += 1;
    }

    println!("Number of valid passports: {}", num_valid_passports);
    println!("Number of invalid passports: {}", num_invalid_passports);
}
