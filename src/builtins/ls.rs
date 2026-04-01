use crate::CommandError;
use super::Command;
use crate::shell::state::ShellState;
use chrono::{DateTime, Local};
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

pub struct Ls;

#[derive(Default)]
struct Widths {
    links: usize,
    user: usize,
    group: usize,
    dev_major: usize,
    dev_minor: usize,
    has_device: bool,
    size: usize,
}

impl Command for Ls {
    fn execute(args: &[&str], state: &mut ShellState) -> Result<(), CommandError> {
        let mut show_all = false;
        let mut classify = false;
        let mut long = false;
        let mut path_buffer: Vec<&str> = Vec::new();

        for arg in args {
            if arg.starts_with('-') && arg.len() > 1 {
                for c in arg.chars().skip(1) {
                    match c {
                        'a' => show_all = true,
                        'F' => classify = true,
                        'l' => long = true,
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
                let entry_meta = fs::symlink_metadata(&target_path)
                    .map_err(|e| io_error_to_cmd_error(target, e))?;

                if long {
                    let widths = widths_for_entries(&[(target.to_string(), entry_meta.clone())]);
                    print_long_entry(target, &target_path, &entry_meta, classify, &widths);
                } else {
                    let display = if classify {
                        classified_name(target, &entry_meta)
                    } else {
                        target.to_string()
                    };
                    println!("{}", display);
                }
                continue;
            }

            if has_multiple_paths {
                println!("{}:", target);
            }

            let mut entries: Vec<(String, fs::Metadata)> = Vec::new();
            for entry in fs::read_dir(&target_path).map_err(|e| io_error_to_cmd_error(target, e))? {
                let entry = entry.map_err(|e| io_error_to_cmd_error(target, e))?;
                let name = entry.file_name().to_string_lossy().to_string();
                if !show_all && name.starts_with('.') {
                    continue;
                }

                let full_path = target_path.join(&name);
                let metadata = fs::symlink_metadata(&full_path)
                    .map_err(|e| io_error_to_cmd_error(&name, e))?;
                entries.push((name, metadata));
            }

            entries.sort_by(|(a, _), (b, _)| {
                let a_lower = a.to_lowercase();
                let b_lower = b.to_lowercase();
                a_lower.cmp(&b_lower)
            });

            let mut visible_entries: Vec<(String, fs::Metadata)> = Vec::new();
            if show_all {
                if let Ok(meta) = fs::metadata(&target_path) {
                    visible_entries.push((".".to_string(), meta));
                }
                if let Ok(meta) = fs::metadata(target_path.join("..")) {
                    visible_entries.push(("..".to_string(), meta));
                }
            }
            visible_entries.extend(entries);

            if long {
                let total_blocks: u64 = visible_entries.iter().map(|(_, m)| m.blocks()).sum();
                println!("total {}", total_blocks / 2);
                let widths = widths_for_entries(&visible_entries);

                for (name, metadata) in visible_entries {
                    let full_path = if name == "." {
                        target_path.clone()
                    } else if name == ".." {
                        target_path.join("..")
                    } else {
                        target_path.join(&name)
                    };
                    print_long_entry(&name, &full_path, &metadata, classify, &widths);
                }
                continue;
            }

            for (name, metadata) in visible_entries {
                if classify {
                    print!("{}  ", classified_name(&name, &metadata));
                } else {
                    print!("{}  ", name);
                }
            }
            println!();
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

fn widths_for_entries(entries: &[(String, fs::Metadata)]) -> Widths {
    let mut widths = Widths::default();
    let mut regular_size_width = 0;

    for (_, meta) in entries {
        widths.links = widths.links.max(meta.nlink().to_string().len());

        let user = users::get_user_by_uid(meta.uid())
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or(meta.uid().to_string());
        widths.user = widths.user.max(user.len());

        let group = users::get_group_by_gid(meta.gid())
            .map(|g| g.name().to_string_lossy().to_string())
            .unwrap_or(meta.gid().to_string());
        widths.group = widths.group.max(group.len());

        let file_type = meta.file_type();
        if file_type.is_block_device() || file_type.is_char_device() {
            let dev = meta.rdev();
            if dev != 0 {
                let major = libc::major(dev);
                let minor = libc::minor(dev);
                widths.has_device = true;
                widths.dev_major = widths.dev_major.max(major.to_string().len());
                widths.dev_minor = widths.dev_minor.max(minor.to_string().len());
            }
        } else {
            regular_size_width = regular_size_width.max(meta.len().to_string().len());
        }
    }

    let device_size_width = if widths.has_device {
        widths.dev_major + 2 + widths.dev_minor
    } else {
        0
    };
    widths.size = regular_size_width.max(device_size_width);

    widths
}

fn print_long_entry(name: &str, full_path: &Path, meta: &fs::Metadata, classify: bool, widths: &Widths) {
    let file_type = meta.file_type();
    let mode = meta.permissions().mode();

    let kind = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_socket() {
        's'
    } else {
        '-'
    };

    let perms = format!(
        "{}{}{}{}{}{}{}{}{}",
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    );

    let links = meta.nlink();
    let user = users::get_user_by_uid(meta.uid())
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or(meta.uid().to_string());
    let group = users::get_group_by_gid(meta.gid())
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or(meta.gid().to_string());
    let size = if file_type.is_block_device() || file_type.is_char_device() {
        let dev = meta.rdev();
        if dev != 0 {
            let major = libc::major(dev);
            let minor = libc::minor(dev);
            let device = format!(
                "{:>major_w$}, {:>minor_w$}",
                major,
                minor,
                major_w = widths.dev_major,
                minor_w = widths.dev_minor,
            );
            format!("{:>size_w$}", device, size_w = widths.size)
        } else {
            String::new()
        }
    } else {
        format!("{:>size_w$}", meta.len(), size_w = widths.size)
    };

    let epoch_seconds = meta.mtime().max(0) as u64;
    let system_time = UNIX_EPOCH + Duration::from_secs(epoch_seconds + 3600);
    let datetime: DateTime<Local> = system_time.into();
    let date = datetime.format("%b %e %H:%M");

    let mut display = if classify {
        classified_name(name, meta)
    } else {
        name.to_string()
    };

    if file_type.is_symlink() {
        if let Ok(target) = fs::read_link(full_path) {
            display = format!("{} -> {}", display, target.to_string_lossy());
        }
    }

    println!(
        "{}{} {:>links$} {:<user$} {:<group$} {} {} {}",
        kind,
        perms,
        links,
        user,
        group,
        size,
        date,
        display,
        links = widths.links,
        user = widths.user,
        group = widths.group,
    );
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
