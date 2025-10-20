mod ai;
mod commands;
mod config;
mod corrections;
mod executor;
mod history;
mod path;
mod ui;

use clap::{Parser, Subcommand};
use colored::Colorize;
use config::UserConfig;
use corrections::find_corrections;
use executor::execute_command;
use history::get_last_command;
use ui::*;

#[derive(Parser)]
#[command(name = "wtf")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Fix typos in your previous command", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,

  /// Force run without confirmation
  #[arg(short, long, global = true)]
  yes: bool,

  /// Show debug information
  #[arg(short, long, global = true)]
  debug: bool,

  /// Use AI to fix the command (requires OpenAI API key)
  #[arg(long, global = true)]
  ai: bool,
}

#[derive(Subcommand)]
enum Commands {
  /// Add a custom typo fix
  #[command(name = "add")]
  Add {
    /// The wrong command (typo)
    wrong: String,
    /// The correct command
    correct: String,
  },

  /// Remove a custom typo fix
  #[command(name = "remove")]
  Remove {
    /// The wrong command to remove
    wrong: String,
  },

  /// List all custom typos
  #[command(name = "list")]
  List,

  /// Clear all custom typos
  #[command(name = "clear")]
  Clear,

  /// Show config file location
  #[command(name = "config")]
  Config,

  /// Add the wrong command from history to custom fixes
  #[command(name = "save")]
  Save {
    /// The correct command
    correct: String,
  },

  /// Set Google AI API key for AI-powered fixing
  #[command(name = "set-api-key")]
  SetApiKey {
    /// Your Google AI API key
    api_key: String,
  },

  /// Add wtf to PATH environment variable
  #[command(name = "install", alias = "i")]
  Install,

  /// Remove wtf from PATH environment variable
  #[command(name = "uninstall", alias = "u")]
  Uninstall,

  /// Enable or disable auto-mode (auto-run first suggestion)
  #[command(name = "auto-mode")]
  AutoMode {
    /// Enable (true) or disable (false) auto-mode
    enabled: bool,
  },

  /// Toggle auto-mode on/off
  #[command(name = "toggle-auto")]
  ToggleAuto,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let cli = Cli::parse();
  let mut user_config = UserConfig::load();

  // Auto-complete first run if installed via system package manager
  if !user_config.first_run_complete && is_system_installed() {
    user_config.mark_first_run_complete();
    let _ = user_config.save();
  }

  // Check for first-run and prompt for installation
  if !user_config.first_run_complete && cli.command.is_none() {
    handle_first_run_prompt(&mut user_config);
  }

  match cli.command {
    Some(Commands::Add { wrong, correct }) => {
      handle_add(&mut user_config, wrong, correct);
    }
    Some(Commands::Remove { wrong }) => {
      handle_remove(&mut user_config, wrong);
    }
    Some(Commands::List) => {
      handle_list(&user_config);
    }
    Some(Commands::Clear) => {
      handle_clear(&mut user_config);
    }
    Some(Commands::Config) => {
      handle_config();
    }
    Some(Commands::Save { correct }) => {
      handle_save(&mut user_config, correct, cli.debug);
    }
    Some(Commands::SetApiKey { api_key }) => {
      handle_set_api_key(api_key);
    }
    Some(Commands::Install) => {
      handle_install();
    }
    Some(Commands::Uninstall) => {
      handle_uninstall();
    }
    Some(Commands::AutoMode { enabled }) => {
      handle_auto_mode(&mut user_config, enabled);
    }
    Some(Commands::ToggleAuto) => {
      handle_toggle_auto(&mut user_config);
    }
    None => {
      // Use -y flag or auto_mode config
      let auto_yes = cli.yes || user_config.auto_mode;

      if cli.ai {
        handle_ai_fix(auto_yes, cli.debug).await;
      } else {
        handle_fix(auto_yes, cli.debug, &user_config);
      }
    }
  }
}

fn is_system_installed() -> bool {
  use std::env;

  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_str) = exe_path.to_str() {
      // Check if installed via system package manager
      let system_paths = ["/usr/local/bin", "/usr/bin", "/opt", "/bin"];

      return system_paths.iter().any(|path| exe_str.starts_with(path));
    }
  }

  false
}

