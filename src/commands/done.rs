use super::BasicCommand;
use clap::{Arg, ArgMatches, Command};

#[path = "../json/mod.rs"]
mod json;
use json::*;

pub struct DoneCommand;

impl BasicCommand for DoneCommand {
    fn new() -> Self {
        Self
    }
    
    fn create_basic_command(&self) -> Command {
        return Command::new("done").arg(Arg::new("id").long("id").required(true));
    }
    
    fn code_to_exec(&self, path: String, matches: &ArgMatches) {
        let file = FileManagement::new(path);
        let formatter = Formatter::new();
        let id: &String = matches.get_one::<String>("id").expect("ID is required!!!");
        let mut list: Vec<ITask> = vec![];
        let mut content: String = String::new();
        let id_formatted = id.parse::<usize>().expect("ID is required");

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

        let updated_task = {
            let task_to_find = &list[id_formatted - 1];
            let completed = if task_to_find.completed { false } else { true };

            ITask { id: id_formatted, description: task_to_find.description.clone(), completed }
        };

        list[id_formatted - 1] = updated_task;

        let list_formatted = formatter.object_or_list_to_string(&list);

        let is_written = match list_formatted {
            Ok(string_formatted) => file.write_file(string_formatted.as_bytes()),
            Err(error) => {
                println!("{}", error);
                return
            }
        };

        match is_written {
            Ok(written) => {
                if written == false {
                    println!("There was a mistake in the writing");
                    return;
                }

                println!("Your task was created successfully")
            },
            Err(err) => {
                if let ResultList::Text(error) = &err[1] {
                    println!("{}", error)
                }

                return
            },
        }
    }
}
