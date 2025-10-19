use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AIConfig {
  pub api_key: Option<String>,
}

impl AIConfig {
  pub fn load() -> Self {
    if let Some(config_dir) = dirs::config_dir() {
      let config_file = config_dir.join("wtf").join("ai_config.json");
      if config_file.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_file) {
          if let Ok(config) = serde_json::from_str(&content) {
            return config;
          }
        }
      }
    }
    Self::default()
  }

  pub fn save(&self) -> Result<(), String> {
    let config_dir = dirs::config_dir()
      .ok_or("Could not find config directory")?
      .join("wtf");

    std::fs::create_dir_all(&config_dir)
      .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let config_file = config_dir.join("ai_config.json");
    let content = serde_json::to_string_pretty(self)
      .map_err(|e| format!("Failed to serialize config: {}", e))?;

    std::fs::write(&config_file, content)
      .map_err(|e| format!("Failed to save AI config: {}", e))?;

    Ok(())
  }
}

pub fn check_api_key() -> Result<String, String> {
  // Try environment variable first
  if let Ok(key) = env::var("GOOGLE_API_KEY") {
    if !key.is_empty() {
      return Ok(key);
    }
  }

  // Try from config
  let config = AIConfig::load();
  if let Some(key) = config.api_key {
    if !key.is_empty() {
      return Ok(key);
    }
  }

  Err("Google API key not found".to_string())
}

pub fn save_api_key(key: String) -> Result<(), String> {
  let mut config = AIConfig::load();
  config.api_key = Some(key);
  config.save()?;
  Ok(())
}

pub async fn fix_command_with_ai(wrong_command: &str) -> Result<String, String> {
  use reqwest::Client;
  use serde_json::json;

  let api_key = check_api_key()?;

  println!(
    "{}",
    "🤖 Asking Google Gemini to fix the command...".bright_cyan()
  );

  let client = Client::new();
  let url =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

  let payload = json!({
      "contents": [{
          "parts": [{
              "text": format!(
                  "You are a shell command expert. Fix this command and output ONLY the corrected command, nothing else: {}",
                  wrong_command
              )
          }]
      }],
      "generationConfig": {
          "temperature": 0.1,
          "maxOutputTokens": 100,
      }
  });

  let response = client
    .post(url)
    .header("Content-Type", "application/json")
    .header("X-goog-api-key", api_key)
    .json(&payload)
    .send()
    .await
    .map_err(|e| format!("API request failed: {}", e))?;

  if !response.status().is_success() {
    return Err(format!("API returned error: {}", response.status()));
  }

  let result: serde_json::Value = response
    .json()
    .await
    .map_err(|e| format!("Failed to parse response: {}", e))?;

  let fixed_command = result["candidates"][0]["content"]["parts"][0]["text"]
    .as_str()
    .ok_or("No response from AI")?
    .trim()
    .to_string();

  clean_ai_response(&fixed_command)
}

fn clean_ai_response(response: &str) -> Result<String, String> {
  // Clean up the response (remove quotes, markdown, etc.)
  let cleaned = response
    .trim_matches('`')
    .trim_matches('"')
    .trim_matches('\'')
    .trim()
    .lines()
    .next()
    .unwrap_or(response)
    .to_string();

  if cleaned.is_empty() {
    return Err("AI returned empty response".to_string());
  }

  Ok(cleaned)
}

pub fn display_api_key_help() {
  println!("{}", "❌ Google API key not found!".bright_red());
  println!();
  println!(
    "{}",
    "To use AI-powered fixing, you need a Google AI API key:".bright_yellow()
  );
  println!();
  println!("{}", "Option 1: Set environment variable".bright_cyan());
  println!("  Windows PowerShell:");
  println!("    $env:GOOGLE_API_KEY = \"your-key-here\"");
  println!("  Linux/macOS:");
  println!("    export GOOGLE_API_KEY=\"your-key-here\"");
  println!();
  println!("{}", "Option 2: Save to config".bright_cyan());
  println!("  wtf set-api-key your-key-here");
  println!();
  println!("{}", "Get your API key from:".bright_cyan());
  println!("  https://aistudio.google.com/app/apikey");
  println!();
  println!(
    "{}",
    "💡 Tip: AI mode uses Google Gemini 2.0 Flash model".dimmed()
  );
}
