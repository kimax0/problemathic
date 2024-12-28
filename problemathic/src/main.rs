use std::collections::HashSet;
use std::env;

// Validate if the base is within the acceptable range (2 to 96)
fn validate_base(base: u8) -> bool {
    (2..=96).contains(&base)
}

// Validate if the number is valid for the given base
fn validate_number(num: &str, base: u8) -> bool {
    let valid_chars: HashSet<char> = (0..base).map(|i| (32 + i) as char).collect();
    num.chars().all(|c| valid_chars.contains(&c))
}

// Convert a number from one base to another
fn convert_base(number: &str, base_from: u8, base_to: u8) -> String {
    let decimal_value = number.chars().fold(0u64, |acc, c| acc * base_from as u64 + (c as u64 - 32));
    let mut result = String::new();
    let mut value = decimal_value;

    while value > 0 {
        result.push((32 + (value % base_to as u64) as u8) as char);
        value /= base_to as u64;
    }

    if result.is_empty() { " ".to_string() } else { result.chars().rev().collect() }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: nsco <number> <base_from> <base_to>");
        return;
    }

    let (num, base_from, base_to) = (&args[1], args[2].parse::<u8>(), args[3].parse::<u8>());

    match (base_from, base_to) {
        (Ok(base_from), Ok(base_to)) if validate_base(base_from) && validate_base(base_to) => {
            if validate_number(num, base_from) {
                println!(
                    "Number: {}, Base from: {}, Base to: {}, Result: {}",
                    num,
                    base_from,
                    base_to,
                    convert_base(num, base_from, base_to)
                );
            } else {
                eprintln!("Error: The number '{}' contains invalid characters for base {}.", num, base_from);
            }
        }
        _ => eprintln!("Error: Bases must be integers between 2 and 96."),
    }
}
