use reqwest::blocking::get;
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

struct Config {
    download_dir: String,
    default_vesion: &'static str,
}

impl Config {
    fn new() -> Self {
        Self {
            download_dir: env::var("DEFAULT_INSTALLER_DIR")
                .expect("インストーラのデフォルトダウンロードパスが環境変数に設定されていません"),
            default_vesion: "pleiades-2024-09-java-win-64bit-jre_20240917",
        }
    }
}

/// Eclipse のインストーラをダウンロードする
///
/// 対話形式でバージョンを指定でき、
/// 同一ファイルが存在する場合はそれを使ってインストール作業へ移行することもできる
pub fn download() -> Result<String, &'static str> {
    let config: Config = Config::new();

    // ダウンロードするバージョンを確定
    let version = confirm_version(config.default_vesion.to_string())?;

    // ダウンロード予定のファイルと同一のファイルが存在するか確認
    let file_path = format!("{}/{}.exe", config.download_dir, version);
    if Path::new(&file_path).exists() && confirm_use_existing_file(&file_path)? {
        return Ok(file_path);
    }

    // ファイルをダウンロード
    println!("{}.exe をダウンロードします。", version);
    let url = construct_download_url(version)?;
    match do_download(&url, &file_path) {
        Ok(_) => return Ok(file_path),
        Err(_) => return Err("ダウンロードに失敗"),
    }
}

/// ダウンロードするインストーラのバージョンを決定
///
/// デフォルトでない場合、入力を行う
fn confirm_version(default_varsion: String) -> Result<String, &'static str> {
    let mut version = default_varsion;
    println!(
        "バージョン {} をダウンロードします。よろしいですか？[y/n]",
        version
    );
    let input = super::utils::read_yes_or_no();

    // No の場合はバージョンをもらう
    if input == super::utils::YesNo::No {
        println!(
                "ダウンロードしたいバージョンを入力してください。ex. pleiades-2024-09-java-win-64bit-jre_20240917"
            );
        version.clear();
        io::stdin()
            .read_line(&mut version)
            .expect("バージョンの読み込みに失敗しました");
    }
    return Ok(version);
}

/// 既存のインストーラを利用するか確認
fn confirm_use_existing_file(file_path: &str) -> Result<bool, &'static str> {
    println!(
        "既に {} が存在します。このファイルを利用してインストール作業を続けますか？[y/n]",
        file_path
    );
    let input = super::utils::read_yes_or_no();
    match input {
        super::utils::YesNo::Yes => return Ok(true),
        super::utils::YesNo::No => {
            println!("セットアップ作業をキャンセルします。");
            return Err("既存ファイル解凍拒否");
        }
    }
}

/// ダウンロード URL を構築する
fn construct_download_url(version: String) -> Result<String, &'static str> {
    const FIXED_URL: &str = "https://ftp.jaist.ac.jp/pub/mergedoc/pleiades";
    let year = version.split("-").nth(1).ok_or("バージョンが不正")?;

    let url = format!("{}/{}/{}.exe", FIXED_URL, year, version);
    return Ok(url);
}

/// ダウンロード作業ロジックの本体
///
/// ダウンロード先ディレクトリの作成、進捗の表示、ファイルのダウンロードと書き込みを行う
fn do_download(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: エラーメッセージを詳しく出力できるようにする。
    // それぞれにmatch 式をつけるとか？

    let file_path = Path::new(file_path);

    // URLからファイルを取得
    let mut response = get(url)?;

    // 親ディレクトリが存在しない場合、再帰的にディレクトリを作成
    if let Some(parent_dir) = file_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }
    }
    // ファイル作成
    let mut file = File::create(file_path)?;

    // レスポンスの内容をバッファに入れて1バイトずつ書き込み
    {
        let total_size = response
            .content_length()
            .ok_or("ファイルサイズを取得できませんでした")?;
        let mut downloaded: u64 = 0;

        let mut buffer = [0; 1024];
        while let Ok(bytes_read) = response.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read as u64;

            // 進行状況の表示
            let per = downloaded as f64 / total_size as f64 * 100.0;
            print!("\rダウンロード進行状況: {:.2}%", per);

            // TODO: progress bar を実装したい気持ち。
            // よさそうなライブラリを探そう

            io::stdout().flush().unwrap();
        }
        print!("\n");
    }

    Ok(())
}
