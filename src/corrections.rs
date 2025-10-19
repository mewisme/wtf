use crate::commands::{get_common_commands, get_common_fixes};
use crate::config::UserConfig;
use strsim::jaro_winkler;

#[derive(Debug, Clone)]
pub struct Correction {
  pub fixed_cmd: String,
  pub reason: String,
  pub confidence: f64,
}

pub fn find_corrections(cmd: &str, user_config: &UserConfig) -> Option<Vec<Correction>> {
  let mut corrections = Vec::new();

  let parts: Vec<&str> = cmd.split_whitespace().collect();
  if parts.is_empty() {
    return None;
  }

  let command = parts[0];
  let args = if parts.len() > 1 {
    parts[1..].join(" ")
  } else {
    String::new()
  };

  // Check user custom fixes first (highest priority)
  for (wrong, correct) in &user_config.custom_typos {
    // Exact match
    if cmd == wrong || command == wrong {
      let fixed = if cmd == wrong {
        correct.clone()
      } else if args.is_empty() {
        correct.clone()
      } else {
        format!("{} {}", correct, args)
      };

      corrections.push(Correction {
        fixed_cmd: fixed,
        reason: "custom fix".to_string(),
        confidence: 1.0,
      });
    }
    // Starts with pattern (for commands with args)
    else if cmd.starts_with(wrong) && cmd.len() > wrong.len() {
      let remaining = &cmd[wrong.len()..];
      if remaining.starts_with(' ') {
        let fixed = format!("{}{}", correct, remaining);
        corrections.push(Correction {
          fixed_cmd: fixed,
          reason: "custom fix".to_string(),
          confidence: 1.0,
        });
      }
    }
  }

  // Check against built-in fixes
  let common_fixes = get_common_fixes();
  for (typo_pattern, fix_info) in &common_fixes {
    let matched = if command == *typo_pattern || cmd == *typo_pattern {
      // Exact match
      true
    } else if cmd.starts_with(typo_pattern) && cmd.len() > typo_pattern.len() {
      // Starts with pattern and has args
      let next_char = &cmd[typo_pattern.len()..typo_pattern.len() + 1];
      next_char == " "
    } else {
      false
    };

    if matched {
      let fixed = if cmd == *typo_pattern {
        fix_info.0.to_string()
      } else if cmd.starts_with(typo_pattern) {
        // Replace the typo part with the fix
        let remaining = &cmd[typo_pattern.len()..];
        format!("{}{}", fix_info.0, remaining)
      } else if args.is_empty() {
        fix_info.0.to_string()
      } else {
        format!("{} {}", fix_info.0, args)
      };

      // Check if not already added
      if !corrections.iter().any(|c| c.fixed_cmd == fixed) {
        corrections.push(Correction {
          fixed_cmd: fixed,
          reason: fix_info.1.to_string(),
          confidence: 1.0,
        });
      }
    }
  }

  // If no exact match, try fuzzy matching
  if corrections.is_empty() {
    let common_commands = get_common_commands();

    for common_cmd in &common_commands {
      let similarity = jaro_winkler(command, common_cmd);

      if similarity > 0.85 && similarity < 1.0 {
        let fixed = if args.is_empty() {
          common_cmd.to_string()
        } else {
          format!("{} {}", common_cmd, args)
        };

        corrections.push(Correction {
          fixed_cmd: fixed,
          reason: format!("similar to '{}'", common_cmd),
          confidence: similarity,
        });
      }
    }
  }

  // Sort by confidence
  corrections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

  // Limit to top 5 suggestions
  if corrections.len() > 5 {
    corrections.truncate(5);
  }

  if corrections.is_empty() {
    None
  } else {
    Some(corrections)
  }
}
