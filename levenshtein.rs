use std::env;
use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (string1, string2) = match args.len() {
        1 => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let inputs: Vec<&str> = input.trim_end().split_whitespace().collect();
            if inputs.len() != 2 {
                eprintln!("Please provide two strings");
                std::process::exit(1);
            }
            (inputs[0].to_string(), inputs[1].to_string())
        }
        2 => {
            let string1 = args[1].clone();
            let mut string2 = String::new();
            io::stdin().read_line(&mut string2).expect("Failed to read input");
            (string1, string2.trim_end().to_string())
        }
        3 => (args[1].clone(), args[2].clone()),
        _ => {
            eprintln!("Usage: {} [string1] [string2]", args[0]);
            std::process::exit(1);
        }
    };

    let distance = levenshtein_distance(&string1, &string2);
    println!("{}", distance);
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

    for i in 0..=s1_len {
        matrix[i][0] = i;
    }

    for j in 0..=s2_len {
        matrix[0][j] = j;
    }

    for (i, s1_char) in s1.chars().enumerate() {
        for (j, s2_char) in s2.chars().enumerate() {
            let substitution_cost = if s1_char == s2_char { 0 } else { 1 };

            matrix[i + 1][j + 1] = cmp::min(
                cmp::min(
                    matrix[i][j + 1] + 1, // deletion
                    matrix[i + 1][j] + 1, // insertion
                ),
                matrix[i][j] + substitution_cost, // substitution
            );
        }
    }

    matrix[s1_len][s2_len]
}

