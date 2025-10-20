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

  /// Use AI to fix the command (requires Google Gemini API key)
  #[arg(long, global = true)]
  ai: bool,
}

#[derive(Subcommand)]
enum Commands {
  /// Add a custom typo fix (alias: a)
  #[command(name = "add", alias = "a")]
  Add {
    /// The wrong command (typo)
    wrong: String,
    /// The correct command
    correct: String,
  },

  /// Remove a custom typo fix (alias: rm)
  #[command(name = "remove", alias = "rm")]
  Remove {
    /// The wrong command to remove
    wrong: String,
  },

  /// List all custom typos (alias: ls)
  #[command(name = "list", alias = "ls")]
  List,

  /// Clear all custom typos (alias: cls)
  #[command(name = "clear", alias = "cls")]
  Clear,

  /// Show config file location (alias: cfg)
  #[command(name = "config", alias = "cfg")]
  Config,

  /// Add the wrong command from history to custom fixes (alias: s)
  #[command(name = "save", alias = "s")]
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

  /// Add wtf to PATH environment variable (alias: i)
  #[command(name = "install", alias = "i")]
  Install,

  /// Remove wtf from PATH environment variable (alias: u)
  #[command(name = "uninstall", alias = "u")]
  Uninstall,

  /// Enable or disable auto-mode (auto-run first suggestion) (alias: am)
  #[command(name = "auto-mode", alias = "am")]
  AutoMode {
    /// Enable (true) or disable (false) auto-mode
    enabled: bool,
  },

  /// Toggle auto-mode on/off (alias: ta)
  #[command(name = "toggle-auto", alias = "ta")]
  ToggleAuto,

  /// Enable or disable AI mode (always use --ai flag) (alias: aim)
  #[command(name = "ai-mode", alias = "aim")]
  AiMode {
    /// Enable (true) or disable (false) AI mode
    enabled: bool,
  },

  /// Toggle AI mode on/off (alias: tai)
  #[command(name = "toggle-ai", alias = "tai")]
  ToggleAi,

  /// Configure bash history for real-time updates (Linux only) (alias: ch)
  #[command(name = "config-history", alias = "ch")]
  ConfigHistory,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let cli = Cli::parse();
  let mut user_config = UserConfig::load();

