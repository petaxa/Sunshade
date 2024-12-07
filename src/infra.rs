use reqwest::{self};
use crate::interfaces::{CommandExecutor, FileSystem, Http, Io};
use std::{
    fs,
    fs::File,
    io,
    io::{Read, Write},
    path::Path,
    process::Command,
};

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), &'static str> {
        fs::create_dir_all(path).map_err(|_| "ディレクトリの作成に失敗しました")
    }

    fn file_create(&self, path: &Path) -> Result<File, &'static str> {
        fs::File::create(path).map_err(|_| "ファイルの作成に失敗しました")
    }

    fn file_write_all(&self, file: &mut File, buf: &[u8]) -> Result<(), &'static str> {
        file.write_all(buf)
            .map_err(|_| "ファイル書き込みに失敗しました")
    }

    fn read(
        &self,
        response: &mut reqwest::blocking::Response,
        buf: &mut [u8],
    ) -> Result<usize, &'static str> {
        response
            .read(buf)
            .map_err(|_| "バッファの読み取りに失敗しました")
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

pub struct RealIo;

impl Io for RealIo {
    fn print(&self, output: &str) -> Result<(), &'static str> {
        print!("{output}");
        Ok(())
    }

    fn println(&self, output: &str) -> Result<(), &'static str> {
        println!("{output}");
        Ok(())
    }

    fn read_line(&self, buf: &mut String) -> Result<(), &'static str> {
        io::stdin()
            .read_line(buf)
            .map_err(|_| "標準入力の読み取りに失敗しました")?;
        Ok(())
    }

    fn flush(&self) -> Result<(), &'static str> {
        io::stdout()
            .flush()
            .map_err(|_| "コマンドラインの flush に失敗しました")
    }
}

pub struct RealHttp;

impl Http for RealHttp {
    fn blocking_get(&self, url: &str) -> Result<reqwest::blocking::Response, &'static str> {
        reqwest::blocking::get(url).map_err(|_| "URL への GET の実行に失敗しました")
    }
}
