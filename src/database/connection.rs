use std::{fs, path::PathBuf};

const DEFAULT_DB_PATH: &str = "~/.n/notes.db";

pub fn get_db_connection(db_path: Option<String>) -> rusqlite::Result<rusqlite::Connection> {
    let db_path = ensure_db_path(db_path);
    rusqlite::Connection::open(&db_path)
}

fn ensure_db_path(db_path: Option<String>) -> PathBuf {
    if let Some(path_str) = db_path {
        PathBuf::from(path_str)
    } else {
        let expanded_path = shellexpand::tilde(DEFAULT_DB_PATH);
        let default_path = PathBuf::from(expanded_path.as_ref());
        fs::create_dir_all(default_path.parent().unwrap()).ok();
        default_path
    }
}
