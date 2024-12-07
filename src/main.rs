use dotenvy::dotenv;
use std::{env, process::exit};

mod infra;
mod interfaces;
mod service;
use infra::{RealCommandExecutor, RealFileSystem, RealHttp, RealIo};

fn main() {
    // .env の読み込み
    dotenv().ok();

    let command_executor = RealCommandExecutor;
    let file_system = RealFileSystem;
    let io = RealIo;
    let http = RealHttp;

    let installer_path = service::download(&file_system, &io, &http).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    let eclipse_path =
        service::install(&command_executor, &io, &installer_path).unwrap_or_else(|e| {
            println!("{}", e);
            exit(1);
        });

    let repo_url = env::var("REPO_URL").expect("リポジトリの URL を取得できませんでした");
    service::clone(&command_executor, &file_system, &eclipse_path, &repo_url).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    service::start_eclipse(&eclipse_path).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
}
