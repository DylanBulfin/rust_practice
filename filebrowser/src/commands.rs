type Tail = Vec<String>;

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
    NF(Tail)
}

fn parse_command(cmd: &str) -> Command {
    let words: Vec<&str> = cmd.split(' ').collect();

    unimplemented!()
}
