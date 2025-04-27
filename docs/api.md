# Readline API Documentation

This document provides detailed information about the Readline library API.

## Readline Struct

The main struct that provides line editing functionality.

```rust
pub struct Readline {
    // ... internal fields
}
```

### Constructor

```rust
pub async fn new(
    reader: Option<Box<dyn AsyncRead + Unpin>>,
    prompt: &str,
    history_file: Option<&Path>,
) -> Self
```

Creates a new `Readline` instance.

**Parameters:**
- `reader`: Optional custom input source. If `None`, defaults to stdin.
- `prompt`: The prompt string to display before each input line.
- `history_file`: Optional path to a file for persisting command history.

**Example:**
```rust
let readline = Readline::new(
    None,
    ">> ",
    Some(Path::new("~/.app_history"))
).await;
```

### Terminal Mode Management

```rust
pub fn enable_raw_mode() -> crossterm::Result<()>
```

Enables raw mode in the terminal, which is required for proper functioning of the line editor.

```rust
pub fn disable_raw_mode() -> crossterm::Result<()>
```

Disables raw mode, restoring the terminal to its normal state. Should be called before exiting the application.

### Input Reading

```rust
pub async fn run(&self) -> std::io::Result<String>
```

Starts the line editing session and returns the entered line when the user presses Enter.

**Returns:**
- `Ok(String)`: The entered line.
- `Err`: If Ctrl+C was pressed or another error occurred.

### Prompt Management

```rust
pub async fn get_prompt(&self) -> String
```

Returns the current prompt string.

```rust
pub async fn set_prompt(&self, new_prompt: String)
```

Sets a new prompt string.

## Key Bindings

The library responds to the following key combinations:

| Key | Action |
|-----|--------|
| Enter | Submit the current line |
| Ctrl+C | Cancel input and exit |
| Left Arrow | Move cursor left |
| Right Arrow | Move cursor right |
| Up Arrow | Navigate to previous history item |
| Down Arrow | Navigate to next history item |
| Backspace | Delete character before cursor |
| Delete | Delete character at cursor position |

## Internal Methods

The library also contains several internal methods that handle specific aspects of line editing:

- History management
- Cursor movement
- Line rendering
- Character insertion and deletion

These methods are not intended to be called directly by users of the library.

## Error Handling

Most methods return `std::io::Result<T>`, allowing errors to be propagated and handled by the caller.

## Thread Safety

The library uses Tokio's synchronization primitives (`RwLock` and `Mutex`) to ensure thread safety when accessing shared state.

