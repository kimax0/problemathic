use std::collections::{HashMap, HashSet};
use std::fs;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};
use clap::{Arg, Command};

const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

fn validate_base(base: usize) -> bool {
    (2..=CHARSET.len()).contains(&base)
}

fn validate_input(input_string: &str, base: usize) -> bool {
    let valid_chars: HashSet<_> = CHARSET.chars().take(base).collect();
    input_string.chars().all(|c| valid_chars.contains(&c))
}

fn convert_base(input_string: &str, base_from: usize, base_to: usize) -> String {
    let char_to_value: HashMap<_, _> = CHARSET.chars().enumerate().map(|(i, c)| (c, i)).collect();
    let value_to_char: Vec<_> = CHARSET.chars().collect();

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
        let remainder = (&decimal_value % &base_to_big).to_usize().unwrap();
        result.push(value_to_char[remainder]);
        decimal_value /= &base_to_big;
    }

    result.into_iter().rev().collect()
}

fn process_file(input_file: &str, output_file: &str, password_file: &str, encrypt: bool) {
    let input_string = match fs::read_to_string(input_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => return eprintln!("Error reading input file: {}", e),
    };

    let password_content = match fs::read_to_string(password_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => return eprintln!("Error reading password file: {}", e),
    };

    let mut bases: Vec<_> = password_content
        .split_whitespace()
        .filter_map(|b| b.parse::<usize>().ok())
        .collect();

    if !encrypt {
        bases.reverse();
    }

    if bases.iter().any(|&base| !validate_base(base)) {
        return eprintln!("Error: Password file contains invalid bases.");
    }

    if !validate_input(&input_string, CHARSET.len()) {
        return eprintln!("Error: Input string contains invalid characters.");
    }

    let mut current_string = input_string;
    for &base in &bases {
        current_string = if encrypt {
            convert_base(&current_string, CHARSET.len(), base)
        } else {
            convert_base(&current_string, base, CHARSET.len())
        };
    }

    if let Err(e) = fs::write(output_file, current_string) {
        eprintln!("Error writing to output file: {}", e);
    }
}

fn main() {
    let matches = Command::new("Problemathic")
        .version("1.0")
        .author("Mateja Nikolić <matejanikolic@proton.me>")
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

    if let Some((cmd, sub_matches)) = matches.subcommand() {
        let input = sub_matches.get_one::<String>("INPUT").unwrap();
        let output = sub_matches.get_one::<String>("OUTPUT").unwrap();
        let password = sub_matches.get_one::<String>("PASSWORD").unwrap();
        process_file(input, output, password, cmd == "encrypt");
    }
}
