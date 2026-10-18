use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use std::io::{self, Write};

const ATTEMPT_LIMIT: u64 = 5_000_000;

// ターゲット単語とその目標値のペアを保持する構造体
struct TargetWord {
    word: String,
    value: u64,
}

#[derive(Parser, Debug)]
#[command(
    name = "cipher-finder-cli",
    version,
    about = "A command-line tool written in Rust for Gematria cipher discovery, which searches for an alphabet cipher matching a list of user-provided word-target pairs.",
    long_about = None
)]

struct Cli {
    // 引数を受け取るための配列
    #[arg(short, long, action = clap::ArgAction::Append)]
    targets: Vec<String>, 
}

// 単語が目標値に到達しているかチェック
fn check_word_target(word: &str, target_value: u64, cipher: &[u8]) -> bool {
    let mut sum: u64 = 0;
    
    for ch in word.chars() {
        let lower_ch = ch.to_ascii_lowercase();
        
        if lower_ch >= 'a' && lower_ch <= 'z' {
            let index = (lower_ch as u32 - 'a' as u32) as usize;

            if index < 26 {
                sum += cipher[index] as u64; 
            }
        }
    }
    sum == target_value
}

fn find_cipher(targets: &[TargetWord]) -> Option<Vec<u8>> {
    let mut rng = thread_rng();
    let mut attempt_count: u64 = 0;

    let start_time = Instant::now();

    while attempt_count < ATTEMPT_LIMIT {
        let mut permutation: Vec<u8> = (1..=26).collect();
        permutation.shuffle(&mut rng);

        let mut all_match = true;
        
        for target in targets {
            if !check_word_target(&target.word, target.value, &permutation) {
                all_match = false;
                break; 
            }
        }

        if all_match {
            println!("\n---------------------------------");
            println!("Matched cipher found! Total elapsed time: {:?}", start_time.elapsed());
            return Some(permutation);
        }

        attempt_count += 1;
        let percentage = (attempt_count as f64 / ATTEMPT_LIMIT as f64) * 100.0;
        
        let progress_display = format!("Progress: {:.2}% (Attempt: {})", percentage, attempt_count);
        print!("\r{}", progress_display);
        io::stdout().flush().unwrap();
    }

    println!("\n---------------------------------");
    println!("No solution found. Total time taken: {:?}", start_time.elapsed());
    None
}

// 暗号の配列を人間が読みやすい形式で表示する関数
fn format_cipher(cipher: &[u8]) -> String {
    let mut result = String::new();
    for (i, &value) in cipher.iter().enumerate() {
        let char_index = (i as u8 + b'a') as char;
        // ★ 引数の修正点: 閉じ括弧 ')' を適切に配置しました。
        result.push_str(&format!("{}({})", char_index, value));
        if i < 25 {
            result.push_str(", ");
        }
    }
    return result; // 関数の一番最後で String を返す
}

fn main() {
    let cli = Cli::parse();

    // 入力された単語と値のペアを収集する処理
    let mut targets: Vec<TargetWord> = Vec::new();

    // ユーザーが入力したすべての引数をループで処理
    for input_string in cli.targets {
        // スペースで分割し、それぞれの単語-値のペアを処理
        for part in input_string.split_whitespace() {
            
            // ハイフンで単語と値を分割
            if let Some((word_part, val_part)) = part.split_once('=') {
                let word = word_part.trim().to_string();
                
                // 値を解析
                let value = val_part.trim().parse::<u64>().unwrap_or(0);
                
                if !word.is_empty() {
                    // ターゲット構造体を生成してリストに保存
                    targets.push(TargetWord { word, value });
                }
            }
        }
    }

    // --- 出力表示の調整 ---
    let debug_output = targets.iter()
        .map(|t| format!("{}={}", t.word, t.value))
        .collect::<Vec<String>>()
        .join(" ");

    println!("Start search for a list of target pairs.");
    println!("Target pair: [{}]", debug_output);
    println!("---------------------------------");

    if targets.is_empty() {
        eprintln!("Error: No valid targets provided after parsing input.");
        return;
    }

    // 検索実行
    if let Some(cipher) = find_cipher(&targets) {
        println!("Found cipher: {}", format_cipher(&cipher));
    }
}
