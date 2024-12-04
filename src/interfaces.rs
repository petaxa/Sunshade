use std::{fs::File, path::Path};

use reqwest::{self};

pub trait FileSystem {
    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), &'static str>;
    fn file_create(&self, path: &Path) -> Result<File, &'static str>;
    fn file_write_all(&self, file: &mut File, buf: &[u8]) -> Result<(), &'static str>;
    fn read(
        &self,
        // TODO: ここ、blocking 型に縛るのは間違い。.read() を持つほかの Response 型も受け付けられるようにしたい。
        response: &mut reqwest::blocking::Response,
        buf: &mut [u8],
    ) -> Result<usize, &'static str>;
}

pub trait CommandExecutor {
    fn run_command(&self, command: &str, args: &[&str]) -> Result<bool, &'static str>;
}

pub trait Io {
    fn println(&self, output: &str) -> Result<(), &'static str>;
    fn print(&self, output: &str) -> Result<(), &'static str>;
    fn read_line(&self, buf: &mut String) -> Result<(), &'static str>;
    fn flush(&self) -> Result<(), &'static str>;
}

pub trait Http {
    fn blocking_get(&self, url: &str) -> Result<reqwest::blocking::Response, &'static str>;
}
