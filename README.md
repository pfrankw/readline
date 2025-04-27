# Readline

A Rust library providing asynchronous command line emulation with editing capabilities and history management.

## 🔑 Features
- Async, non-blocking terminal input
- History navigation (Up/Down arrows)
- Persistent history file support
- Line editing (Left/Right arrows, Backspace, Delete)
- Customizable prompt
- Support for custom input sources (Works with any tokio::io::AsyncRead)
- Cross-platform (Linux, macOS, Windows)

## ⚙️ Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
readline = { git = "https://github.com/pfrankw/readline.git", version = "0.1.5" }
```

## ⚡ Quick start

```rust
use readline::Readline;
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let readline = Readline::new(None, "> ", Some(Path::new("history.txt"))).await;

    Readline::enable_raw_mode()?;

    loop {
        match readline.run().await {
            Ok(line) => {
                println!("You entered: {}", line);
            }
            Err(_) => {
                break;
            }
        }
    }

    Readline::disable_raw_mode()?;

    Ok(())
}

```

## Key Features

### History Management

The library automatically manages command history, allowing users to navigate through previous commands using up and down arrow keys. History can be persisted to a file for use across sessions.

### Line Editing

Users can edit the current input line with:
- Left/right arrow keys to move the cursor
- Backspace to delete characters before the cursor
- Delete key to remove characters after the cursor

### Custom Input Sources

While the library defaults to using stdin, you can provide your own input source that implements `AsyncRead + Unpin`.

## Author
Made with ❤️ by Francesco Pompo'.
