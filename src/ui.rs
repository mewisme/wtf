use crate::corrections::Correction;
use colored::*;
use std::io::{self, Write};

pub fn display_corrections(last_cmd: &str, corrections: &[Correction]) {
    println!("{}", "Previous command:".bright_red());
    println!("  {}", last_cmd.bright_yellow());
    println!();

    for (i, correction) in corrections.iter().enumerate() {
        println!(
            "{} {} {} {}",
            format!("[{}]", i + 1).bright_cyan(),
            "Suggested fix:".bright_green(),
            correction.fixed_cmd.bright_white().bold(),
            format!("({})", correction.reason).dimmed()
        );
    }
    println!();
}

pub fn display_no_suggestions(last_cmd: &str) {
    println!(
        "{} No suggestions found for: {}",
        "¯\\_(ツ)_/¯".bright_yellow(),
        last_cmd.bright_white()
    );
    println!(
        "{}",
        "The command might be correct or too complex to fix automatically.".dimmed()
    );
    println!();
    println!("{}", "💡 Tip: Add your own fix with:".bright_cyan());
    println!(
        "  {} \"{}\" \"<correct_command>\"",
        "wtf --add".bright_white(),
        last_cmd.dimmed()
    );
}

pub fn prompt_selection(max: usize) -> Option<usize> {
    print!(
        "{} [1-{}] (or 'n' to cancel): ",
        "Select a fix".bright_cyan(),
        max
    );
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let trimmed = input.trim().to_lowercase();

    if trimmed == "n" || trimmed == "no" {
        return None;
    }

    if let Ok(num) = trimmed.parse::<usize>() {
        if num > 0 && num <= max {
            return Some(num - 1);
        }
    }

    // Default to first option if just Enter is pressed
    if trimmed.is_empty() && max > 0 {
        return Some(0);
    }

    None
}

pub fn display_success(cmd: &str) {
    println!(
        "{} {}",
        "Running:".bright_green().bold(),
        cmd.bright_white()
    );
    println!();
}

pub fn display_error(msg: &str) {
    eprintln!("{} {}", "Error:".bright_red(), msg);
}

pub fn display_custom_typos(typos: &[(String, String)]) {
    if typos.is_empty() {
        println!("{}", "No custom typos configured.".yellow());
        println!();
        println!("{}", "Add one with:".dimmed());
        println!("  wtf --add \"wrong_cmd\" \"correct_cmd\"");
        return;
    }

    println!("{}", "Custom Typos:".bright_cyan().bold());
    println!();

    for (i, (wrong, correct)) in typos.iter().enumerate() {
        println!(
            "{} {} {} {}",
            format!("[{}]", i + 1).bright_black(),
            wrong.bright_yellow(),
            "→".bright_white(),
            correct.bright_green()
        );
    }
    println!();
    println!("{} custom typo(s)", typos.len());
}

pub fn display_added(wrong: &str, correct: &str) {
    println!(
        "{} {} {} {}",
        "✓".bright_green(),
        "Added:".bright_green(),
        wrong.bright_yellow(),
        format!("→ {}", correct).bright_white()
    );
}

pub fn display_removed(wrong: &str) {
    println!(
        "{} {} {}",
        "✓".bright_green(),
        "Removed:".bright_green(),
        wrong.bright_yellow()
    );
}

pub fn display_info(msg: &str) {
    println!("{}", msg.bright_cyan());
}
