use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct Rm;

impl Command for Rm {
    fn execute(args: &[&str], _state: &mut ShellState) -> Result<(), CommandError> {
        if args.is_empty() {
            return Err(CommandError::InvalidArgs("missing operand".to_string()));
        }

        let mut recursive = false;
        let mut paths: Vec<&str> = Vec::new();

        let limiter_idx = args.iter().position(|val| *val == "--");

        let options_slice = if let Some(idx) = limiter_idx {
            paths.extend_from_slice(&args[idx + 1..]);
            &args[..idx]
        } else {
            args
        };

        for operand in options_slice {
            match *operand {
                "-r" | "-R" => recursive = true,
                "---" => (),
                _ => {
                    if *operand != "-" && operand.starts_with('-') {
                        for ch in operand[1..].chars() {
                            if ch == '-' {
                                return Err(CommandError::InvalidArgs(format!(
                                    "unrecognized option '{}'",
                                    operand
                                )));
                            } else if ch != 'r' && ch != 'R' {
                                return Err(CommandError::InvalidArgs(format!(
                                    "invalid option -- '{}'",
                                    ch
                                )));
                            } else {
                                recursive = true;
                            }
                        }
                    } else {
                        paths.push(*operand);
                    }
                }
            }
        }

        if paths.is_empty() {
            return Err(CommandError::InvalidArgs("missing operand".to_string()));
        }

        let mut last_error: Option<CommandError> = None;

        for target in paths {
            if target == "."
                || target == ".."
                || target == "/"
                || target.ends_with("/.")
                || target.ends_with("/..")
                || target.ends_with("/./")
                || target.ends_with("/../")
            {
                last_error = Some(CommandError::FileOperationFailed(format!(
                    "refusing to remove '{}' directory",
                    target
                )));
                continue;
            }

            let path = resolve_path(target, _state)?;

            if !path.exists() {
                last_error = Some(CommandError::FileOperationFailed(format!(
                    "cannot remove '{}': No such file or directory",
                    target
                )));
                continue;
            }

            let metadata = match path.symlink_metadata() {
                Ok(m) => m,
                Err(err) => {
                    last_error = Some(CommandError::FileOperationFailed(format!(
                        "cannot access '{}': {}",
                        target,
                        map_io_error(&err)
                    )));
                    continue;
                }
            };

            if metadata.is_dir() {
                if recursive {
                    if let Err(err) = fs::remove_dir_all(&path) {
                        last_error = Some(CommandError::FileOperationFailed(format!(
                            "cannot remove '{}': {}",
                            target,
                            map_io_error(&err)
                        )));
                    }
                } else {
                    last_error = Some(CommandError::FileOperationFailed(format!(
                        "cannot remove '{}': Is a directory",
                        target
                    )));
                }
            } else {
                let is_write_protected = metadata.permissions().mode() & 0o200 == 0;
                if is_write_protected && !confirm_write_protected(target) {
                    continue;
                }

                if let Err(err) = fs::remove_file(&path) {
                    last_error = Some(CommandError::FileOperationFailed(format!(
                        "cannot remove '{}': {}",
                        target,
                        map_io_error(&err)
                    )));
                }
            }
        }

        if let Some(err) = last_error {
            return Err(err);
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

fn map_io_error(err: &std::io::Error) -> String {
    match err.kind() {
        std::io::ErrorKind::NotFound => "No such file or directory".to_string(),
        std::io::ErrorKind::PermissionDenied => "Permission denied".to_string(),
        _ => err.to_string(),
    }
}

fn confirm_write_protected(target: &str) -> bool {
    print!("rm: remove write-protected regular file '{}'? ", target);
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    matches!(input.trim(), "y" | "Y")
}
