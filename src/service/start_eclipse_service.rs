use std::process::Command;

/// Eclipse を実行する
pub fn start_eclipse(eclipse_path: &str) -> Result<(), &'static str> {
    // コマンド実行
    run_eclipse(&eclipse_path)?;

    return Ok(());
}

fn run_eclipse(eclipse_path: &str) -> Result<(), &'static str> {
    let eclipse_dir: String = format!("{}/eclipse/", eclipse_path);
    let workspace_dir: String = format!("{}/workspace/", eclipse_path);

    match Command::new(format!("{}/eclipse.exe", eclipse_dir))
        // Eclipse が cd に依存するためディレクトリ移動
        .current_dir(eclipse_dir)
        .arg("-data")
        .arg(workspace_dir)
        .spawn()
    {
        Ok(_) => {
            println!("Eclipse を起動しました");
            return Ok(());
        }
        Err(_) => return Err("Eclipse の起動に失敗しました"),
    }
}