fn handle_first_run_prompt(config: &mut UserConfig) {
  use std::io::{self, Write};

  println!();
  println!(
    "{}",
    "🎉 Welcome to WTF - Command Typo Fixer!"
      .bright_cyan()
      .bold()
  );
  println!();
  println!(
    "{}",
    "Would you like to install WTF globally to your PATH?".bright_white()
  );
  println!("This will allow you to run 'wtf' from anywhere.");
  println!();
  println!(
    "{}",
    "  • You can run 'wtf install' later to install".dimmed()
  );
  println!(
    "{}",
    "  • You can run 'wtf uninstall' to remove it".dimmed()
  );
  println!();
  print!("{} [Y/n]: ", "Install globally?".bright_cyan());
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let answer = input.trim().to_lowercase();

  config.mark_first_run_complete();
  if let Err(e) = config.save() {
    eprintln!(
      "{}",
      format!("Warning: Failed to save config: {}", e).yellow()
    );
  }

  if answer.is_empty() || answer == "y" || answer == "yes" {
    println!();
    println!("{}", "Installing WTF to PATH...".bright_cyan());
    println!();

    match path::add_to_path() {
      Ok(_) => {
        println!();
        println!(
          "{} {}",
          "✓".bright_green(),
          "Installation complete!".bright_green()
        );
        println!();
        println!("{}", "You can now use 'wtf' from anywhere!".bright_cyan());
        println!();
        println!(
          "{}",
          "💡 Tip: Restart your terminal for PATH changes to take effect".yellow()
        );
        println!();
      }
      Err(e) => {
        eprintln!();
        eprintln!("{}", format!("Installation failed: {}", e).red());
        eprintln!();
        eprintln!("{}", "You can try again later with:".yellow());
        eprintln!("  wtf install");
        println!();
      }
    }
  } else {
    println!();
    println!("{}", "Skipped installation.".yellow());
    println!();
    println!("{}", "You can install later by running:".bright_cyan());
    println!("  wtf install");
    println!();
  }
}

fn handle_fix(auto_yes: bool, debug: bool, user_config: &UserConfig) {
  match get_last_command() {
    Ok(last_cmd) => {
      if debug {
        println!("Last command: {}", last_cmd);
      }

      match find_corrections(&last_cmd, user_config) {
        Some(corrections) => {
          display_corrections(&last_cmd, &corrections);

          let selected = if auto_yes {
            0
          } else {
            match prompt_selection(corrections.len()) {
              Some(idx) => idx,
              None => {
                println!("{}", "Cancelled.".yellow());
                return;
              }
            }
          };

          let cmd_to_run = &corrections[selected].fixed_cmd;
          display_success(cmd_to_run);

          if let Err(e) = execute_command(cmd_to_run) {
            display_error(&e);
            std::process::exit(1);
          }
        }
        None => {
          display_no_suggestions(&last_cmd);
        }
      }
    }
    Err(e) => {
      display_error(&e);
      std::process::exit(1);
    }
  }
}

fn handle_add(config: &mut UserConfig, wrong: String, correct: String) {
  // Check if it's in built-in fixes
  let builtin_fixes = commands::get_common_fixes();
  let is_builtin = builtin_fixes
    .iter()
    .any(|(typo, fix)| *typo == wrong || fix.0 == correct);

  if is_builtin {
    // If built-in has a fix for this typo, just add the typo to custom
    config.add_from_builtin(wrong.clone(), correct.clone());
    display_info("ℹ This typo is already in built-in database, adding to your custom list.");
  } else {
    config.add_typo(wrong.clone(), correct.clone());
  }

  if let Err(e) = config.save() {
    display_error(&format!("Failed to save config: {}", e));
    std::process::exit(1);
  }

  display_added(&wrong, &correct);
}

fn handle_remove(config: &mut UserConfig, wrong: String) {
  if config.remove_typo(&wrong) {
    if let Err(e) = config.save() {
      display_error(&format!("Failed to save config: {}", e));
      std::process::exit(1);
    }
    display_removed(&wrong);
  } else {
    display_error(&format!("Typo '{}' not found in custom list", wrong));
    std::process::exit(1);
  }
}

fn handle_list(config: &UserConfig) {
  display_custom_typos(&config.custom_typos);
}

fn handle_clear(config: &mut UserConfig) {
  let count = config.custom_typos.len();
  config.custom_typos.clear();

  if let Err(e) = config.save() {
    display_error(&format!("Failed to save config: {}", e));
    std::process::exit(1);
  }

  println!("{} Cleared {} custom typo(s)", "✓".bright_green(), count);
}

fn handle_config() {
  println!("{}", "Config file location:".bright_cyan());
  println!("  {}", UserConfig::get_config_path_display().bright_white());
}

fn handle_auto_mode(config: &mut UserConfig, enabled: bool) {
  config.set_auto_mode(enabled);

  if let Err(e) = config.save() {
    display_error(&format!("Failed to save config: {}", e));
    std::process::exit(1);
  }

  if enabled {
    println!(
      "{} {}",
      "✓".bright_green(),
      "Auto-mode enabled!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now automatically run the first suggestion without prompting.".bright_cyan()
    );
    println!();
    println!("{}", "This is equivalent to always using 'wtf -y'".dimmed());
  } else {
    println!(
      "{} {}",
      "✓".bright_green(),
      "Auto-mode disabled!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now prompt before running suggestions.".bright_cyan()
    );
  }
}

