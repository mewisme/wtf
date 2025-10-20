use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ShellType {
  PowerShell,
  Bash,
  Zsh,
  Fish,
}

pub fn get_last_command() -> Result<String, String> {
  let history_path = get_history_path()?;

  if !history_path.exists() {
    return Err(format!("History file not found: {:?}", history_path));
  }

  let content =
    fs::read_to_string(&history_path).map_err(|e| format!("Failed to read history: {}", e))?;

  let shell_type = detect_shell_type(&history_path);

  let result = match shell_type {
    ShellType::PowerShell => parse_powershell_history(&content),
    ShellType::Bash | ShellType::Zsh => parse_bash_zsh_history(&content),
    ShellType::Fish => parse_fish_history(&content),
  };

  if result.is_err() && matches!(shell_type, ShellType::Bash) {
    return Err(
      "History file is not up to date. Add this to your ~/.bashrc:\n\
             shopt -s histappend\n\
             PROMPT_COMMAND='history -a'"
        .to_string(),
    );
  }

  result
}

fn detect_shell_type(path: &PathBuf) -> ShellType {
  let path_str = path.to_string_lossy().to_lowercase();

  if path_str.contains("powershell") || path_str.contains("consolehost_history") {
    ShellType::PowerShell
  } else if path_str.contains("fish") {
    ShellType::Fish
  } else if path_str.contains("zsh") {
    ShellType::Zsh
  } else {
    ShellType::Bash
  }
}

fn get_history_path() -> Result<PathBuf, String> {
  if cfg!(target_os = "windows") {
    if let Ok(appdata) = env::var("APPDATA") {
      let ps_history = PathBuf::from(appdata)
        .join("Microsoft")
        .join("Windows")
        .join("PowerShell")
        .join("PSReadLine")
        .join("ConsoleHost_history.txt");

      if ps_history.exists() {
        return Ok(ps_history);
      }
    }
    Err("PowerShell history not found".to_string())
  } else {
    let home = dirs::home_dir().ok_or("Home directory not found")?;

    if let Ok(histfile) = env::var("HISTFILE") {
      let path = PathBuf::from(histfile);
      if path.exists() {
        return Ok(path);
      }
    }

    let possible_paths = vec![
      home.join(".zsh_history"),
      home.join(".bash_history"),
      home.join(".local/share/fish/fish_history"),
    ];

    for path in possible_paths {
      if path.exists() {
        return Ok(path);
      }
    }

    Err("No shell history file found".to_string())
  }
}

fn parse_powershell_history(content: &str) -> Result<String, String> {
  let lines: Vec<&str> = content.lines().collect();

  if lines.len() < 2 {
    return Err("Not enough history".to_string());
  }

  Ok(lines[lines.len() - 2].trim().to_string())
}

fn parse_bash_zsh_history(content: &str) -> Result<String, String> {
  let lines: Vec<&str> = content.lines().collect();

  if lines.is_empty() {
    return Err("Empty history".to_string());
  }

  let re = Regex::new(r"^: \d+:\d+;(.+)$").unwrap();

  for line in lines.iter().rev() {
    let cmd = if let Some(caps) = re.captures(line) {
      caps.get(1).unwrap().as_str()
    } else {
      line.trim()
    };

    if !cmd.starts_with("wtf") && !cmd.is_empty() {
      return Ok(cmd.to_string());
    }
  }

  Err("No valid command found in history".to_string())
}

fn parse_fish_history(content: &str) -> Result<String, String> {
  let re = Regex::new(r"- cmd: (.+)").unwrap();
  let lines: Vec<&str> = content.lines().collect();

  for line in lines.iter().rev() {
    if let Some(caps) = re.captures(line) {
      let cmd = caps.get(1).unwrap().as_str().trim();
      if !cmd.starts_with("wtf") && !cmd.is_empty() {
        return Ok(cmd.to_string());
      }
    }
  }

  Err("No valid command found in history".to_string())
}
