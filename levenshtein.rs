use std::io;
use std::cmp;

fn main() {
    // have to use mut and constructor because its kinda like c++ 
    let mut string1 = String::new();
    let mut string2 = String::new();

    // standing input from io i
    println!("Enter the first string:");
    io::stdin().read_line(&mut string1).expect("Failed to read input");
    let string1 = string1.trim_end();

    println!("Enter the second string:");
    io::stdin().read_line(&mut string2).expect("Failed to read input");
    let string2 = string2.trim_end();

    let distance = levenshtein_distance(string1, string2);
    println!("The Levenshtein distance between the two strings is: {}", distance);
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

