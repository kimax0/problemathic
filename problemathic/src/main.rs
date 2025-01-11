use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

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
    // Mapping characters to their values and vice versa
    let char_to_value: HashMap<char, usize> = charset.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let value_to_char: Vec<char> = charset.chars().collect();

    // Convert the input string to a decimal value
    let mut decimal_value = BigUint::zero();
    let base_from_big = BigUint::from(base_from);

    for c in input_string.chars() {
        if let Some(&value) = char_to_value.get(&c) {
            decimal_value = decimal_value * &base_from_big + BigUint::from(value);
        }
    }

    // Handle zero case explicitly
    if decimal_value.is_zero() {
        return value_to_char[0].to_string();
    }

    // Convert decimal value to the target base
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

fn main() {
    // The character set supports up to base 95
    const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: problemathic <input_file> <output_file> <base_from> <base_to>");
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let base_from = args[3].parse::<usize>();
    let base_to = args[4].parse::<usize>();

    // Parse and validate the bases
    match (base_from, base_to) {
        (Ok(base_from), Ok(base_to)) => {
            if validate_base(base_from, CHARSET) && validate_base(base_to, CHARSET) {
                match fs::read_to_string(input_file) {
                    Ok(input_string) => {
                        let input_string = &input_string;

                        if validate_input(input_string, base_from, CHARSET) {
                            let result = convert_base(input_string, base_from, base_to, CHARSET);
                            if let Err(e) = fs::write(output_file, result) {
                                eprintln!("Error writing to output file: {}", e);
                            }
                        } else {
                            eprintln!(
                                "Error: The string '{}' contains invalid characters for base {}.",
                                input_string, base_from
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading input file: {}", e);
                    }
                }
            } else {
                eprintln!(
                    "Error: Bases must be between 2 and the length of the charset ({}).",
                    CHARSET.len()
                );
            }
        }
        _ => eprintln!("Error: Invalid base values. Ensure both bases are integers."),
    }
}
