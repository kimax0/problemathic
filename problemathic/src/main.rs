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

    if args.len() != 4 {
        eprintln!("Usage: problemathic <input_file> <output_file> <password_file>");
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let password_file = &args[3];

    // Read the input file
    let input_string = match fs::read_to_string(input_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    // Read the password file and parse bases
    let password_content = match fs::read_to_string(password_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading password file: {}", e);
            return;
        }
    };

    let base_to = CHARSET.len();
    let bases: Vec<usize> = password_content
        .split_whitespace()
        .filter_map(|b| b.parse::<usize>().ok())
        .rev() // Reverse the order of the bases for decryption
        .collect();

    // Validate bases
    if bases.iter().any(|&base| !validate_base(base, CHARSET)) {
        eprintln!("Error: Password file contains invalid bases.");
        return;
    }

    // Validate input string
    if !validate_input(&input_string, bases[0], CHARSET) {
        eprintln!(
            "Error: The string '{}' contains invalid characters for base {}.",
            input_string, bases[0]
        );
        return;
    }

    // Perform sequential conversions in reverse order
    let mut current_string = input_string;
    for &base_from in &bases {
        current_string = convert_base(&current_string, base_from, base_to, CHARSET);
    }

    // Write the final result to the output file
    if let Err(e) = fs::write(output_file, current_string) {
        eprintln!("Error writing to output file: {}", e);
    }
}
