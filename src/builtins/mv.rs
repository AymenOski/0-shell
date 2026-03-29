use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::PathBuf;

pub struct Mv;

impl Command for Mv {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArgs("missing file operand".to_string()));
        }
        if args.len() == 1 {
            return Err(CommandError::InvalidArgs(format!("missing destination file operand after '{}'", args[0])));
        }

        let src_name = args[0];
        let dest_name = args[1];
        
        let src = resolve_path(src_name, state)?;
        let mut dest = resolve_path(dest_name, state)?;
        
        if src == dest {
            return Err(CommandError::FileOperationFailed(
                format!("'{}' and '{}' are the same file", src_name, dest_name)
            ));
        }
        
        if dest.is_dir() {
            if let Some(file_name) = src.file_name() {
                dest = dest.join(file_name);
                
                if src == dest {
                    return Err(CommandError::FileOperationFailed(
                        format!("cannot move '{}' to a subdirectory of itself, '{}'", src_name, dest.display())
                    ));
                }
            }
        }
        
        fs::rename(&src, &dest)
            .map_err(|e| io_error_to_cmd_error(src_name, e))?;
        
        Ok(())
    }
    
    fn name() -> &'static str {
        "mv"
    }
    
    fn help() -> &'static str {
        "mv: move or rename files"
    }

}

// Resolve a path: handle ~, absolute paths, and relative paths
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

// Convert standard IO errors into our centralized CommandError format
fn io_error_to_cmd_error(src_name: &str, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            CommandError::FileOperationFailed(format!("cannot stat '{}': No such file or directory", src_name))
        }
        std::io::ErrorKind::PermissionDenied => {
            CommandError::PermissionDenied(src_name.to_string())
        }
        _ => {
            // Using FileOperationFailed allows the REPL to format it cleanly as `mv: {msg}` without "IO error:"
            CommandError::FileOperationFailed(format!("cannot move '{}': {}", src_name, e))
        }
    }
}
