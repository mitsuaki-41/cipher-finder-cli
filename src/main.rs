use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use std::io::{self, Write};

const ATTEMPT_LIMIT: u64 = 5_000_000;

// Target structure holding word and its required sum value
struct TargetWord {
    word: String,
    value: u64,
}

#[derive(Parser, Debug)]
#[command(
    name = "cipher-finder-cli",
    version,
    about = "A command-line tool for gematria cipher discovery, searching for an alphabet cipher matching target word-value pairs."
)]
struct Cli {
    // Arguments for target word-value pairs
    #[arg(short, long, action = clap::ArgAction::Append)]
    targets: Vec<String>, 
}

// Check if a word matches the target value using the provided cipher
fn check_word_target(word: &str, target_value: u64, cipher: &[u8]) -> bool {
    let mut sum: u64 = 0;
    
    for ch in word.chars() {
        let lower_ch = ch.to_ascii_lowercase();
        
        if lower_ch >= 'a' && lower_ch <= 'z' {
            let index = (lower_ch as u32 - 'a' as u32) as usize;

            if index < 26 {
                // Map the character's index to the cipher byte value
                sum += cipher[index] as u64; 
            }
        }
    }
    sum == target_value
}

/// Finds a matching cipher by randomly shuffling permutations and testing against targets
fn find_cipher(targets: &[TargetWord]) -> Option<Vec<u8>> {
    let mut rng = thread_rng();
    let mut attempt_count: u64 = 0;
    let start_time = Instant::now();

    while attempt_count < ATTEMPT_LIMIT {
        // Generate a random permutation of 1-26 for the cipher values
        let mut permutation: Vec<u8> = (1..=26).collect();
        permutation.shuffle(&mut rng);

        let mut all_match = true;
        
        // Check permutation against all provided target words
        for target in targets {
            if !check_word_target(&target.word, target.value, &permutation) {
                all_match = false;
                break; 
            }
        }

        if all_match {
            let duration = start_time.elapsed();
            println!("\n---------------------------------");
            // Display total elapsed time formatted to 2 decimal places (seconds)
            println!("Matched cipher found (Total elapsed time: {:.2}s):", duration.as_secs_f64());
            return Some(permutation);
        }

        attempt_count += 1;
        let percentage = (attempt_count as f64 / ATTEMPT_LIMIT as f64) * 100.0;
        
        let progress_display = format!("Progress: {:.2}% (Attempt: {})", percentage, attempt_count);
        print!("\r{}", progress_display);
        io::stdout().flush().unwrap();
    }

    let duration = start_time.elapsed();
    println!("\n---------------------------------");
    // Display total elapsed time formatted to 2 decimal places (seconds)
    println!("No solution found. Total time taken: {:.2}s", duration.as_secs_f64());
    None
}

/// Formats the cipher array into a readable string like 'a(1), b(2), ...'
fn format_cipher(cipher: &[u8]) -> String {
    let mut result = String::new();
    for (i, &value) in cipher.iter().enumerate() {
        let char_index = (i as u8 + b'a') as char;
        // Corrected: added closing parenthesis for format! macro call
        result.push_str(&format!("{}({})", char_index, value));
        if i < 25 {
            result.push_str(", ");
        }
    }
    result
}

fn main() {
    let cli = Cli::parse();

    let mut targets: Vec<TargetWord> = Vec::new();

    for input_string in cli.targets {
        // Split by whitespace and then by '='
        for part in input_string.split_whitespace() {
            if let Some((word_part, val_part)) = part.split_once('=') {
                let word = word_part.trim().to_string();
                
                // Parse the value
                let value = val_part.trim().parse::<u64>().unwrap_or(0);
                
                if !word.is_empty() {
                    targets.push(TargetWord { word, value });
                }
            }
        }
    }

    let debug_output = targets.iter()
        .map(|t| format!("{}={}", t.word, t.value))
        .collect::<Vec<String>>()
        .join(" ");

    println!("Start search for a list of target pairs:\n{}", debug_output);
    println!("---------------------------------");

    if targets.is_empty() {
        eprintln!("Error: No valid targets provided after parsing input.");
        return;
    }

    if let Some(cipher) = find_cipher(&targets) {
        println!("{}", format_cipher(&cipher));
    }
}
