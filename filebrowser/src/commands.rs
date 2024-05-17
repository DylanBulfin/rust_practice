use std::{fs, path::PathBuf};

use crate::{paths::get_canon_path, types::State};

fn get_file_list(dir: &PathBuf) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();

    paths
        .map(|x| String::from(x.unwrap().path().to_str().unwrap()))
        .collect()
}

fn print_file_list(files: Vec<String>) {
    println!("");
    for c in files.chunks(2) {
        if c.len() == 1 {
            println!("{}", c[0])
        } else {
            println!("{}    {}", c[0], c[1])
        }
    }
    println!("");
}

pub fn change_directory(tail: Vec<String>, state: &mut State) -> Result<(), String> {
    if tail.len() != 1 {
        return Err(String::from("This command requires exactly 1 argument"));
    }

    if let Ok(path) = get_canon_path(tail[0].as_str(), &state) {
        print_file_list(get_file_list(&path));
        state.set_cwd(path);
    } else {
        return Err(String::from(
            "Error getting full path. Dir likely doesn't exist",
        ));
    }

    Ok(())
}

pub fn quit(tail: Vec<String>, state: &mut State, quit_flag: &mut bool) -> Result<(), String> {
    if tail.len() != 0 {
        return Err(String::from("This command requires exactly 0 arguments"));
    }

    *quit_flag = true;

    Ok(())
}
