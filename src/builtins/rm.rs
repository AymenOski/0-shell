use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::PathBuf;

pub struct Rm;

impl Command for Rm {
    fn execute(args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArgs("missing operand".to_string()));
        }

        let mut recursive = false;
        let mut targets: Vec<&str> = Vec::new();

        for arg in args {
            if *arg == "-r" || *arg == "-R" {
                recursive = true;
            } else if arg.starts_with('-') && *arg != "-" {
                return Err(CommandError::InvalidArgs(format!(
                    "invalid option -- '{}'",
                    arg.trim_start_matches('-')
                )));
            } else {
                targets.push(*arg);
            }
        }

        if targets.is_empty() {
            return Err(CommandError::InvalidArgs("missing operand".to_string()));
        }

        for target in targets {
            let path = resolve_path(target, _state)?;
            let metadata = fs::metadata(&path)
                .map_err(|e| io_error_to_cmd_error(target, e))?;

            if metadata.is_dir() {
                if recursive {
                    fs::remove_dir_all(&path)
                        .map_err(|e| io_error_to_cmd_error(target, e))?;
                } else {
                    return Err(CommandError::FileOperationFailed(
                        format!("cannot remove '{}': Is a directory", target)
                    ));
                }
            } else {
                fs::remove_file(&path)
                    .map_err(|e| io_error_to_cmd_error(target, e))?;
            }
        }

        Ok(())
    }
    
    fn name() -> &'static str {
        "rm"
    }
    
    fn help() -> &'static str {
        "rm: remove files or directories"
    }
}

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

fn io_error_to_cmd_error(target: &str, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => CommandError::FileOperationFailed(
            format!("cannot remove '{}': No such file or directory", target)
        ),
        std::io::ErrorKind::PermissionDenied => CommandError::FileOperationFailed(
            format!("cannot remove '{}': Permission denied", target)
        ),
        _ => CommandError::IOError(e.to_string()),
    }
}
