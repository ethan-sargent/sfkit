use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::fs;

use crate::config;

#[derive(Debug, clap::Subcommand)]
enum Actions {
    List,
    Login,
    Logout,
}

pub fn subcommand() -> Command {
    Command::new("auth")
        .about("Commands that allow you to view and update connections to salesforce orgs.")
        .subcommand_required(true)
        .subcommands([
            Command::new("list").about("Lists available usernames and aliases"),
            Command::new("set")
                .about("Change the default org for the current project.")
                .args([Arg::new("target-org")
                    .short('o')
                    .help("Username or alias for the current org.")
                    .required(true)]),
        ])
}

pub fn run(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("list", _sub_m)) => match get_usernames() {
            Ok(usernames) => {
                print_usernames(&usernames);
                Ok(())
            }
            Err(e) => Err(e),
        },
        _ => {
            unreachable!("A subcommand is required")
        }
    }
}

pub fn get_usernames() -> Result<Vec<String>> {
    let sfdx_dir = config::get_config_path(&config::ConfigLocation::Global)?;

    let files = fs::read_dir(sfdx_dir)?;
    let username_regex = Regex::new(r"(?P<username>.*@.*)\.json").unwrap();
    let usernames: Vec<String> = files
        .filter_map(|file| {
            Some(
                username_regex
                    .captures(file.unwrap().file_name().to_str().unwrap())?
                    .name("username")?
                    .as_str()
                    .to_owned(),
            )
        })
        .collect();
    Ok(usernames)
}

pub fn print_usernames(usernames: &Vec<String>) {
    for username in usernames {
        println!("Username: {}", username)
    }
}
