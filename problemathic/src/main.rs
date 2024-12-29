use std::collections::{HashMap, HashSet};
use std::env;

// Validate if the base is within the acceptable range (2 to the length of the charset)
fn validate_base(base: usize, charset: &str) -> bool {
    (2..=charset.len()).contains(&base)
}

// Validate if the input is valid for the given base and charset
fn validate_input(input_string: &str, base: usize, charset: &str) -> bool {
    let valid_chars: HashSet<char> = charset.chars().take(base).collect();
    input_string.chars().all(|c| valid_chars.contains(&c))
}

// Convert the input from one base to another
fn convert_base(input_string: &str, base_from: usize, base_to: usize, charset: &str) -> String {
    let char_to_value: HashMap<char, usize> = charset.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let value_to_char: Vec<char> = charset.chars().collect();

    let decimal_value = input_string
        .chars()
        .fold(0usize, |acc, c| acc * base_from + char_to_value[&c]);

    let mut result = String::new();
    let mut value = decimal_value;

    while value > 0 {
        result.push(value_to_char[value % base_to]);
        value /= base_to;
    }

    if result.is_empty() {
        value_to_char[0].to_string()
    } else {
        result.chars().rev().collect()
    }
}

fn main() {
    // Define the charset as a constant within the code
    const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: nsco <input_string> <base_from> <base_to>");
        return;
    }

    let input_string = &args[1];
    let base_from = args[2].parse::<usize>();
    let base_to = args[3].parse::<usize>();

    match (base_from, base_to) {
        (Ok(base_from), Ok(base_to)) if validate_base(base_from, CHARSET) && validate_base(base_to, CHARSET) => {
            if validate_input(input_string, base_from, CHARSET) {
                println!(
                    "Input string: {}, Base from: {}, Base to: {}, Result: {}",
                    input_string,
                    base_from,
                    base_to,
                    convert_base(input_string, base_from, base_to, CHARSET)
                );
            } else {
                eprintln!(
                    "Error: The string '{}' contains invalid characters for base {}.",
                    input_string, base_from
                );
            }
        }
        _ => eprintln!(
            "Error: Bases must be integers between 2 and the length of the charset ({}).",
            CHARSET.len()
        ),
    }
}
