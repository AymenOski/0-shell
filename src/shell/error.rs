use crate::CommandError;

pub fn format_error(cmd_name: &str, err: &CommandError) -> String {
	match err {
		CommandError::CommandNotFound(name) => format!("Command '{}' not found", name),
		CommandError::FileNotFound(path) => {
			format!("{}: {}: No such file or directory", cmd_name, path)
		}
		CommandError::PermissionDenied(path) => {
			format!("{}: {}: Permission denied", cmd_name, path)
		}
		CommandError::IsADirectory(path) => {
			format!("{}: {}: Is a directory", cmd_name, path)
		}
		CommandError::NotADirectory(path) => {
			format!("{}: {}: Not a directory", cmd_name, path)
		}
		CommandError::AlreadyExists(path) => {
			format!("{}: cannot create directory '{}': File exists", cmd_name, path)
		}
		CommandError::FileOperationFailed(msg) => format!("{}: {}", cmd_name, msg),
		CommandError::IOError(msg) => format!("{}: {}", cmd_name, msg),
		CommandError::InvalidArgs(msg) => format!("{}: {}", cmd_name, msg),
	}
}
