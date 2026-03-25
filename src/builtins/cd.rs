use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::path::PathBuf;

pub struct Cd;

impl Command for Cd {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        // Get the target directory from args
        let target = args[0];
        
        // Convert to PathBuf
        let target_path = if target.starts_with('/') {
            // Absolute path
            PathBuf::from(target)
        } else {
            // Relative path - resolve from current directory
            state.current_dir.join(target)
        };
        
        // Check if directory exists
        if !target_path.exists() {
            return Err(CommandError::FileNotFound(format!("{}", target_path.display())));
        }
        
        // Check if it's actually a directory
        if !target_path.is_dir() {
            return Err(CommandError::InvalidArgs(format!("{} is not a directory", target)));
        }
        
        // Tell the OS to actually change the working directory
        std::env::set_current_dir(&target_path)
            .map_err(|e| CommandError::IOError(e.to_string()))?;
        
        // Update our state to track the current directory
        // We do this AFTER set_current_dir succeeds, so both are in sync
        state.current_dir = target_path;
        Ok(())
    }
    
    fn name() -> &'static str {
        "cd"
    }
    
    fn help() -> &'static str {
        "cd: change directory"
    }
    
    fn validate_args(args: &[&str]) -> bool {
        !args.is_empty()
    }
}
