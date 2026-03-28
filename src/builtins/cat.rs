use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::PathBuf;

pub struct Cat;

impl Command for Cat {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        let mut i = 0;
        // Loop through each file argument
        for file_path in args {
            i += 1;
            // Resolve the path: handle ~, absolute paths, and relative paths
            let path = resolve_path(file_path, state)?;
            
            // Try to read the file
            let content = fs::read_to_string(&path)
                .map_err(|e| io_error_to_cmd_error(&path, e))?;
            if args.len() == i {
                println!("{}", content);
            }else {
                print!("{}", content);
            }
        }
        
        Ok(())
    }
    
    fn name() -> &'static str {
        "cat"
    }
    
    fn help() -> &'static str {
        "cat: display file contents"
    }
    
    fn validate_args(_args: &[&str]) -> bool {
        !_args.is_empty()
    }
}

/// Convert io::Error to CommandError based on error kind
fn io_error_to_cmd_error(path: &std::path::PathBuf, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            CommandError::FileNotFound(format!("{}", path.display()))
        }
        std::io::ErrorKind::PermissionDenied => {
            CommandError::PermissionDenied(format!("{}", path.display()))
        }
        std::io::ErrorKind::IsADirectory => {
            CommandError::IsADirectory(format!("{}", path.display()))
        }
        _ => {
            CommandError::IOError(e.to_string())
        }
    }
}

/// Resolve a path: handle ~, absolute paths, and relative paths
fn resolve_path(file_path: &str, state: &ShellState) -> Result<PathBuf, CommandError> {
    let path = if file_path == "~" {
        // Just ~ means home directory
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?
    } else if file_path.starts_with("~/") {
        // ~/ means home directory + rest of path
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?;
        home.join(&file_path[2..])
    } else if file_path.starts_with('/') {
        // Absolute path
        PathBuf::from(file_path)
    } else {
        // Relative path: join with current directory
        state.current_dir.join(file_path)
    };
    
    Ok(path)
}
