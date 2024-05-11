use std::{fs, mem};

use chrono::NaiveDate as Date;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    due_date: Date,
    complete: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    next_id: u32,
    tasks: Vec<Task>,
}

const STORE_PATH: &str = "tasks.db";

impl State {
    fn from_file() -> Self {
        if fs::metadata(STORE_PATH).is_err() {
            return State {
                next_id: 0,
                tasks: Vec::new(),
            };
        }

        let file_str = fs::read_to_string(STORE_PATH).expect("Unable to read file");

        serde_json::from_str(file_str.as_str()).expect("State stored in file invalid")
    }

    fn save_state(&self) {
        let file_str = serde_json::to_string(self).expect("Cannot save state, invalid format");

        fs::write(STORE_PATH, file_str).expect("Unable to write file")
    }

    fn get_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id
    }

    fn add_task(&mut self, description: String, due_date: Date) {
        let new_task = Task {
            id: self.get_id(),
            complete: false,
            due_date,
            description,
        };

        (&mut self.tasks).push(new_task)
    }

    fn get_tasks(&self) -> &Vec<Task> {
        return &self.tasks;
    }

    fn find_task(&self, id: u32) -> Option<&Task> {
        return self.tasks.iter().find(|t| t.id == id);
    }

    fn complete_task(&mut self, id: u32) {
        self.tasks = mem::take(&mut self.tasks)
            .into_iter()
            .map(|t| {
                if t.id == id {
                    Task {
                        id: t.id,
                        due_date: t.due_date,
                        description: t.description,
                        complete: true,
                    }
                } else {
                    t
                }
            })
            .collect();
    }

    fn delete_task(&mut self, id: u32) -> Option<Task> {
        let ind = self.tasks.iter().position(|t| t.id == id);
        ind.map(|i| (&mut self.tasks).remove(i))
    }

    fn edit_task(
        &mut self,
        id: u32,
        new_desc: Option<String>,
        new_date: Option<Date>,
    ) -> Option<&Task> {
        for task in &mut self.tasks {
            if task.id != id {
                continue;
            }
            if let Some(d) = new_desc {
                task.description = d;
            }
            if let Some(d) = new_date {
                task.due_date = d;
            }
            return Some(task);
        }
        None
    }
}

pub fn add_task(description: String, due_date: Date) {
    let mut state = State::from_file();

    state.add_task(description, due_date);

    println!("Successfully created task");

    state.save_state();
}

pub fn list_tasks() {
    let state = State::from_file();

    let tasks = state.get_tasks();

    println!(
        "{: <6} | {: <8} | {: <20} | {: <10}",
        "id".bold(),
        "complete".bold(),
        "description".bold(),
        "due_date".bold()
    );
    for t in tasks {
        let desc;
        if t.description.len() > 20 {
            desc = &t.description[..20];
        } else {
            desc = &t.description[..];
        }

        println!(
            "{: <6} | {: <8} | {: <20} | {: <10}",
            t.id.to_string().green(),
            t.complete.to_string().green(),
            desc.green(),
            t.due_date.to_string().green()
        );
    }
}

pub fn complete_task(id: u32) {
    let mut state = State::from_file();

    let curr_task = state.find_task(id);
    match curr_task {
        None => eprintln!("Unable to find task with id {}", id),
        Some(t) => {
            if t.complete {
                eprintln!("Task is already marked as complete");
                return;
            }
        }
    }

    state.complete_task(id);
    println!("Successfully marked task {} as complete", id);

    state.save_state();
}

pub fn delete_task(id: u32) {
    let mut state = State::from_file();

    if let Some(t) = state.delete_task(id) {
        println!("Successfully created task {}", id);
        println!("{:?}", t);
    } else {
        eprintln!("Unable to find specified task to delete");
    }

    state.save_state();
}

pub fn edit_task(id: u32, new_desc: Option<String>, new_date: Option<Date>) {
    let mut state = State::from_file();

    if let Some(_) = state.edit_task(id, new_desc, new_date) {
        println!("Successfully updated task {}", id);
    } else {
        eprintln!("Unable to find specified task to edit");
    }

    state.save_state()
}
