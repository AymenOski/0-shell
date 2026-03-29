pub mod shell;
pub mod builtins;

pub enum CommandError {
    FileNotFound(String),
    PermissionDenied(String),
    InvalidArgs(String),
    IOError(String),
    CommandNotFound(String),
    IsADirectory(String),
    NotADirectory(String),
    AlreadyExists(String),
    FileOperationFailed(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandError::FileNotFound(s) => write!(f, "No such file or directory: {}", s),
            CommandError::PermissionDenied(s) => write!(f, "Permission denied: {}", s),
            CommandError::InvalidArgs(s) => write!(f, "{}", s),
            CommandError::IOError(s) => write!(f, "{}", s),
            CommandError::CommandNotFound(s) => write!(f, "Command '{}' not found", s),
            CommandError::IsADirectory(s) => write!(f, "Is a directory: {}", s),
            CommandError::NotADirectory(s) => write!(f, "Not a directory: {}", s),
            CommandError::AlreadyExists(s) => write!(f, "File exists: {}", s),
            CommandError::FileOperationFailed(s) => write!(f, "{}", s),
        }
    }
}
