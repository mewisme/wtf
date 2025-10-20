use crate::config::UserConfig;
use colored::Colorize;
use std::env;

pub fn check_api_key() -> Result<String, String> {
  if let Ok(key) = env::var("GOOGLE_API_KEY") {
    if !key.is_empty() {
      return Ok(key);
    }
  }

  let config = UserConfig::load();
  if let Some(key) = config.get_google_api_key() {
    if !key.is_empty() {
      return Ok(key);
    }
  }

  Err("Google API key not found".to_string())
}

pub fn save_api_key(key: String) -> Result<(), String> {
  let mut config = UserConfig::load();
  config.set_google_api_key(key);
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
