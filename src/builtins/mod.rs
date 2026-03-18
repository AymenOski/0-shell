use crate::CommandError;

pub fn echo(args: &[&str]) -> Result<(), CommandError>;
pub fn cat(args: &[&str]) -> Result<(), CommandError>;
pub fn cd(args: &[&str]) -> Result<(), CommandError>;
pub fn cp(args: &[&str]) -> Result<(), CommandError>;
pub fn exit(args: &[&str]) -> Result<(), CommandError>;
pub fn ls(args: &[&str]) -> Result<(), CommandError>;
pub fn mkdir(args: &[&str]) -> Result<(), CommandError>;
pub fn mv(args: &[&str]) -> Result<(), CommandError>;
pub fn pwd(args: &[&str]) -> Result<(), CommandError>;
pub fn rm(args: &[&str]) -> Result<(), CommandError>;

