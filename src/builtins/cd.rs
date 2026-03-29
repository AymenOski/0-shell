use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::path::PathBuf;

pub struct Cd;

impl Command for Cd {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        // If no arguments, default to HOME
        let target = if args.is_empty() {
            "~"  // Default to home directory
        } else {
            args[0]
        };
        
        // Handle special cases before path building
        let target_path = if target == "-" {
            // Go to previous directory (cd -)
            // Check if we have a previous directory stored
            state.previous_dir.clone()
                .ok_or(CommandError::InvalidArgs("No previous directory".to_string()))?
        } else if target.starts_with('~') {
            // Expand ~ to home directory
            let home = std::env::var("HOME")
                .ok()
                .map(PathBuf::from)
                .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?;
            
            if target == "~" {
                // Just ~ means home
                home
            } else if target.starts_with("~/") {
                // ~/ means home + rest of path
                home.join(&target[2..])
            } else {
                return Err(CommandError::InvalidArgs(format!("Invalid path: {}", target)));
            }
        } else if target.starts_with('/') {
            // Absolute path
            PathBuf::from(target)
        } else {
            // Relative path - resolve from current directory
            state.current_dir.join(target)
        };
        
        // Canonicalize the path to resolve .. and .
        // This converts /home/user/target/.. into /home/user (resolved)
        let canonical_path = target_path.canonicalize()
            .map_err(|e| io_error_to_cmd_error(target, e))?;
        
        // **Store the current directory before changing** (for cd -)
        let previous_dir = state.current_dir.clone();
        
        // Tell the OS to actually change the working directory
        std::env::set_current_dir(&canonical_path)
            .map_err(|e| io_error_to_cmd_error(target, e))?;
        
        // Update our state to track both current and previous directory
        state.previous_dir = Some(previous_dir);
        state.current_dir = canonical_path;
        Ok(())
    }
    
    fn name() -> &'static str {
        "cd"
    }
    
    fn help() -> &'static str {
        "cd: change directory"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        // cd accepts 0 args (go to home) or 1 arg (go to specific path)
        // Reject 2+ args: "cd /tmp /home" is invalid
        args.len() <= 1
    }
}

/// Convert io::Error to CommandError based on error kind
fn io_error_to_cmd_error(target: &str, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            CommandError::FileNotFound(target.to_string())
        }
        std::io::ErrorKind::PermissionDenied => {
            CommandError::PermissionDenied(target.to_string())
        }
        std::io::ErrorKind::NotADirectory => {
            CommandError::NotADirectory(target.to_string())
        }
        _ => {
            CommandError::IOError(e.to_string())
        }
    }
}
