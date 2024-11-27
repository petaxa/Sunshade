use std::{env, path::Path, process::Command};

/// workspace ディレクトリにソースコードをクローンする
pub fn clone(eclipse_path: &str) -> Result<(), &'static str> {
    let workspace_dir: String = format!("{}/workspace", eclipse_path);

    // workspace ディレクトリを作成
    match make_workspace(&workspace_dir) {
        Ok(_) => {}
        Err(_) => return Err("workspace ディレクトリの作成に失敗しました"),
    };

    // git コマンドの存在確認
    match Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
    {
        Ok(_) => {}
        Err(_) => return Err("git コマンドが利用できません"),
    }

    // ソースコードのクローン
    match do_clone(&workspace_dir) {
        Ok(_) => {},
        Err(_) => return Err("リポジトリのクローンに失敗しました")
    };
    return Ok(());
}

/// workspace ディレクトリを作成する
fn make_workspace(workspace_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&workspace_dir).exists() {
        Command::new("mkdir").arg(&workspace_dir).status()?;
    }
    Ok(())
}

fn do_clone(workspace_dir: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let url = env::var("REPO_URL").expect("リポジトリの URL を取得できませんでした");
    let repo_name = extract_repo_name(&url).ok_or_else(|| "リポジトリ名を抽出できませんでした")?;

    let args = ["clone", &url, &format!("{}/{}", &workspace_dir, &repo_name)];
    let result = Command::new("git")
        .args(&args)
        .output()
        .map(|output| output.status.success())?;

    return Ok(result);
}

/// リポジトリ URL からリポジトリ名を抽出する
fn extract_repo_name(repo_url: &str) -> Option<String> {
    let repo_name = repo_url
        .trim_end_matches('/')
        .split('/')
        .last()?
        .trim_end_matches(".git");

    Some(repo_name.to_string())
}
