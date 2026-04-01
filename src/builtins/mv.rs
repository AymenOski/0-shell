use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Mv;

impl Command for Mv {
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
            let source = resolve_path(src_name, state)?;

            if dest.is_dir() {
                let Some(name) = source.file_name() else {
                    return Err(CommandError::FileOperationFailed(format!(
                        "invalid source path '{}'",
                        source.display()
                    )));
                };

                let dest_file = dest.join(name);
                return move_one(&source, &dest_file);
            }

            return move_one(&source, &dest);
        }

        if !dest.is_dir() {
            return Err(CommandError::FileOperationFailed(format!(
                "target '{}' is not a directory",
                dest.display()
            )));
        }

        let mut last_error: Option<CommandError> = None;

        for src_name in sources {
            let source = resolve_path(src_name, state)?;

            let Some(name) = source.file_name() else {
                last_error = Some(CommandError::FileOperationFailed(format!(
                    "invalid source path '{}'",
                    source.display()
                )));
                continue;
            };

            let dest_file = dest.join(name);

            if let Err(err) = move_one(&source, &dest_file) {
                last_error = Some(err);
            }
        }

        if let Some(err) = last_error {
            return Err(err);
        }

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

fn move_one(source: &Path, dest: &Path) -> Result<(), CommandError> {
    if !source.exists() {
        return Err(CommandError::FileOperationFailed(format!(
            "cannot stat '{}': No such file or directory",
            source.display()
        )));
    }

    if dest.exists() && dest.is_dir() && !source.is_dir() {
        return Err(CommandError::FileOperationFailed(format!(
            "cannot overwrite directory '{}' with non-directory",
            dest.display()
        )));
    }

    if dest.exists() && !dest.is_dir() {
        fs::remove_file(dest).map_err(|e| {
            CommandError::FileOperationFailed(format!(
                "cannot remove '{}': {}",
                dest.display(),
                e
            ))
        })?;
    }

    fs::rename(source, dest).map_err(|e| {
        CommandError::FileOperationFailed(format!(
            "cannot move '{}' to '{}': {}",
            source.display(),
            dest.display(),
            e
        ))
    })?;

    Ok(())
}
