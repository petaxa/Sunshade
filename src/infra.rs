use std::fs;
use std::process::Command;
use crate::interfaces::{FileSystem, CommandExecutor};

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir(&self, path: &str) -> Result<(), &'static str> {
        if std::path::Path::new(path).exists() {
            Ok(())
        } else {
            fs::create_dir_all(path).map_err(|_| "ディレクトリの作成に失敗しました")
        }
    }
}

pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn run_command(&self, command: &str, args: &[&str]) -> Result<bool, &'static str> {
        let status = Command::new(command).args(args).status();
        match status {
            Ok(status) => Ok(status.success()),
            Err(_) => Err("コマンドの実行に失敗しました"),
        }
    }
}
