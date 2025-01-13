use std::collections::{HashMap, HashSet};
use std::fs;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};
use clap::{Arg, Command};

/// Validates if the base is within the allowable range for the given charset.
fn validate_base(base: usize, charset: &str) -> bool {
    (2..=charset.len()).contains(&base)
}

/// Validates if the input string contains only valid characters for the given base.
fn validate_input(input_string: &str, base: usize, charset: &str) -> bool {
    let valid_chars: HashSet<char> = charset.chars().take(base).collect();
    input_string.chars().all(|c| valid_chars.contains(&c))
}

/// Converts a string from one base to another using the given charset.
fn convert_base(input_string: &str, base_from: usize, base_to: usize, charset: &str) -> String {
    let char_to_value: HashMap<char, usize> = charset.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let value_to_char: Vec<char> = charset.chars().collect();

    let mut decimal_value = BigUint::zero();
    let base_from_big = BigUint::from(base_from);

    for c in input_string.chars() {
        if let Some(&value) = char_to_value.get(&c) {
            decimal_value = decimal_value * &base_from_big + BigUint::from(value);
        }
    }

    if decimal_value.is_zero() {
        return value_to_char[0].to_string();
    }

    let base_to_big = BigUint::from(base_to);
    let mut result = Vec::new();

    while !decimal_value.is_zero() {
        let remainder = (&decimal_value % &base_to_big)
            .to_usize()
            .expect("Value too large for conversion");
        result.push(value_to_char[remainder]);
        decimal_value /= &base_to_big;
    }

    result.into_iter().rev().collect()
}

fn process_file(input_file: &str, output_file: &str, password_file: &str, encrypt: bool) {
    const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    let input_string = match fs::read_to_string(input_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let password_content = match fs::read_to_string(password_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading password file: {}", e);
            return;
        }
    };

    let bases: Vec<usize> = password_content
        .split_whitespace()
        .filter_map(|b| b.parse::<usize>().ok())
        .collect();

    let bases = if encrypt { bases } else { bases.into_iter().rev().collect::<Vec<_>>() };

    if bases.iter().any(|&base| !validate_base(base, CHARSET)) {
        eprintln!("Error: Password file contains invalid bases.");
        return;
    }

    if !validate_input(&input_string, CHARSET.len(), CHARSET) {
        eprintln!(
            "Error: The string '{}' contains invalid characters for the initial base.",
            input_string
        );
        return;
    }

    let mut current_string = input_string;
    for &base in bases.iter() {
        current_string = if encrypt {
            convert_base(&current_string, CHARSET.len(), base, CHARSET)
        } else {
            convert_base(&current_string, base, CHARSET.len(), CHARSET)
        };
    }

    if let Err(e) = fs::write(output_file, current_string) {
        eprintln!("Error writing to output file: {}", e);
    }
}

fn main() {
    let matches = Command::new("Problemathic")
        .version("1.0")
        .author("Mateja Nikolić matejanikolic@proton.me")
        .about("Encrypts or decrypts files using a base conversion method")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts the input file")
                .arg(Arg::new("INPUT").help("Input file").required(true))
                .arg(Arg::new("OUTPUT").help("Output file").required(true))
                .arg(Arg::new("PASSWORD").help("Password file").required(true)),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts the input file")
                .arg(Arg::new("INPUT").help("Input file").required(true))
                .arg(Arg::new("OUTPUT").help("Output file").required(true))
                .arg(Arg::new("PASSWORD").help("Password file").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let input = sub_matches.get_one::<String>("INPUT").unwrap();
            let output = sub_matches.get_one::<String>("OUTPUT").unwrap();
            let password = sub_matches.get_one::<String>("PASSWORD").unwrap();
            process_file(input, output, password, true);
        }
        Some(("decrypt", sub_matches)) => {
            let input = sub_matches.get_one::<String>("INPUT").unwrap();
            let output = sub_matches.get_one::<String>("OUTPUT").unwrap();
            let password = sub_matches.get_one::<String>("PASSWORD").unwrap();
            process_file(input, output, password, false);
        }
        _ => unreachable!(),
    }
}
