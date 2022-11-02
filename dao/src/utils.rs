pub mod utils {
    pub use dotenv::dotenv;
    pub use std::collections::HashMap;
    pub use std::{env, path::Path, path::PathBuf};

    pub fn load_env_file() -> HashMap<String, String> {
        dotenv().ok();
        let env_vars: HashMap<String, String> = env::vars()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        env_vars
    }

    pub fn get_env_var(var: &str) -> Option<String> {
        let vars = load_env_file();
        Some(vars.get(&var.to_string())?.to_string())
    }

    pub fn get_full_db_path() -> Option<PathBuf> {
        let mut db_dir = get_env_var(&"DB_FILE_PATH".to_string())
            .unwrap_or("ERROR: Unknown database path".to_string());
        db_dir = format!("../{}", db_dir.to_string());
        Some(Path::join(
            env::current_dir().unwrap().as_path(),
            Path::new(&db_dir).to_str().unwrap(),
        ))
    }
}
