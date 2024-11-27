use std::{env, io, process::Command};

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
pub fn install(installer_path: &str) -> Result<String, &'static str> {
    let config: Config = Config::new();

    // 解凍先フォルダ指定
    let eclipse_dir = confirm_eclipse_path(config.default_eclipse_dir)?;

    // コマンド実行
    run_installer(&eclipse_dir, installer_path)?;

    return Ok(eclipse_dir);
}

/// 解凍先のフォルダを確定
fn confirm_eclipse_path(default_path: String) -> Result<String, &'static str> {
    let mut path = default_path;
    println!(
        "{} にEclipse をインストールします。インストール先を変更せずにインストールを行いますか？[y/n]",
        path
    );

    let input = super::utils::read_yes_or_no();
    // No の場合はファイルパスをもらう
    if input == super::utils::YesNo::No {
        println!("インストール先のフルパスを入力してください。");
        path.clear();
        io::stdin()
            .read_line(&mut path)
            // ファイルパスの形式になっているかチェックしたいね。
            .expect("インストール先パスの読み込みに失敗しました。");
    }

    return Ok(path);
}

/// インストーラを実行
fn run_installer(eclipse_dir: &str, installer_path: &str) -> Result<String, &'static str> {
    let args = ["-s2", &format!("-d{}", &eclipse_dir)];

    match Command::new(installer_path).args(&args).status() {
        Ok(_) => return Ok(eclipse_dir.to_string()),
        Err(_) => return Err("インストールに失敗しました"),
    }
}
