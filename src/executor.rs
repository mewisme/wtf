use std::process::{Command, Stdio};

pub fn execute_command(cmd: &str) -> Result<(), String> {
    let (shell, shell_arg) = if cfg!(target_os = "windows") {
        ("powershell", "-Command")
    } else {
        ("sh", "-c")
    };

    let status = Command::new(shell)
        .arg(shell_arg)
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !status.success() {
        return Err(format!("Command exited with status: {}", status));
    }

    Ok(())
}

