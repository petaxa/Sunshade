use crate::interfaces::{CommandExecutor, Io};
use std::env;

use super::utils::{self};

struct Config {
    default_eclipse_dir: String,
}

impl Config {
    fn new() -> Self {
        Self {
            default_eclipse_dir: env::var("DEFAULT_ECLIPSE_DIR")
                .expect("Eclipse のデフォルトダウンロードパスが環境変数に設定されていません"),
        }
    }
}

/// ダウンロードしたインストーラを実行する
pub fn install<CE: CommandExecutor, IO: Io>(
    command_executor: &CE,
    io: &IO,
    installer_path: &str,
) -> Result<String, &'static str> {
    let config: Config = Config::new();

    // 解凍先フォルダ指定
    let eclipse_dir = confirm_eclipse_path(io, config.default_eclipse_dir)?;

    // コマンド実行
    run_installer(command_executor, &eclipse_dir, installer_path)?;

    return Ok(eclipse_dir);
}

/// 解凍先のフォルダを確定
fn confirm_eclipse_path<IO: Io>(io: &IO, default_path: String) -> Result<String, &'static str> {
    let mut path = default_path;
    io.println(&format!("{} にEclipse をインストールします。インストール先を変更せずにインストールを行いますか？[y/n]",
        path))?;

    let input = utils::read_yes_or_no(io);
    // No の場合はファイルパスをもらう
    if input == utils::YesNo::No {
        io.println("インストール先のフルパスを入力してください。")?;
        path.clear();
        io.read_line(&mut path)
            // ファイルパスの形式になっているかチェックしたいね。
            .expect("インストール先パスの読み込みに失敗しました。");
    }

    return Ok(path);
}

/// インストーラを実行
fn run_installer<CE: CommandExecutor>(
    command_executor: &CE,
    eclipse_dir: &str,
    installer_path: &str,
) -> Result<String, &'static str> {
    let args = ["-s2", &format!("-d{}", &eclipse_dir)];

    match command_executor.run_command(installer_path, &args) {
        Ok(_) => return Ok(eclipse_dir.to_string()),
        Err(_) => return Err("インストールに失敗しました"),
    }
}
