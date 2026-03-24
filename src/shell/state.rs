use std::path::PathBuf;
use std::env;

/// Represents the state of the shell
/// This includes things like the current directory and previous directory
pub struct ShellState {
    pub current_dir: PathBuf,
    pub previous_dir: Option<PathBuf>,
}

impl ShellState {
    /// Create a new shell state with the current OS working directory
    pub fn new() -> std::io::Result<Self> {
        let current_dir = env::current_dir()?;
        Ok(ShellState {
            current_dir,
            previous_dir: None,
        })
    }
}
