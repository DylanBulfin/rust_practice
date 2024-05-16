mod types;
mod paths;
mod commands; 
use std::io::{self, BufRead, Write};


use types::State;

use crate::paths::get_canon_path;

fn main() -> io::Result<()> {
    let mut state = State::new();
    let mut buf = String::with_capacity(2048);

    loop {
        //print a prompt
        print!("{}> ", &state.get_cwd());

        io::stdout().flush()?;

        let mut stdin = io::stdin().lock();

        stdin.read_line(&mut buf)?;

        // Removes trailing newline
        buf.pop();

        println!("{:?}", com)

        buf.clear();
    }
}
