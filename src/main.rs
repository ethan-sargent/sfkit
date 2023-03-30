pub mod auth;
pub mod completions;
pub mod config;
use clap::{Command, Subcommand, Parser};
use config::ConfigCommand;
use anyhow::{Result, Error};


#[derive(Parser)]
pub enum Commands {
    #[command(subcommand)]
    Config(ConfigCommand)
}

fn cli() -> Command {
    let command = Command::new("sfkit")
        .author("Ethan Sargent, ethan.sargent@icloud.com")
        .version("0.0.2")
        .about("Blazingly fast Salesforce developer tools, built with Rust.")
        .arg_required_else_help(true)
        .subcommands([
            auth::subcommand(),
            completions::subcommand()
        ]);
    let command = Commands::augment_subcommands(command);
    command
}


pub fn main() { 
    let app_m = cli().get_matches();

     let result: Result<(), Error> = match app_m.subcommand() {
        Some(("auth", sub_matches)) => {
            auth::run(&sub_matches)
        }
        Some(("completions", sub_matches)) => {
            let mut cli = cli();
            completions::print_completions(&sub_matches, &mut cli)
        }
        Some(("config", sub_matches)) => {
           config::run(&sub_matches)
        }
        _ => {
            unreachable!()
        }
    };
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("{:#?}", Error::from(e))
    };
}
