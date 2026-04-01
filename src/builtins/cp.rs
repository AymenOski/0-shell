use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Cp;

impl Command for Cp {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArgs("missing file operand".to_string()));
        }
        if args.len() == 1 {
            return Err(CommandError::InvalidArgs(format!("missing destination file operand after '{}'", args[0])));
        }

        let dest_name = args[args.len() - 1];
        let sources = &args[..args.len() - 1];
        let dest = resolve_path(dest_name, state)?;

        if sources.len() == 1 {
            let src_name = sources[0];
            let src = resolve_path(src_name, state)?;

            if dest.is_dir() {
                let Some(name) = src.file_name() else {
                    return Err(CommandError::FileOperationFailed(format!(
                        "invalid source path '{}'",
                        src_name
                    )));
                };

                let dest_file = dest.join(name);
                let dest_display = join_display_path(dest_name, name);
                return copy_one(&src, &dest_file, src_name, &dest_display);
            }

            return copy_one(&src, &dest, src_name, dest_name);
        }

        if !dest.is_dir() {
            return Err(CommandError::FileOperationFailed(format!(
                "target '{}' is not a directory",
                dest_name
            )));
        }

        let mut last_error: Option<CommandError> = None;

        for src_name in sources {
            let src = resolve_path(src_name, state)?;

            let Some(name) = src.file_name() else {
                last_error = Some(CommandError::FileOperationFailed(format!(
                    "invalid source path '{}'",
                    src_name
                )));
                continue;
            };

            let dest_file = dest.join(name);
            let dest_display = join_display_path(dest_name, name);

            if let Err(err) = copy_one(&src, &dest_file, src_name, &dest_display) {
                last_error = Some(err);
            }
        }

        if let Some(err) = last_error {
            return Err(err);
        }
        
        Ok(())
    }
    
    fn name() -> &'static str {
        "cp"
    }
    
    fn help() -> &'static str {
        "cp: copy files"
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

fn copy_one(src: &Path, dest: &Path, src_display: &str, dest_display: &str) -> Result<(), CommandError> {
    if !src.exists() {
        return Err(CommandError::FileOperationFailed(format!(
            "cannot stat '{}': No such file or directory",
            src_display
        )));
    }

    if src.is_dir() {
        return Err(CommandError::FileOperationFailed(format!(
            "omitting directory '{}'",
            src_display
        )));
    }

    fs::copy(src, dest)
        .map_err(|e| io_error_to_cmd_error(src_display, dest_display, e))?;

    Ok(())
}

fn io_error_to_cmd_error(src_name: &str, dest_name: &str, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            CommandError::FileOperationFailed(format!("cannot stat '{}': No such file or directory", src_name))
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

fn join_display_path(base: &str, child: &std::ffi::OsStr) -> String {
    Path::new(base).join(child).display().to_string()
}
