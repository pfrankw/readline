# Readline

Readline is a Rust asynchronous command line reader/emulator library.


# 🔑 Features
- Async, non-blocking terminal input
- History navigation (Up/Down arrows)
- Line editing (Left/Right arrows, Backspace, Delete)
- Persistent history file support
- Customizable prompt
- Works with any tokio::io::AsyncRead
- Cross-platform (Linux, macOS, Windows)

# ✨ Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
readline = { git = "https://github.com/pfrankw/readline.git", version = "0.1.5" }
```

# ⚡ Quick start

```rust
use readline::Readline; // or whatever your crate name will be
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let readline = Readline::new(None, "> ", Some(Path::new("history.txt"))).await;

    Readline::enable_raw_mode().unwrap();

    loop {
        match readline.run().await {
            Ok(line) => {
                println!("You entered: {}", line);
            }
            Err(_) => {
                break; // Exit on Ctrl+C
            }
        }
    }

    Readline::disable_raw_mode().unwrap();

    Ok(())
}

```

# 🔥 Why use this?

- Native tokio support — no blocking anywhere
- Multithread natively supported
- Lightweight and minimal
- Clean async design

# Author
Made with ❤️ by Francesco Pompo'.
