use std::collections::{HashMap, HashSet};
use std::env;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive}; // Import ToPrimitive for `to_usize`

fn validate_base(base: usize, charset: &str) -> bool {
    (2..=charset.len()).contains(&base)
}

fn validate_input(input_string: &str, base: usize, charset: &str) -> bool {
    let valid_chars: HashSet<char> = charset.chars().take(base).collect();
    input_string.chars().all(|c| valid_chars.contains(&c))
}

fn convert_base(input_string: &str, base_from: usize, base_to: usize, charset: &str) -> String {
    let char_to_value: HashMap<char, usize> = charset.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let value_to_char: Vec<char> = charset.chars().collect();

    // Convert input to decimal (BigUint)
    let mut decimal_value = BigUint::zero();
    let base_from_big = BigUint::from(base_from);

    for c in input_string.chars() {
        decimal_value = decimal_value * &base_from_big + BigUint::from(char_to_value[&c]);
    }

    // Convert decimal to target base (BigUint)
    if decimal_value.is_zero() {
        return value_to_char[0].to_string();
    }

    let mut result = String::new();
    let base_to_big = BigUint::from(base_to);

    while !decimal_value.is_zero() {
        let remainder = (&decimal_value % &base_to_big)
            .to_usize()
            .expect("Value too large for conversion");
        result.push(value_to_char[remainder]);
        decimal_value /= &base_to_big;
    }

    result.chars().rev().collect()
}

fn main() {
    const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:;<=>?@[\\]^_{|}~";

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
