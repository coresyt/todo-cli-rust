use super::BasicCommand;
use clap::{Arg, ArgMatches, Command};

#[path = "../json/mod.rs"]
mod json;
use json::*;

pub struct AddCommand;

impl BasicCommand for AddCommand {
    fn new() -> Self {
        Self
    }
    
    fn create_basic_command(&self) -> Command {
        return Command::new("add").arg(Arg::new("description").short('d').long("description").required(true));
    }
    
    fn code_to_exec(&self, path: String, matches: &ArgMatches) {
        let file = FileManagement::new(path);
        let formatter = Formatter::new();
        let description: &String = matches.get_one::<String>("description").expect("Description is required!!!");
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

        let new_task = ITask {
            description: description.clone(),
            id: if (&list).len() == 0 { 1 } else { (&list).len() + 1 },
            completed: false
        };

        list.push(new_task);

        let list_formatted = formatter.object_or_list_to_string(&list);

        let is_written = match list_formatted {
            Ok(string_formatted) => file.write_file(string_formatted.as_bytes()),
            Err(error) => {
                println!("{}", error);
                return
            }
        };

        println!("\"{}\"", &content.as_str());
        println!("\"{:?}\"", list);
        
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
