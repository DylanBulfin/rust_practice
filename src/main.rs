pub mod datastore;
pub mod types;

extern crate chrono;
use clap::Parser;
use types::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    println!("Hello, world!");
    println!("{:?}", cli);

    match cli.command {
        Commands::Add {
            description,
            due_date,
        } => datastore::add_task(description, due_date),
        Commands::List => datastore::list_tasks(),
        Commands::Complete { id } => datastore::complete_task(id),
        Commands::Delete { id } => datastore::delete_task(id),
        Commands::Edit { id, new_desc, new_date } => datastore::edit_task(id, new_desc ,new_date),
    };

    //unimplemented!()
}
