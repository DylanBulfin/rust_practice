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
}

pub fn parse_command(cmd: &str) -> Command {
    let words: Vec<String> = cmd.split(" ").map(|x | x.to_string()).collect();

    match words[0].as_str() {
        "cd" => Command::CD(words[1..].to_vec()),
        "cp" => Command::CP(words[1..].to_vec()),
        "ct" => Command::CT(words[1..].to_vec()),
        "ps" => Command::PS(words[1..].to_vec()),
        "rn" => Command::RN(words[1..].to_vec()),
        "rm" => Command::RM(words[1..].to_vec()),
        "ed" => Command::ED(words[1..].to_vec()),
        "nf" => Command::NF(words[1..].to_vec()),
        _ => panic!("Invalid command"),
    }
}
