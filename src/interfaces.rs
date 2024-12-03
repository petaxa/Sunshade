pub trait FileSystem {
    fn create_dir(&self, path: &str) -> Result<(), &'static str>;
}

pub trait CommandExecutor {
    fn run_command(&self, command: &str, args: &[&str]) -> Result<bool, &'static str>;
}
