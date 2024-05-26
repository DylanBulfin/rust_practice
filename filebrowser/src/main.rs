mod commands;
mod parser;
mod paths;
mod types;
use std::io::{self, BufRead, Write};

use colored::Colorize;
use parser::parse_command;
use parser::Command;
use types::State;

fn main() -> io::Result<()> {
    let mut state = State::new();
    let mut buf = String::with_capacity(2048);

    let mut quit_flag = false;

    loop {
        //print a prompt
        print!("{}{} ", &state.get_cwd_str().green(), ">".green());

        io::stdout().flush()?;

        let mut stdin = io::stdin().lock();

        stdin.read_line(&mut buf)?;

        // Removes trailing newline
        buf.pop();

        let command = parse_command(buf.as_str());

        println!("");

        let res = match command {
            Command::CD(tail) => commands::change_directory(tail, &mut state),
            Command::QT(tail) => commands::quit(tail, &mut quit_flag),
            Command::LS(tail) => commands::list_files(tail, &state),
            Command::RM(tail) => commands::remove_files(tail, &state),
            Command::CP(tail) => commands::copy(tail, &mut state),
            Command::CT(tail) => commands::cut(tail, &mut state),
            Command::PS(tail) => commands::paste(tail, &mut state),
            Command::RN(tail) => commands::rename(tail, &state),
            Command::NF(tail) => commands::new_files(tail, &state),
            Command::ED(tail) => commands::open_editor(tail, &state),
            Command::ND(tail) => commands::new_dirs(tail, &state),
        };

        if let Err(msg) = res {
            eprintln!("{}\n", msg.red());
        } else {
            println!("");
        }

        if quit_flag {
            break;
        }

        buf.clear();
    }

    Ok(())
}
