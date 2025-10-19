use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
  pub custom_typos: Vec<(String, String)>,
  #[serde(default)]
  pub first_run_complete: bool,
  #[serde(default)]
  pub auto_mode: bool,
  #[serde(default)]
  pub google_api_key: Option<String>,
}

impl Default for UserConfig {
  fn default() -> Self {
    Self {
      custom_typos: Vec::new(),
      first_run_complete: false,
      auto_mode: false,
      google_api_key: None,
    }
  }
}

impl UserConfig {
  pub fn load() -> Self {
    match Self::config_path() {
      Ok(path) => {
        if path.exists() {
          if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
              return config;
            }
          }
        }
        Self::default()
      }
      Err(_) => Self::default(),
    }
  }

  pub fn save(&self) -> Result<(), String> {
    let path = Self::config_path()?;

    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(self)
      .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
  }

  pub fn add_typo(&mut self, wrong: String, correct: String) {
    // Remove if already exists
    self.custom_typos.retain(|(w, _)| w != &wrong);

    // Add new entry
    self.custom_typos.push((wrong, correct));
  }

  pub fn remove_typo(&mut self, wrong: &str) -> bool {
    let original_len = self.custom_typos.len();
    self.custom_typos.retain(|(w, _)| w != wrong);
    self.custom_typos.len() < original_len
  }

  pub fn add_from_builtin(&mut self, wrong: String, correct: String) {
    // When adding from built-in, check if it's not already there
    if !self.custom_typos.iter().any(|(w, _)| w == &wrong) {
      self.custom_typos.push((wrong, correct));
    }
  }

  pub fn mark_first_run_complete(&mut self) {
    self.first_run_complete = true;
  }

  pub fn set_auto_mode(&mut self, enabled: bool) {
    self.auto_mode = enabled;
  }

  pub fn toggle_auto_mode(&mut self) -> bool {
    self.auto_mode = !self.auto_mode;
    self.auto_mode
  }

  pub fn set_google_api_key(&mut self, key: String) {
    self.google_api_key = Some(key);
  }

  pub fn get_google_api_key(&self) -> Option<String> {
    self.google_api_key.clone()
  }

  fn config_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home.join(".wtf").join("config.json"))
  }

  pub fn get_config_path_display() -> String {
    Self::config_path()
      .map(|p| p.to_string_lossy().to_string())
      .unwrap_or_else(|_| "unknown".to_string())
  }
}
