use crate::interfaces::{CommandExecutor, FileSystem};

/// workspace ディレクトリにソースコードをクローンする
pub fn clone(
    command_executor: &dyn CommandExecutor,
    file_system: &dyn FileSystem,
    eclipse_path: &str,
    repo_url: &str,
) -> Result<(), &'static str> {
    let workspace_dir: String = format!("{}/workspace", eclipse_path);

    // workspace ディレクトリを作成
    match file_system.create_dir(&workspace_dir) {
        Ok(_) => {}
        Err(_) => return Err("workspace ディレクトリの作成に失敗しました"),
    };

    // git コマンドの存在確認
    match command_executor.run_command("git", &["--version"]) {
        Ok(_) => {}
        Err(_) => return Err("git コマンドが利用できません"),
    }

    // ソースコードのクローン
    let repo_dir: String = match make_repo_dir(&workspace_dir, &repo_url) {
        Some(value) => value,
        None => return Err("リポジトリディレクトリ名の作成に失敗しました"),
    };
    match command_executor.run_command("git", &["clone", &repo_url, &repo_dir]) {
        Ok(_) => {}
        Err(_) => return Err("リポジトリのクローンに失敗しました"),
    };
    return Ok(());
}

/// リポジトリのディレクトリパスを成形する
///
/// 1. リポジトリ URL から リポジトリ名を抽出
/// 2. workspace ディレクトリパスと統合
fn make_repo_dir(workspace_dir: &str, repo_url: &str) -> Option<String> {
    let repo_name: &str = repo_url
        .trim_end_matches('/')
        .split('/')
        .last()?
        .trim_end_matches(".git");

    Some(format!("{}/{}", &workspace_dir, &repo_name))
}
