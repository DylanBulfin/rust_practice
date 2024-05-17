mod types;
mod paths;
mod commands; 
mod parser;
use std::io::{self, BufRead, Write};


use parser::parse_command;
use parser::Command;
use paths::get_canon_path;
use types::State;

fn main() -> io::Result<()> {
    let mut state = State::new();
    let mut buf = String::with_capacity(2048);

    let mut quit_flag = false;

    loop {
        //print a prompt
        print!("{}> ", &state.get_cwd_str());

        io::stdout().flush()?;

        let mut stdin = io::stdin().lock();

        stdin.read_line(&mut buf)?;

        // Removes trailing newline
        buf.pop();

        let command = parse_command(buf.as_str());

        let res = match command {
            Command::CD(tail) => commands::change_directory(tail, &mut state),
            Command::QT(tail) => commands::quit(tail, &mut state, &mut quit_flag),
            _ => Ok(()) 
        };

        if let Err(msg)= res {
            eprintln!("{}", msg);
        }

        if quit_flag {
            break;
        }

        buf.clear();
    }

    Ok(())
}
