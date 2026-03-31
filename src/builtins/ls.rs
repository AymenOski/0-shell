use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct Ls;

impl Command for Ls {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        let mut show_all = false;
        let mut classify = false;
        let mut path_buffer: Vec<&str> = Vec::new();

        for arg in args {
            if arg.starts_with('-') && arg.len() > 1 {
                for c in arg.chars().skip(1) {
                    match c {
                        'a' => show_all = true,
                        'F' => classify = true,
                        _ => {
                            return Err(CommandError::InvalidArgs(format!(
                                "ls: invalid option '{}'",
                                c
                            )))
                        }
                    }
                }
            } else {
                path_buffer.push(*arg);
            }
        }

        if path_buffer.is_empty() {
            path_buffer.push(".");
        }

        let has_multiple_paths = path_buffer.len() > 1;

        for (i, target) in path_buffer.iter().enumerate() {
            if i > 0 {
                println!();
            }

            let target_path = resolve_target(target, state)?;
            let metadata = fs::metadata(&target_path)
                .map_err(|e| io_error_to_cmd_error(target, e))?;

            if metadata.is_file() {
                let display = if classify {
                    let symlink_meta = fs::symlink_metadata(&target_path)
                        .map_err(|e| io_error_to_cmd_error(target, e))?;
                    classified_name(target, &symlink_meta)
                } else {
                    target.to_string()
                };
                println!("{}", display);
                continue;
            }

            if has_multiple_paths {
                println!("{}:", target);
            }

            let mut names = Vec::new();
            for entry in fs::read_dir(&target_path).map_err(|e| io_error_to_cmd_error(target, e))? {
                let entry = entry.map_err(|e| io_error_to_cmd_error(target, e))?;
                let name = entry.file_name().to_string_lossy().to_string();
                if !show_all && name.starts_with('.') {
                    continue;
                }
                names.push(name);
            }

            names.sort_by_key(|s| s.to_ascii_lowercase());

            for name in names {
                if classify {
                    let full_path = target_path.join(&name);
                    let metadata = fs::symlink_metadata(&full_path)
                        .map_err(|e| io_error_to_cmd_error(&name, e))?;
                    println!("{}", classified_name(&name, &metadata));
                } else {
                    println!("{}", name);
                }
            }
        }

        Ok(())
    }
    
    fn name() -> &'static str {
        "ls"
    }
    
    fn help() -> &'static str {
        "ls: list directory contents"
    }
}

fn classified_name(name: &str, meta: &fs::Metadata) -> String {
    let file_type = meta.file_type();
    let suffix = if file_type.is_dir() {
        '/'
    } else if file_type.is_symlink() {
        '@'
    } else if file_type.is_fifo() {
        '|'
    } else if file_type.is_socket() {
        '='
    } else if meta.permissions().mode() & 0o111 != 0 {
        '*'
    } else {
        '\0'
    };

    if suffix == '\0' {
        name.to_string()
    } else {
        format!("{}{}", name, suffix)
    }
}

fn resolve_target(target: &str, state: &ShellState) -> Result<PathBuf, CommandError> {
    if target.starts_with('~') {
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .ok_or(CommandError::InvalidArgs("Could not determine home directory".to_string()))?;

        if target == "~" {
            Ok(home)
        } else if target.starts_with("~/") {
            Ok(home.join(&target[2..]))
        } else {
            Err(CommandError::InvalidArgs(format!("Invalid path: {}", target)))
        }
    } else if target.starts_with('/') {
        Ok(PathBuf::from(target))
    } else {
        Ok(state.current_dir.join(target))
    }
}

fn io_error_to_cmd_error(target: &str, e: std::io::Error) -> CommandError {
    match e.kind() {
        std::io::ErrorKind::NotFound => CommandError::FileNotFound(target.to_string()),
        std::io::ErrorKind::PermissionDenied => CommandError::PermissionDenied(target.to_string()),
        std::io::ErrorKind::NotADirectory => CommandError::NotADirectory(target.to_string()),
        _ => CommandError::IOError(e.to_string()),
    }
}
