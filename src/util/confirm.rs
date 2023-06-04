use std::io::{self, Write};

pub fn confirm(prompt: &str) -> bool {
    let mut input = String::new();
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_lowercase();

    input == "y" || input == "yes"
}