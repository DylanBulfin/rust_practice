use std::{
    fs::{self, File},
    path::PathBuf,
    process::Command
};

use crate::{
    paths::{get_canon_path, get_file_name},
    types::State,
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
    let mut remove_list: Vec<PathBuf> = Vec::new();

    for f in tail {
        if let Ok(path) = get_canon_path(f.as_str(), state) {
            if path.is_dir() {
                return Err(String::from(
                    "Only file deletion supported currently, aborting",
                ));
            }

            remove_list.push(path);
        } else {
            return Err(String::from(
                "Error getting full path. File likely doesn't exist",
            ));
        }
    }

    for f in remove_list {
        let fname = get_file_name(&f);
        if let Ok(()) = fs::remove_file(f) {
            println!("Successfully removed file {}", fname);
        } else {
            return Err(format!("Unable to delete file {}", fname));
        };
    }

    Ok(())
}

fn add_to_clip(tail: Vec<String>, state: &mut State, is_cut: bool) -> Result<(), String> {
    let mut copy_list: Vec<PathBuf> = Vec::new();

    for f in tail {
        if let Ok(path) = get_canon_path(f.as_str(), state) {
            if path.is_dir() {
                return Err(String::from(
                    "Only file deletion supported currently, aborting",
                ));
            }

            copy_list.push(path);
        } else {
            return Err(String::from(
                "Error getting full path. File likely doesn't exist",
            ));
        }
    }

    for path in copy_list {
        state.add_to_clip(path, is_cut);
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
        let mut new_file = PathBuf::from(state.get_cwd_str());
        new_file.push(e.get_path().file_name().unwrap());

        let fname = get_file_name(&e.get_path());
        if let Err(_) = fs::copy(e.get_path(), new_file) {
            return Err(format!("Failed to copy file {}, aborting", fname));
        }

        if e.is_cut() {
            if let Err(_) = fs::remove_file(e.get_path()) {
                return Err(format!("Failed to clean old version: {}", fname));
            }

            println!(
                "Successfully moved file {} to {}",
                fname,
                state.get_cwd_str()
            );
        } else {
            println!(
                "Successfully copied file {} to {}",
                fname,
                state.get_cwd_str()
            );
        }
    }

    Ok(())
}

pub fn rename(tail: Vec<String>, state: &State) -> Result<(), String> {
    if tail.len() != 2 {
        return Err(String::from("This command requires exactly 2 arguments"));
    }

    if let Ok(old_file) = get_canon_path(tail[0].as_str(), state) {
        if let Err(_) = fs::rename(old_file, PathBuf::from(tail[1].as_str())) {
            return Err(String::from("Rename failed"));
        }
    } else {
        return Err(String::from(
            "Error getting full path. File likely doesn't exist",
        ));
    }

    Ok(())
}

pub fn new_files(tail: Vec<String>, _: &State) -> Result<(), String> {
    if tail.len() < 1 {
        return Err(String::from("This command requires 1 or more arguments"));
    }

    for f in tail {
        if let Some(dir) = PathBuf::from(f.as_str()).parent() {
            if let Ok(mut path) = dir.canonicalize() {
                if let Some(fname) = PathBuf::from(f.as_str()).file_name() {
                    path.push(fname);
                    if let Err(_) = File::create_new(path.as_path()) {
                        return Err(format!("Unable to create file {}", get_file_name(&path)));
                    }
                } else {
                    return Err(format!("{} is not a file, use nd to create directories", f));
                }
            } else {
                return Err(format!(
                    "Error finding directory for argument {}. Likely doesn't exist",
                    f
                ));
            }
        } else {
            return Err(String::from("No parent for given path"));
        }
    }

    Ok(())
}

pub fn new_dirs(tail: Vec<String>, _: &State)-> Result<(), String>{
    if tail.len() < 1 {
        return Err(String::from("This command requires 1 or more arguments"));
    }

    for d in tail {
        let path = PathBuf::from(d.as_str());
        
        if let Some(child) = path.components().rev().next() {
            if let Some(dir) = PathBuf::from(d.as_str()).parent() {
                if let Ok(mut path) = dir.canonicalize() {
                    path.push(child);
                    if let Err(_) = fs::create_dir(path) {
                        return Err(format!("Unable to create directory {}", d));
                    }
                } else {
                    return Err(format!("Can't find parent directory for argument {}", d))
                }
            } else {
                return Err(String::from("No parent for given path"));
            }
        } else {
            return Err(String::from("Failed to find last component of path"));
        }
    }

    Ok(())
}

pub fn open_editor(tail: Vec<String>, state: &State) -> Result<(), String> {
    let mut paths: Vec<PathBuf> = Vec::new();

    for f in tail.as_slice() {
        if let Ok(path) = get_canon_path(f.as_str(), state) {
            if path.is_dir() {
                return Err(format!("{} is not a file, aborting", f));
            }

            paths.push(path);
        } else {
            return Err(format!(
                "Unable to process argument {}, file likely doesn't exist",
                f
            ));
        }
    }

    if let Ok(editor) = std::env::var("EDITOR") {
        let mut editor_comm = Command::new(editor);
        editor_comm.args(tail);

        if let Err(_) = editor_comm.status() {
            return Err(String::from(
                "Unknown error opening editor, make sure $EDITOR is set correctly",
            ));
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
