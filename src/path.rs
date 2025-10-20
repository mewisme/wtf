use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_install_dir() -> Result<PathBuf, String> {
  let home = dirs::home_dir().ok_or("Could not find home directory")?;
  Ok(home.join(".wtf").join("bin"))
}

pub fn get_current_exe() -> Result<PathBuf, String> {
  env::current_exe().map_err(|e| format!("Could not get current executable path: {}", e))
}

#[cfg(target_os = "windows")]
pub fn add_to_path() -> Result<(), String> {
  use std::process::Command;

  let install_dir = get_install_dir()?;
  let current_exe = get_current_exe()?;

  fs::create_dir_all(&install_dir)
    .map_err(|e| format!("Failed to create install directory: {}", e))?;

  let dest = install_dir.join("wtf.exe");
  fs::copy(&current_exe, &dest).map_err(|e| format!("Failed to copy binary: {}", e))?;

  println!("{} Binary copied to:", "✓".bright_green());
  println!("  {}", dest.display().to_string().bright_white());
  println!();

  let install_dir_str = install_dir.to_string_lossy().to_string();

  let ps_command = format!(
    "$path = [Environment]::GetEnvironmentVariable('Path', 'User'); \
         if ($path -notlike '*{}*') {{ \
             [Environment]::SetEnvironmentVariable('Path', $path + ';{}', 'User'); \
             Write-Host 'Added to PATH' \
         }} else {{ \
             Write-Host 'Already in PATH' \
         }}",
    install_dir_str, install_dir_str
  );

  let output = Command::new("powershell")
    .arg("-Command")
    .arg(&ps_command)
    .output()
    .map_err(|e| format!("Failed to update PATH: {}", e))?;

  if output.status.success() {
    println!("{} Added to PATH:", "✓".bright_green());
    println!("  {}", install_dir_str.bright_white());
    println!();
    println!(
      "{}",
      "⚠️  Restart your terminal for PATH changes to take effect".bright_yellow()
    );
  } else {
    return Err("Failed to add to PATH. You may need administrator privileges.".to_string());
  }

  Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn add_to_path() -> Result<(), String> {
  let install_dir = get_install_dir()?;
  let current_exe = get_current_exe()?;

  fs::create_dir_all(&install_dir)
    .map_err(|e| format!("Failed to create install directory: {}", e))?;

  let dest = install_dir.join("wtf");
  fs::copy(&current_exe, &dest).map_err(|e| format!("Failed to copy binary: {}", e))?;

  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&dest)
      .map_err(|e| format!("Failed to get file permissions: {}", e))?
      .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&dest, perms).map_err(|e| format!("Failed to set permissions: {}", e))?;
  }

  println!("{} Binary installed to:", "✓".bright_green());
  println!("  {}", dest.display().to_string().bright_white());
  println!();

  let install_dir_str = install_dir.to_string_lossy().to_string();

  let shell = env::var("SHELL").unwrap_or_else(|_| String::from("unknown"));
  let shell_name = shell.split('/').last().unwrap_or("sh");

  println!("{}", "Add to your shell configuration:".bright_cyan());
  println!();

  match shell_name {
    "bash" => {
      println!("  echo 'export PATH=\"$HOME/.wtf/bin:$PATH\"' >> ~/.bashrc");
      println!("  source ~/.bashrc");
    }
    "zsh" => {
      println!("  echo 'export PATH=\"$HOME/.wtf/bin:$PATH\"' >> ~/.zshrc");
      println!("  source ~/.zshrc");
    }
    "fish" => {
      println!("  fish_add_path ~/.wtf/bin");
    }
    _ => {
      println!("  export PATH=\"$HOME/.wtf/bin:$PATH\"");
    }
  }

  println!();
  println!("{}", "Or add manually:".dimmed());
  println!("  export PATH=\"{}:$PATH\"", install_dir_str);

  Ok(())
}

#[cfg(target_os = "windows")]
pub fn remove_from_path() -> Result<(), String> {
  use std::process::Command;

  let install_dir = get_install_dir()?;
  let install_dir_str = install_dir.to_string_lossy().to_string();

  let ps_command = format!(
    "$path = [Environment]::GetEnvironmentVariable('Path', 'User'); \
         $newPath = ($path -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'; \
         [Environment]::SetEnvironmentVariable('Path', $newPath, 'User'); \
         Write-Host 'Removed from PATH'",
    install_dir_str
  );

  let output = Command::new("powershell")
    .arg("-Command")
    .arg(&ps_command)
    .output()
    .map_err(|e| format!("Failed to update PATH: {}", e))?;

  if output.status.success() {
    println!("{} Removed from PATH:", "✓".bright_green());
    println!("  {}", install_dir_str.bright_white());
    println!();

    let binary = install_dir.join("wtf.exe");
    if binary.exists() {
      if let Err(e) = fs::remove_file(&binary) {
        println!("{}", format!("⚠️  Could not remove binary: {}", e).yellow());
      } else {
        println!("{} Binary removed", "✓".bright_green());
      }
    }

    if install_dir.exists() {
      if let Ok(entries) = fs::read_dir(&install_dir) {
        if entries.count() == 0 {
          let _ = fs::remove_dir(&install_dir);
        }
      }
    }

    println!();
    println!(
      "{}",
      "⚠️  Restart your terminal for PATH changes to take effect".bright_yellow()
    );
  } else {
    return Err("Failed to remove from PATH".to_string());
  }

  Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn remove_from_path() -> Result<(), String> {
  let install_dir = get_install_dir()?;
  let install_dir_str = install_dir.to_string_lossy().to_string();

  println!("{}", "Manual removal required:".bright_yellow());
  println!();

  let shell = env::var("SHELL").unwrap_or_else(|_| String::from("unknown"));
  let shell_name = shell.split('/').last().unwrap_or("sh");

  println!("{}", "Remove from your shell configuration:".bright_cyan());
  println!();

  match shell_name {
    "bash" => println!(
      "  Edit ~/.bashrc and remove the line with: {}/.wtf/bin",
      dirs::home_dir().unwrap().display()
    ),
    "zsh" => println!(
      "  Edit ~/.zshrc and remove the line with: {}/.wtf/bin",
      dirs::home_dir().unwrap().display()
    ),
    "fish" => {
      println!("  Run: set -U fish_user_paths (string match -v ~/.wtf/bin $fish_user_paths)")
    }
    _ => println!("  Remove from PATH: {}", install_dir_str),
  }

  println!();

  let binary = install_dir.join("wtf");
  if binary.exists() {
    fs::remove_file(&binary).map_err(|e| format!("Failed to remove binary: {}", e))?;
    println!("{} Binary removed from:", "✓".bright_green());
    println!("  {}", binary.display().to_string().bright_white());
  }

  if install_dir.exists() {
    if let Ok(entries) = fs::read_dir(&install_dir) {
      if entries.count() == 0 {
        fs::remove_dir(&install_dir).map_err(|e| format!("Failed to remove directory: {}", e))?;
        println!("{} Directory removed", "✓".bright_green());
      }
    }
  }

  Ok(())
}
