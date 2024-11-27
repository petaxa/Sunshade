use dotenvy::dotenv;
use std::process::exit;

mod service;

fn main() {
    // .env の読み込み
    dotenv().ok();

    let installer_path = service::download().unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    let eclipse_path = service::install(&installer_path).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    service::clone(&eclipse_path).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    service::start_eclipse(&eclipse_path).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
}