  if !user_config.first_run_complete && is_system_installed() {
    user_config.mark_first_run_complete();
    let _ = user_config.save();
  }

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
    Some(Commands::AiMode { enabled }) => {
      handle_ai_mode(&mut user_config, enabled);
    }
    Some(Commands::ToggleAi) => {
      handle_toggle_ai(&mut user_config);
    }
    Some(Commands::ConfigHistory) => {
      handle_config_history();
    }
    None => {
      let auto_yes = cli.yes || user_config.auto_mode;

      if cli.ai || user_config.ai_mode {
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

  #[cfg(not(target_os = "windows"))]
  check_and_configure_bash_history();
}

#[cfg(not(target_os = "windows"))]
fn should_configure_bash_history() -> bool {
  use std::env;
  use std::fs;

  let shell = env::var("SHELL").unwrap_or_default();
  if !shell.contains("bash") {
    return false;
  }

  let home = match dirs::home_dir() {
    Some(h) => h,
    None => return false,
  };

  let bashrc_path = home.join(".bashrc");
  if !bashrc_path.exists() {
    return false;
  }

  let bashrc_content = match fs::read_to_string(&bashrc_path) {
    Ok(content) => content,
    Err(_) => return false,
  };

  let has_histappend = bashrc_content.contains("shopt -s histappend");
  let has_prompt_command =
    bashrc_content.contains("PROMPT_COMMAND") && bashrc_content.contains("history -a");

  !(has_histappend && has_prompt_command)
}

#[cfg(not(target_os = "windows"))]
fn check_and_configure_bash_history() {
  if should_configure_bash_history() {
    configure_bash_history();
  }
}

#[cfg(not(target_os = "windows"))]
fn configure_bash_history() {
  use std::env;
  use std::fs;
  use std::io::{self, Write};

  let shell = env::var("SHELL").unwrap_or_default();
  if !shell.contains("bash") {
    println!();
    println!(
      "{}",
      "Bash history configuration is only for bash shell.".yellow()
    );
    println!();
    println!("{}", format!("Your current shell: {}", shell).dimmed());
    return;
  }

  let home = match dirs::home_dir() {
    Some(h) => h,
    None => {
      println!("{}", "Could not find home directory.".red());
      return;
    }
  };

  let bashrc_path = home.join(".bashrc");
  if !bashrc_path.exists() {
    println!();
    println!("{}", "~/.bashrc not found.".yellow());
    println!();
    println!(
      "{}",
      "Please create it first or configure manually.".dimmed()
    );
    return;
  }

  let bashrc_content = match fs::read_to_string(&bashrc_path) {
    Ok(content) => content,
    Err(e) => {
      println!("{}", format!("Failed to read .bashrc: {}", e).red());
      return;
    }
  };

  let has_histappend = bashrc_content.contains("shopt -s histappend");
  let has_prompt_command =
    bashrc_content.contains("PROMPT_COMMAND") && bashrc_content.contains("history -a");

  if has_histappend && has_prompt_command {
    println!();
    println!(
      "{} {}",
      "✓".bright_green(),
      "Bash history is already configured!".bright_green()
    );
    println!();
    println!(
      "{}",
      "Your .bashrc already has the required settings.".dimmed()
    );
    return;
  }

  println!();
  println!("{}", "📝 Bash History Configuration".bright_cyan().bold());
  println!();
  println!(
    "{}",
    "For real-time history updates, we need to configure bash.".bright_white()
  );
  println!("This will allow WTF to see your most recent commands.");
  println!();
  println!("{}", "Configuration to add:".dimmed());
  println!("  shopt -s histappend");
  println!("  PROMPT_COMMAND='history -a'");
  println!();
  print!("{} [Y/n]: ", "Configure bash history now?".bright_cyan());
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let answer = input.trim().to_lowercase();

  if answer.is_empty() || answer == "y" || answer == "yes" {
    let mut new_content = bashrc_content.clone();

    new_content.push_str("\n\n");
    new_content.push_str("# WTF - Command Typo Fixer: Enable real-time history\n");

    if !has_histappend {
      new_content.push_str("shopt -s histappend\n");
    }

    if !has_prompt_command {
      new_content.push_str("PROMPT_COMMAND='history -a'\n");
    }

    match fs::write(&bashrc_path, new_content) {
      Ok(_) => {
        println!();
        println!(
          "{} {}",
          "✓".bright_green(),
          "Bash configuration updated!".bright_green()
        );
        println!();
        println!("{}", "Run this to apply changes:".bright_cyan());
        println!("  source ~/.bashrc");
        println!();
      }
      Err(e) => {
        eprintln!();
        eprintln!("{}", format!("Failed to update .bashrc: {}", e).red());
        eprintln!();
        eprintln!(
          "{}",
          "You can manually add these lines to ~/.bashrc:".yellow()
        );
        eprintln!("  shopt -s histappend");
        eprintln!("  PROMPT_COMMAND='history -a'");
        println!();
      }
    }
  } else {
    println!();
    println!("{}", "Skipped bash configuration.".yellow());
    println!();
    println!(
      "{}",
      "You can manually add these to ~/.bashrc:".bright_cyan()
    );
    println!("  shopt -s histappend");
    println!("  PROMPT_COMMAND='history -a'");
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
  let builtin_fixes = commands::get_common_fixes();
  let is_builtin = builtin_fixes
    .iter()
    .any(|(typo, fix)| *typo == wrong || fix.0 == correct);

  if is_builtin {
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

fn handle_ai_mode(config: &mut UserConfig, enabled: bool) {
  config.set_ai_mode(enabled);

  if let Err(e) = config.save() {
    display_error(&format!("Failed to save config: {}", e));
    std::process::exit(1);
  }

  if enabled {
    println!(
      "{} {}",
      "✓".bright_green(),
      "AI mode enabled!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now use AI for command fixing (requires Google Gemini API key).".bright_cyan()
    );
    println!();
    println!(
      "{}",
      "This is equivalent to always using 'wtf --ai'".dimmed()
    );
  } else {
    println!(
      "{} {}",
      "✓".bright_green(),
      "AI mode disabled!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now use pattern matching for command fixing.".bright_cyan()
    );
  }
}

fn handle_toggle_ai(config: &mut UserConfig) {
  let new_state = config.toggle_ai_mode();

  if let Err(e) = config.save() {
    display_error(&format!("Failed to save config: {}", e));
    std::process::exit(1);
  }

  if new_state {
    println!(
      "{} {}",
      "✓".bright_green(),
      "AI mode toggled ON!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now use AI for command fixing.".bright_cyan()
    );
  } else {
    println!(
      "{} {}",
      "✓".bright_green(),
      "AI mode toggled OFF!".bright_green()
    );
    println!();
    println!(
      "{}",
      "wtf will now use pattern matching for command fixing.".bright_cyan()
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
      println!();

      #[cfg(not(target_os = "windows"))]
      {
        if should_configure_bash_history() {
          handle_config_history();
        }
      }
    }
    Err(e) => {
      display_error(&e);
      std::process::exit(1);
    }
  }
}

fn handle_config_history() {
  #[cfg(target_os = "windows")]
  {
    println!(
      "{}",
      "This command is only available on Linux/Unix systems.".yellow()
    );
    println!();
    println!(
      "{}",
      "Bash history configuration is automatic on Windows PowerShell.".dimmed()
    );
    return;
  }

  #[cfg(not(target_os = "windows"))]
  configure_bash_history();
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
