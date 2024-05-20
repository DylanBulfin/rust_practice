use std::path::PathBuf;

use crate::types::State;

pub fn get_file_name(path: &PathBuf) -> String {
    String::from(path.to_str().unwrap())
}

pub fn get_canon_path(path: &str, state: &State) -> std::io::Result<PathBuf> {
    let mut cwd = PathBuf::from(state.get_cwd());
    cwd.push(path);
    cwd.canonicalize()
}
