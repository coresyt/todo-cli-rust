mod commands;
use clap::Command;

use crate::commands::BasicCommand;

fn main() {
    const PATH_USED: &'static str = "personas.json";
    let add_command = commands::AddCommand::new();
    let list_command = commands::ListCommand::new();
    let done_command = commands::DoneCommand::new();
    let remove_command = commands::RemoveCommand::new();

    let mut main = Command::new("")
        .version("0.5")
        .author("CoresYT, x@x.xyz")
        .subcommand(add_command.create_basic_command())
        .subcommand(list_command.create_basic_command())
        .subcommand(done_command.create_basic_command())
        .subcommand(remove_command.create_basic_command());

    let matches = main.clone().get_matches();
    let subcommands = matches.subcommand();

    match subcommands {
        Some(("add", sub_matches)) => {
            add_command.code_to_exec(String::from(PATH_USED), sub_matches)
        }
        Some(("list", sub_matches)) => {
            list_command.code_to_exec(String::from(PATH_USED), sub_matches)
        },
        Some(("done", sub_matches)) => {
            done_command.code_to_exec(String::from(PATH_USED), sub_matches)
        },
        Some(("remove", sub_matches)) => {
            remove_command.code_to_exec(String::from(PATH_USED), sub_matches)
        },
        Some(_) | None => {
            main.print_help().unwrap();
            println!();
            std::process::exit(0);
        }
    }
}
