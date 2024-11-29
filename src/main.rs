
#[cfg(feature = "payload")]
mod payload;  // Inclure le module reverse_shell si la feature "reverse_shell" est activée

#[cfg(feature = "listener")]
mod listener;  // Inclure le module listener si la feature "listener" est activée

fn main() {
    // Logique conditionnelle
    #[cfg(feature = "payload")]
    {
        println!("Exécution du reverse shell...");
        payload::run_reverse_shell();
    }

    #[cfg(feature = "listener")]
    {
        println!("Lancement du listener...");
        listener::start_listener();
    }
}