fn handle_toggle_auto(config: &mut UserConfig) {
  let new_state = config.toggle_auto_mode();

  if let Err(e) = config.save() {
    display_error(&format!("Failed to save config: {}", e));
    std::process::exit(1);
  }

  if new_state {
    println!(
      "{} {}",
      "✓".bright_green(),
      "Auto-mode toggled ON!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now automatically run the first suggestion.".bright_cyan()
    );
  } else {
    println!(
      "{} {}",
      "✓".bright_green(),
      "Auto-mode toggled OFF!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now prompt before running suggestions.".bright_cyan()
    );
  }
}

fn handle_save(config: &mut UserConfig, correct: String, debug: bool) {
  match get_last_command() {
    Ok(last_cmd) => {
      if debug {
        println!("Last command: {}", last_cmd);
      }

      config.add_typo(last_cmd.clone(), correct.clone());

      if let Err(e) = config.save() {
        display_error(&format!("Failed to save config: {}", e));
        std::process::exit(1);
      }

      display_added(&last_cmd, &correct);
      println!();
      println!(
        "{}",
        "Now you can use 'wtf' to fix this typo in the future!".bright_cyan()
      );
    }
    Err(e) => {
      display_error(&e);
      std::process::exit(1);
    }
  }
}

fn handle_set_api_key(api_key: String) {
  match ai::save_api_key(api_key) {
    Ok(_) => {
      println!(
        "{} {}",
        "✓".bright_green(),
        "Google AI API key saved successfully!".bright_green()
      );
      println!();
      println!(
        "{}",
        "You can now use AI-powered fixing with:".bright_cyan()
      );
      println!("  wtf --ai");
      println!();
      println!(
        "{}",
        "💡 Tip: The API key is stored in your config directory".dimmed()
      );
    }
    Err(e) => {
      display_error(&format!("Failed to save API key: {}", e));
      std::process::exit(1);
    }
  }
}

fn handle_install() {
  println!("{}", "Installing WTF to PATH...".bright_cyan());
  println!();

  match path::add_to_path() {
    Ok(_) => {
      println!();
      println!(
        "{} {}",
        "✓".bright_green(),
        "Installation complete!".bright_green()
      );
      println!();
      println!("{}", "You can now use 'wtf' from anywhere!".bright_cyan());
    }
    Err(e) => {
      display_error(&e);
      std::process::exit(1);
    }
  }
}

fn handle_uninstall() {
  println!("{}", "Removing WTF from PATH...".bright_cyan());
  println!();

  match path::remove_from_path() {
    Ok(_) => {
      println!();
      println!(
        "{} {}",
        "✓".bright_green(),
        "Uninstallation complete!".bright_green()
      );
    }
    Err(e) => {
      display_error(&e);
      std::process::exit(1);
    }
  }
}

async fn handle_ai_fix(auto_yes: bool, debug: bool) {
  // Check API key first
  if let Err(_) = ai::check_api_key() {
    ai::display_api_key_help();
    std::process::exit(1);
  }

  match get_last_command() {
    Ok(last_cmd) => {
      if debug {
        println!("Last command: {}", last_cmd);
      }

      display_corrections(&last_cmd, &[]);

      match ai::fix_command_with_ai(&last_cmd).await {
        Ok(fixed_cmd) => {
          println!();
          println!(
            "{} {} {}",
            "🤖".bright_cyan(),
            "AI suggestion:".bright_green(),
            fixed_cmd.bright_white().bold()
          );
          println!();

          let should_run = if auto_yes {
            true
          } else {
            print!("{} [Y/n]: ", "Run this command?".bright_cyan());
            use std::io::{self, Write};
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            let answer = input.trim().to_lowercase();
            answer.is_empty() || answer == "y" || answer == "yes"
          };

          if should_run {
            display_success(&fixed_cmd);
            if let Err(e) = execute_command(&fixed_cmd) {
              display_error(&e);
              std::process::exit(1);
            }
          } else {
            println!("{}", "Cancelled.".yellow());
          }
        }
        Err(e) => {
          display_error(&format!("AI fix failed: {}", e));
          println!();
          println!(
            "{}",
            "💡 Tip: Falling back to built-in typo detection...".yellow()
          );
          println!();

          // Fall back to regular fix
          let user_config = UserConfig::load();
          handle_fix(auto_yes, debug, &user_config);
        }
      }
    }
    Err(e) => {
      display_error(&e);
      std::process::exit(1);
    }
  }
}
