use crossterm::terminal;
use std::path::Path;
use tokio::{
    fs::OpenOptions,
    io::{self, AsyncRead, AsyncReadExt, AsyncWriteExt},
    sync::{Mutex, RwLock},
};

pub struct Readline<R> {
    prompt: RwLock<String>,
    history: RwLock<Vec<String>>,
    history_pos: RwLock<usize>, // Add history position tracking
    current_input: RwLock<String>,
    ci_pos: RwLock<usize>,
    reader: Mutex<R>,
    history_file: Option<Mutex<tokio::fs::File>>,
}

impl<R: AsyncRead + Unpin> Readline<R> {
    /// Creates a new instance of the `Readline` struct.
    ///
    /// This function initializes a `Readline` instance with the provided reader, prompt string,
    /// and an optional history file. It also loads the history from the file, if available,
    /// and initializes the internal state.
    ///
    /// # Arguments
    ///
    /// * `reader` - An asynchronous reader that provides the input stream for reading user input.
    /// * `prompt` - A string that will be displayed as the prompt for the user to see when they
    ///   are typing. This is typically a short message like "Enter your command: ".
    /// * `history_file` - An optional path to a file from which previous command history is loaded.
    ///   If `None`, no history is loaded, and the history will be empty.
    ///
    /// # Returns
    ///
    /// This function returns a `Readline` instance that is ready to run. The state is initialized,
    /// and history is loaded if a valid file path is provided.
    ///
    /// # Example
    ///
    /// ```
    /// use readline::Readline;
    /// use std::io::{self, AsyncRead};
    /// use tokio::io::stdin;
    ///
    /// let reader = stdin();
    /// let readline = Readline::new(reader, "Enter your command: ", None).await.unwrap();
    /// ```
    pub async fn new(reader: R, prompt: &str, history_file: Option<&Path>) -> Self {
        let readline = Self {
            prompt: RwLock::new(String::from(prompt)),
            history: RwLock::new(Vec::new()),
            history_pos: RwLock::new(0), // Initialize history position
            current_input: RwLock::new(String::new()),
            ci_pos: Default::default(),
            reader: Mutex::new(reader),
            history_file: match history_file {
                Some(path) => Some(Mutex::new(
                    OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .truncate(false)
                        .open(path)
                        .await
                        .unwrap(),
                )),
                None => None,
            },
        };

        readline.history_load().await.unwrap();
        readline
    }

    pub fn enable_raw_mode() -> crossterm::Result<()> {
        terminal::enable_raw_mode()
    }

    pub fn disable_raw_mode() -> crossterm::Result<()> {
        terminal::disable_raw_mode()
    }

    pub async fn run(&self) -> std::io::Result<String> {
        let _ = self.print_current_line().await;

        loop {
            let k = self.get_keycode().await?;

            match k {
                // CTRL + c
                3 => {
                    break;
                }
                // Control code
                27 => {
                    let _ = self.get_keycode().await?;
                    let k = self.get_keycode().await?;
                    match k {
                        65 => {
                            self.on_up_arrow().await?;
                        }
                        66 => {
                            self.on_down_arrow().await?;
                        }
                        67 => {
                            self.on_right_arrow().await?;
                        }
                        68 => {
                            self.on_left_arrow().await?;
                        }
                        _ => { /*break;*/ }
                    }
                }
                126 => {
                    self.on_canc().await?;
                }
                127 => {
                    self.on_backspace().await?;
                }
                13 => {
                    return self.on_enter().await;
                }
                _ => {
                    self.insert_ci(k as char).await?;
                }
            }
        }

        Err(std::io::Error::new(std::io::ErrorKind::Other, "Exited"))
    }

    async fn get_keycode(&self) -> Result<u8, io::Error> {
        let mut buffer = [0u8; 1];

        self.reader.lock().await.read_exact(&mut buffer).await?;
        Ok(buffer[0])
    }

    async fn insert_ci(&self, what: char) -> io::Result<()> {
        self.ci_insert_pos(what).await;

        if *self.ci_pos.read().await != self.current_input.read().await.len() {
            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        } else {
            Self::write_flush(format!("{}", what)).await?;
        }

        Ok(())
    }

    async fn on_left_arrow(&self) -> io::Result<()> {
        let mut ci_pos = self.ci_pos.write().await;

        if *ci_pos > 0 {
            *ci_pos -= 1;
            std::mem::drop(ci_pos);

            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        }

        Ok(())
    }

    async fn on_right_arrow(&self) -> io::Result<()> {
        let ci = self.current_input.read().await;
        let mut ci_pos = self.ci_pos.write().await;

        if *ci_pos < ci.len() {
            *ci_pos += 1;
            std::mem::drop(ci_pos);
            std::mem::drop(ci);

            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        }

        Ok(())
    }

