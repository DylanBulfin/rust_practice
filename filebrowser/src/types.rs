use std::path::PathBuf; 
use std::env::var;

pub struct ClipEntry {
    path: PathBuf,
    is_cut: bool,
}

pub struct State {
    cwd: PathBuf,
    clip: Vec<ClipEntry>
}

impl ClipEntry {
    pub fn new(path: &str, is_cut: bool) -> Self{
        ClipEntry {
            path : PathBuf::from(path),
            is_cut
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            cwd: PathBuf::from(var("HOME").unwrap_or(String::from(r"/"))), // if no $HOME, start at root
            clip: Vec::new()
        }
    }

    pub fn get_cwd_str(&self) -> String {
        String::from(self.cwd.to_str().unwrap())
    }

    pub fn get_cwd(&self) -> &PathBuf {
        &self.cwd
    }

    pub fn set_cwd(&mut self, path: PathBuf) {
        self.cwd = path;
    }

    pub fn add_to_clip(&mut self, path: &str, is_cut: bool) {
        let entry = ClipEntry::new(path, is_cut);
        self.clip.push(entry);
    }

    pub fn clear_clip(&mut self) -> Vec<ClipEntry> {
        self.clip.drain(..).collect()
    }
}
