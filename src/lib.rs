pub mod shell;
pub mod builtins;

pub enum CommandError {
    FileNotFound(String),
    PermissionDenied(String),
    InvalidArgs(String),
    IOError(String),
    CommandNotFound(String),
    IsADirectory(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandError::FileNotFound(s) => write!(f, "File not found: {}", s),
            CommandError::PermissionDenied(s) => write!(f, "Permission denied: {}", s),
            CommandError::InvalidArgs(s) => write!(f, "Invalid arguments: {}", s),
            CommandError::IOError(s) => write!(f, "IO error: {}", s),
            CommandError::CommandNotFound(s) => write!(f, "Command '{}' not found", s),
            CommandError::IsADirectory(s) => write!(f, "Is a directory: {}", s),
        }
    }
}
