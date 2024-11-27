use std::io::{self};

/// Yes/No Question を行う
/// YesNo 型で返却する
#[derive(PartialEq)]
pub enum YesNo {
    Yes,
    No,
}
pub fn read_yes_or_no() -> YesNo {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return YesNo::Yes,
                "n" | "no" => return YesNo::No,
                _ => {
                    println!("無効な入力です。y または n を入力してください。");
                }
            },
            Err(e) => {
                eprintln!("入力の読み込みに失敗しました: {}", e);
                return YesNo::No;
            }
        }
    }
}
