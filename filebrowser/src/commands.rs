use std::{
    fmt::format,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    paths::get_canon_path,
    types::{ClipEntry, State},
};

use colored::{ColoredString, Colorize};

fn get_file_list(dir: &PathBuf) -> Vec<String> {
    let unsorted_paths = fs::read_dir(dir).unwrap();

    let mut paths = unsorted_paths
        .map(|x| String::from(x.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();
    paths.sort();

    paths
}

fn colorize_file(path_str: &String) -> ColoredString {
    if PathBuf::from(path_str.as_str()).is_dir() {
        path_str.blue().bold()
    } else {
        path_str.bright_cyan()
    }
}

fn print_file_list(files: Vec<String>) {
    if files.is_empty() {
        return;
    }

    let max_len = files.iter().map(String::len).max().unwrap();

    for c in files.chunks(2) {
        if c.len() == 1 {
            println!("{}", colorize_file(&c[0]))
        } else {
            let pad = max_len - c[0].len();
            print!("{}    ", colorize_file(&c[0]));
            print!("{:indent$}{}\n", "", colorize_file(&c[1]), indent = pad);
        }
    }
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

pub fn list_files(tail: Vec<String>, state: &State) -> Result<(), String> {
    if tail.len() != 0 {
        return Err(String::from("This command requires exactly 0 arguments"));
    }

    print_file_list(get_file_list(state.get_cwd()));

    Ok(())
}

pub fn remove_files(tail: Vec<String>, state: &State) -> Result<(), String> {
    let mut remove_list: Vec<(PathBuf, String)> = Vec::new();

    for f in tail {
        if let Ok(path) = get_canon_path(f.as_str(), state) {
            if path.is_dir() {
                return Err(String::from(
                    "Only file deletion supported currently, aborting",
                ));
            }

            remove_list.push((path, f));
        } else {
            return Err(String::from(
                "Error getting full path. File likely doesn't exist",
            ));
        }
    }

    for (f, name) in remove_list {
        if let Ok(()) = fs::remove_file(f) {
            println!("Successfully removed file {}", name);
        } else {
            return Err(format!("Unable to delete file {}", name));
        };
    }

    Ok(())
}

fn add_to_clip(tail: Vec<String>, state: &mut State, is_cut: bool) -> Result<(), String> {
    let mut copy_list: Vec<(PathBuf, String)> = Vec::new();

    for f in tail {
        if let Ok(path) = get_canon_path(f.as_str(), state) {
            if path.is_dir() {
                return Err(String::from(
                    "Only file deletion supported currently, aborting",
                ));
            }

            copy_list.push((path, f.to_string()));
        } else {
            return Err(String::from(
                "Error getting full path. File likely doesn't exist",
            ));
        }
    }

    for (path, name) in copy_list {
        state.add_to_clip(path, name, is_cut);
    }

    Ok(())
}

pub fn copy(tail: Vec<String>, state: &mut State) -> Result<(), String> {
    add_to_clip(tail, state, false)
}

pub fn cut(tail: Vec<String>, state: &mut State) -> Result<(), String> {
    add_to_clip(tail, state, true)
}

pub fn paste(tail: Vec<String>, state: &mut State) -> Result<(), String> {
    if tail.len() != 0 {
        return Err(String::from("This command requires exactly 0 arguments"));
    }

    let clip = state.clear_clip();

    for e in clip {
        let mut new_file= PathBuf::from(state.get_cwd_str());
        new_file.push(e.get_path().file_name().unwrap());
        if let Err(_) = fs::copy(e.get_path(), new_file) {
            return Err(format!("Failed to copy file {}, aborting", e.get_name()));
        }

        if e.is_cut() {
            if let Err(_) = fs::remove_file(e.get_path()) {
                return Err(format!("Failed to clean old version: {}", e.get_name()));
            }

            println!(
                "Successfully moved file {} to {}",
                e.get_name(),
                state.get_cwd_str()
            );
        } else {
            println!(
                "Successfully copied file {} to {}",
                e.get_name(),
                state.get_cwd_str()
            );
        }
    }

    Ok(())
}

pub fn quit(tail: Vec<String>, quit_flag: &mut bool) -> Result<(), String> {
    if tail.len() != 0 {
        return Err(String::from("This command requires exactly 0 arguments"));
    }

    *quit_flag = true;

    Ok(())
}
