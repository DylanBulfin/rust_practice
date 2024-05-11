use chrono::NaiveDate as Date;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[command(name = "add", about = "Add a task")]
    Add {
        #[arg(long, short)]
        #[arg(name = "description", help = "The description of the task")]
        description: String,

        #[arg(long)]
        #[arg(name = "due_date", help = "The due date of the task")]
        due_date: Date,
    },
    #[command(name = "list", about = "List all tasks")]
    List,
    #[command(name = "delete", about = "Delete a task by ID")]
    Delete {
        #[arg(long, short)]
        #[arg(name = "id", help = "The id of the task")]
        id: u32,
    },
    #[command(name = "edit", about = "Edit a task by ID")]
    Edit {
        #[arg(long, short)]
        #[arg(name = "id", help = "The id of the task")]
        id: u32,

        #[arg(name = "description", help = "The new description")]
        #[arg(long, short)]
        new_desc: Option<String>,

        #[arg(name = "due_date", help = "The new due date to use")]
        #[arg(long)]
        new_date: Option<Date>,
    },
    #[command(name = "complete", about = "Mark a task as complete")]
    Complete {
        #[arg(long, short)]
        #[arg(name = "id", help = "The id of the task")]
        id: u32,
    },
}
