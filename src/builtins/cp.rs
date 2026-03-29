use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::PathBuf;

pub struct Cp;

impl Command for Cp {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        let src_name = args[0];
        let dest_name = args[1];
        
        // Resolve source and destination paths
        let src = resolve_path(src_name, state)?;
        let dest = resolve_path(dest_name, state)?;
        
        // Check if source is a directory
        if src.is_dir() {
            return Err(CommandError::FileOperationFailed(
                format!("omitting directory '{}'", src_name)
            ));
        }
        
        // Copy the file
        fs::copy(&src, &dest)
            .map_err(|e| io_error_to_cmd_error(src_name, dest_name, &src, &dest, e))?;
        
        Ok(())
    }
    
    fn name() -> &'static str {
        "cp"
    }
    
    fn help() -> &'static str {
        "cp: copy files"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        args.len() >= 2
    }
}

/// Resolve a path: handle ~, absolute paths, and relative paths
fn resolve_path(file_name: &str, state: &ShellState) -> Result<PathBuf, CommandError> {
    let path = if file_name == "~" {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?
    } else if file_name.starts_with("~/") {
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?;
        home.join(&file_name[2..])
    } else if file_name.starts_with('/') {
        PathBuf::from(file_name)
    } else {
        state.current_dir.join(file_name)
    };
    
    Ok(path)
}

// Convert io::Error to CommandError for cp operation
/// Takes both original names (for error messages) and resolved paths
fn io_error_to_cmd_error(src_name: &str, dest_name: &str, _src: &PathBuf, _dest: &PathBuf, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            CommandError::FileNotFound(src_name.to_string())
        }
        std::io::ErrorKind::PermissionDenied => {
            CommandError::PermissionDenied(src_name.to_string())
        }
        std::io::ErrorKind::IsADirectory => {
            CommandError::IsADirectory(dest_name.to_string())
        }
        _ => {
            CommandError::IOError(e.to_string())
        }
    }
}
