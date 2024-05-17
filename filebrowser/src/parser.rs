type Tail = Vec<String>;

#[derive(Debug)]
pub enum Command {
    // Change directory
    // 1 argument
    CD(Tail),

    // Copy file(s)
    // 1+ arguments
    CP(Tail),

    // Cut file(s)
    // 1+ arguments
    CT(Tail),

    // Paste file(s) into a directory
    // 1 argument
    PS(Tail),

    // Rename file
    // 2 arguments (old name, new name)
    RN(Tail),

    // Remove file(s)
    // 1+ arguments
    RM(Tail),

    // Open file in editor
    // 1 argument
    ED(Tail),

    // New file(s)
    // 1+ argument
    NF(Tail),

    // List files in directory
    // 0 arguments
    LS(Tail),

    // Quit the program
    // 0 arguments
    QT(Tail),
}

pub fn parse_command(cmd: &str) -> Command {
    let words: Vec<String> = cmd.split(" ").map(|x | x.to_string()).collect();
    let tail = words[1..].to_vec();

    match words[0].as_str() {
        "cd" => Command::CD(tail),
        "cp" | "copy" => Command::CP(tail),
        "ct" | "cut"=> Command::CT(tail),
        "ps" | "paste" => Command::PS(tail),
        "rn" | "rename"=> Command::RN(tail),
        "rm" | "remove" => Command::RM(tail),
        "ed" | "edit" | "editor" => Command::ED(tail),
        "nf" | "new" | "newfile" => Command::NF(tail),
        "ls" | "list" => Command::LS(tail),
        "q" | "qt" | "quit" => Command::QT(tail),
        _ => panic!("Invalid command"),
    }
}
