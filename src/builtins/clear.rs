use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::io::{self, Write};

pub struct Clear;

impl Command for Clear {
    fn execute(_args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        // ANSI escape sequence: clear screen + move cursor to top-left,
        print!("\x1B[2J\x1B[H\n");
        io::stdout()
            .flush()
            .map_err(|e| CommandError::IOError(e.to_string()))?;

        Ok(())
    }

    fn name() -> &'static str {
        "clear"
    }

    fn help() -> &'static str {
        "clear: clear the terminal screen"
    }
}
