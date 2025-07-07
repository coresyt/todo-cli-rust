use super::BasicCommand;
use clap::{ArgMatches, Command};

#[path = "../json/mod.rs"]
mod json;
use json::*;

pub struct ListCommand;

impl BasicCommand for ListCommand {
    fn new() -> Self {
        ListCommand
    }

    fn create_basic_command(&self) -> Command {
        return Command::new("list");
    }

    fn code_to_exec(&self, path: String, _: &ArgMatches) {
        let file = FileManagement::new(path);
        let formatter = Formatter::new();
        let mut list: Vec<ITask> = vec![];
        let mut content: String = String::new();

        match file.read_file() {
            Ok(result) => {
                let mut was_read: bool = false;
                if let ResultList::Flag(boolean) = &result[0] {
                    was_read = *boolean;
                }

                if was_read == false {
                    println!();
                    return
                }

                if let ResultList::Text(file_content) = &result[1] {
                    content.insert_str(0, &file_content);
                }
            },
            Err(err) => {
                if let ResultList::Text(error) = &err[1] {
                    println!("{}", error)
                }

                return
            }
        };

        match formatter.str_to_object_or_list::<Vec<ITask>>(content.clone()) {
            Ok(ok) => {
                ok.clone_into(&mut list);
            },
            Err(err) => {
                println!("{}", err);
                return
            }
        };

        for task in list {
            let is_completed = if task.completed { "completed" } else { "in completed" };
            println!("{}. \"{}\" is {}", task.id, task.description, is_completed)
        }
    }
}