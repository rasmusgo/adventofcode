use failure::{bail, format_err};
use lazy_static::lazy_static;
use regex::Regex;
use std::{io, io::prelude::*};
use std::collections::BTreeSet;

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

type Result<T> = std::result::Result<T, failure::Error>;

fn validate_integer(s: &str, min: u32, max: u32) -> Result<()> {
    let i: u32 = s.parse::<u32>()?;
    if i < min {
        bail!("Too small");
    }
    if i > max {
        bail!("Too large");
    }
    Ok(())
}

fn validate_height(text: &str) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)((cm)|(in))$").unwrap();
    }
    // let RE: Regex = Regex::new(r"^(\d+)((cm)|(in))$").unwrap();

    let caps = RE.captures(text).ok_or(format_err!("Height didn't match regex"))?;
    let value = caps.get(1).unwrap().as_str();
    let unit = caps.get(2).unwrap().as_str();

    match unit {
        "cm" => validate_integer(value, 150, 193)?,
        "in" => validate_integer(value, 59, 76)?,
        _ => panic!("Unexpected unit"),
    }
    Ok(())
}

fn validate_hair_color(s: &str) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    if RE.is_match(s) {
        Ok(())
    } else {
        bail!("Invalid hair color");
    }
}

fn validate_eye_color(s: &str) -> Result<()> {
    match s {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
        _ => bail!("Invalid eye color"),
    }
}

fn validate_pid(s: &str) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    if RE.is_match(s) {
        Ok(())
    } else {
        bail!("Invalid Passport ID");
    }
}

fn validate_field(field_name: &str, field_data: &str) -> Result<()> {
    match field_name {
        // (Birth Year) - four digits; at least 1920 and at most 2002.
        "byr" => validate_integer(field_data, 1920, 2002)?,
        // (Issue Year) - four digits; at least 2010 and at most 2020.
        "iyr" => validate_integer(field_data, 2010, 2020)?,
        // (Expiration Year) - four digits; at least 2020 and at most 2030.
        "eyr" => validate_integer(field_data, 2020, 2030)?,
        // (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        "hgt" => validate_height(field_data)?,
        // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        "hcl" => validate_hair_color(field_data)?,
        // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        "ecl" => validate_eye_color(field_data)?,
        // (Passport ID) - a nine-digit number, including leading zeroes.
        "pid" => validate_pid(field_data)?,
        // (Country ID) - ignored, missing or not.
        "cid" => (),
        _ => bail!("unexpected field name"),
    }
    Ok(())
}

fn validate_passport(passport: &str) -> Result<()> {
    let mut field_names = BTreeSet::new();
    for field in passport.split_ascii_whitespace() {
        let (field_name, field_data) = field
            .split_once(":")
            .expect("Failed to split field into name and data");
        field_names.insert(field_name);
        validate_field(field_name, field_data)?;
    }
    for required_field in REQUIRED_FIELDS.iter() {
        if !field_names.contains(required_field) {
            bail!(format!("Missing {}", required_field));
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
