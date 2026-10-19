use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use std::io::{self, Write};

/// Multiplier for the actual number of attempts (1 = 1 million attempts)
const DEFAULT_ATTEMPTS_MULTIPLIER: u64 = 1;
const ATTEMPTS_MULTIPLIER_SCALE: u64 = 1_000_000;

// Target structure holding word and its required sum value
struct TargetWord {
    word: String,
    value: u64,
}

#[derive(Parser, Debug)]
#[command(
    name = "cipher-finder-cli",
    version,
    about = "A command-line tool for gematria cipher discovery, searching for an alphabet cipher matching target word-value pairs.",
    verbatim_doc_comment,
    next_line_help = true,
    arg_required_else_help = true,
)]
struct Cli {
	#[arg(
        short,
        long,
        action = clap::ArgAction::Append,
        help = "Arguments for target word-value pairs\n(e.g., 'apple=50, banana=33)"
    )]
    targets: Vec<String>,

	#[arg(
        short,
        long,
        requires = "targets",
        default_value_t = DEFAULT_ATTEMPTS_MULTIPLIER,
        help = "Multiplier for the actual number of attempts.\nExample: 3 results in 3,000,000 attempts.\n"
    )]
    attempts_multiplier: u64,
}

// Check if a word matches the target value using the provided cipher
fn check_word_target(word: &str, target_value: u64, cipher: &[u8]) -> bool {
    let mut sum: u64 = 0;

    for ch in word.chars() {
        let lower_ch = ch.to_ascii_lowercase();

        if lower_ch >= 'a' && lower_ch <= 'z' {
            let index = (lower_ch as u32 - 'a' as u32) as usize;

            if index < 26 {
                // Map character index to cipher byte value and accumulate sum.
                sum += cipher[index] as u64;
            }
        }
    }
    sum == target_value
}

/// Finds a matching cipher by randomly shuffling permutations and testing against targets.
fn find_cipher(targets: &[TargetWord], actual_attempt_limit: u64) -> Option<Vec<u8>> {
    let mut rng = thread_rng();
    let mut attempt_count: u64 = 0;
    let start_time = Instant::now();

    while attempt_count < actual_attempt_limit {
        // Generate a random permutation of 1-26 for the cipher values.
        let mut permutation: Vec<u8> = (1..=26).collect();
        permutation.shuffle(&mut rng);

        let mut all_match = true;

        // Test the permutation against all provided targets.
        for target in targets {
            if !check_word_target(&target.word, target.value, &permutation) {
                all_match = false;
                break;
            }
        }

        if all_match {
            let duration = start_time.elapsed();
            println!("\n---------------------------------");
            // Display elapsed time upon success.
            println!("Matched cipher found (Total elapsed time: {:.2}s):", duration.as_secs_f64());
            return Some(permutation);
        }

        attempt_count += 1;
        let percentage = (attempt_count as f64 / actual_attempt_limit as f64) * 100.0;

        let progress_display = format!("Progress: {:.2}% (Attempt: {} / {})", percentage, attempt_count, actual_attempt_limit);
        print!("\r{}", progress_display);
        io::stdout().flush().unwrap();
    }

    let duration = start_time.elapsed();
    println!("\n---------------------------------");
    // Display elapsed time upon failure.
    println!("No solution found. Total time taken: {:.2}s", duration.as_secs_f64());
    None
}

/// Formats the cipher array into a readable string like 'a(1), b(2), ...'
fn format_cipher(cipher: &[u8]) -> String {
    let mut result = String::new();
    for (i, &value) in cipher.iter().enumerate() {
        let char_index = (i as u8 + b'a') as char;
        // Construct the formatted pair (char(value)).
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

    // Process input strings into TargetWord structs.
    for input_string in cli.targets {
        for part in input_string.split_whitespace() {
            if let Some((word_part, val_part)) = part.split_once('=') {
                let word = word_part.trim().to_string();
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

    // Calculate the actual attempt limit based on the multiplier.
    let actual_limit = cli.attempts_multiplier.checked_mul(ATTEMPTS_MULTIPLIER_SCALE).expect("Attempt limit overflow");
    
    println!("Start search for target pairs:\n{}", debug_output);
    println!("Total attempts: {}\n---------------------------------", actual_limit);

    if let Some(cipher) = find_cipher(&targets, actual_limit) {
        println!("{}", format_cipher(&cipher));
    }
}