    async fn on_up_arrow(&self) -> io::Result<()> {
        let mut hp = self.history_pos.write().await;
        let history = self.history.read().await;

        if *hp > 0 {
            *hp -= 1;
            self.set_ci(history[*hp].clone()).await;
            std::mem::drop(hp);
            std::mem::drop(history);

            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        }

        Ok(())
    }

    async fn on_down_arrow(&self) -> io::Result<()> {
        let mut hp = self.history_pos.write().await;
        let history = self.history.read().await;

        if *hp < history.len() {
            *hp += 1;
            self.set_ci(history.get(*hp).cloned().unwrap_or_default())
                .await;
            std::mem::drop(hp);
            std::mem::drop(history);

            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        }

        Ok(())
    }

    async fn on_enter(&self) -> io::Result<String> {
        let mut ci = self.current_input.write().await;

        Self::write_flush("\r\n".to_string()).await?; // Move to the next line

        if !(*ci).is_empty() {
            self.history_push(ci.clone()).await;
        }

        self.reset_history_pos().await;
        let r = ci.clone();

        ci.clear(); // error on purpose

        *self.ci_pos.write().await = 0;

        Ok(r)
    }

    async fn on_backspace(&self) -> io::Result<()> {
        if self.ci_remove_pos().await {
            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        }

        Ok(())
    }

    async fn on_canc(&self) -> io::Result<()> {
        if self.ci_remove_pos_right().await {
            Self::clear_current_line().await?;
            let _ = self.print_current_line().await;
        }

        Ok(())
    }

    async fn _current_input_pop(&self) {
        self.current_input.write().await.pop();
    }

    async fn _current_input_push(&self, what: char) {
        self.current_input.write().await.push(what);
    }

    async fn ci_insert_pos(&self, what: char) {
        let mut ci_pos = self.ci_pos.write().await;

        self.current_input.write().await.insert(*ci_pos, what);
        *ci_pos += 1;
    }

    // Returns where to update the current line or not
    async fn ci_remove_pos(&self) -> bool {
        let mut ci = self.current_input.write().await;
        let mut ci_pos = self.ci_pos.write().await;

        // If there is nothing to delete or the position is already zero.
        if ci.is_empty() || *ci_pos == 0 {
            return false;
        }

        *ci_pos -= 1;
        ci.remove(*ci_pos);

        return true;
    }

    // Returns where to update the current line or not
    async fn ci_remove_pos_right(&self) -> bool {
        let mut ci = self.current_input.write().await;
        let ci_pos = self.ci_pos.read().await;

        // If there is nothing to delete or the position is already at the extreme right.
        if ci.is_empty() || *ci_pos == ci.len() {
            return false;
        }

        ci.remove(*ci_pos);

        true
    }

    async fn set_ci(&self, what: String) {
        *self.ci_pos.write().await = what.len();
        *self.current_input.write().await = what;
    }

    async fn reset_history_pos(&self) {
        *self.history_pos.write().await = self.history.read().await.len(); // Reset history position
                                                                           // History file truncate
    }

    async fn history_load(&self) -> std::io::Result<()> {
        if let Some(file) = self.history_file.as_ref() {
            let mut file = file.lock().await;

            let mut content = String::new();
            file.read_to_string(&mut content).await?;

            let mut history = self.history.write().await;
            *history = content.lines().map(|s| s.to_string()).collect();
        }

        self.reset_history_pos().await;

        Ok(())
    }

    async fn history_push(&self, what: String) {
        self.history.write().await.push(what.clone());

        // Add the history item to the history file
        if let Some(file) = self.history_file.as_ref() {
            let mut file = file.lock().await;
            file.write_all(format!("{}\n", what).as_bytes())
                .await
                .unwrap();
            file.flush().await.unwrap();
        }
    }

    async fn write_flush(what: String) -> std::io::Result<()> {
        let mut stderr = tokio::io::stderr();

        stderr.write_all(what.as_bytes()).await?;
        stderr.flush().await?;

        Ok(())
    }

    async fn print_current_line(&self) -> std::io::Result<()> {
        let mut stderr = tokio::io::stderr();
        let prompt = self.get_prompt().await;

        stderr
            .write_all(format!("\r{}{}", prompt, self.current_input.read().await).as_bytes())
            .await?;
        stderr.flush().await?;
        Self::move_cursor_col(prompt.len() + *self.ci_pos.read().await + 1).await?;

        Ok(())
    }

    pub async fn get_prompt(&self) -> String {
        self.prompt.read().await.clone()
    }

    pub async fn set_prompt(&self, new_prompt: String) {
        let mut prompt = self.prompt.write().await;
        *prompt = new_prompt;
    }

    async fn _move_cursor(row: usize, col: usize) -> std::io::Result<()> {
        Self::write_flush(format!("\x1B[{};{}H", row, col)).await
    }

    async fn move_cursor_col(col: usize) -> std::io::Result<()> {
        Self::write_flush(format!("\x1B[{}G", col)).await
    }

    async fn clear_current_line() -> std::io::Result<()> {
        Self::write_flush("\x1B[2K\r".to_string()).await
    }
}
