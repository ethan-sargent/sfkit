pub mod auth;
pub mod completions;
pub mod config;
use anyhow::{Error, Result};
use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
pub enum Commands {
    #[command(subcommand)]
    Config(config::Commands),
}

fn cli() -> Command {
    let command = Command::new("sfkit")
        .author("Ethan Sargent, ethan.sargent@icloud.com")
        .version("0.0.2")
        .about("Blazingly fast Salesforce developer tools, built with Rust.")
        .arg_required_else_help(true)
        .subcommands([auth::subcommand(), completions::subcommand()]);

    Commands::augment_subcommands(command)
}

pub fn main() {
    let app_m = cli().get_matches();

    let result: Result<(), Error> = match app_m.subcommand() {
        Some(("auth", sub_matches)) => auth::run(sub_matches),
        Some(("completions", sub_matches)) => {
            let mut cli = cli();
            completions::print(sub_matches, &mut cli)
        }
        Some(("config", sub_matches)) => config::run(sub_matches),
        _ => {
            unreachable!()
        }
    };
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("{e:#?}"),
    };
}
