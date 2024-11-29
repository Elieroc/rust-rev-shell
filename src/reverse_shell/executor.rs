use std::process::{Command, Stdio};

pub fn execute_powershell_command(command: &str) -> String {
    let cmd = format!("powershell -NoProfile -Command {}", command);
    match Command::new("cmd")
        .arg("/C")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{}\n{}", stdout, stderr)
        }
        Err(e) => format!("Error executing PowerShell command: {}\n", e),
    }
}

pub fn execute_bash_command(command: &str) -> String {
    match Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{}\n{}", stdout, stderr)
        }
        Err(e) => format!("Error executing Bash command: {}\n", e),
    }
}
