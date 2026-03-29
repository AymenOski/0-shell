use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::PathBuf;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        // Loop through each directory to create
        for dir_name in args {
            let path = resolve_path(dir_name, state)?;
            
            // Try to create the directory
            fs::create_dir(&path)
                .map_err(|e| io_error_to_cmd_error(&path, e))?;
        }
        
        Ok(())
    }
    
    fn name() -> &'static str {
        "mkdir"
    }
    
    fn help() -> &'static str {
        "mkdir: create directories"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        !_args.is_empty()
    }
}

/// Resolve a path: handle ~, absolute paths, and relative paths
fn resolve_path(dir_name: &str, state: &ShellState) -> Result<PathBuf, CommandError> {
    let path = if dir_name == "~" {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?
    } else if dir_name.starts_with("~/") {
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?;
        home.join(&dir_name[2..])
    } else if dir_name.starts_with('/') {
        PathBuf::from(dir_name)
    } else {
        state.current_dir.join(dir_name)
    };
    
    Ok(path)
}

/// Convert io::Error to CommandError based on error kind
fn io_error_to_cmd_error(path: &PathBuf, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::AlreadyExists => {
            CommandError::FileOperationFailed(format!("mkdir: cannot create directory '{}': File exists", path.display()))
        }
        std::io::ErrorKind::PermissionDenied => {
            CommandError::PermissionDenied(format!("{}", path.display()))
        }
        std::io::ErrorKind::NotFound => {
            // Parent directory doesn't exist
            CommandError::FileOperationFailed(format!("mkdir: cannot create directory '{}': No such file or directory", path.display()))
        }
        _ => {
            CommandError::IOError(e.to_string())
        }
    }
}
