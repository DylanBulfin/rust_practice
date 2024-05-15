pub mod types;
pub mod paths;
use std::io::{self, BufRead, Write};

use types::State;

use crate::paths::get_abs_path;

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

        // Use the read-in value
        println!("Read in: {}", &buf);
        println!("Full path: {:?}", get_abs_path(&buf, &state));

        buf.clear();
    }
}
