use crate::interfaces::Io;

/// Yes/No Question を行う
/// YesNo 型で返却する
#[derive(PartialEq)]
pub enum YesNo {
    Yes,
    No,
}
pub fn read_yes_or_no<IO: Io>(io: &IO) -> YesNo {
    let mut input = String::new();
    loop {
        match io.read_line(&mut input) {
            Ok(_) => match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return YesNo::Yes,
                "n" | "no" => return YesNo::No,
                _ => {
                    io.println("無効な入力です。y または n を入力してください。").unwrap();
                }
            },
            Err(_) => {
                io.println("入力の読み込みに失敗しました").unwrap();
                return YesNo::No;
            }
        }
    }
}
