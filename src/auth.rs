use anyhow::Result;
use clap::{ArgMatches, Command};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

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
        .subcommands([Command::new("list").about("Lists available usernames and aliases")])
}

pub fn run(args: &ArgMatches) -> Result<()> {
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
    let sfdx_dir = config::get_config_dir(&true)?;

    let usernames = read_usernames(&sfdx_dir)?;
    Ok(usernames)
}

fn read_usernames(sfdx_dir: &PathBuf) -> Result<Vec<String>> {
    // sfdx stores usernames in user@example.com.json files
    let username_regex = Regex::new(r"(?P<username>.*@.*)\.json").unwrap();
    let files = fs::read_dir(sfdx_dir)?;
    Ok(files
        .filter_map(|file| {
            Some(
                username_regex
                    .captures(file.unwrap().file_name().to_str().unwrap())?
                    .name("username")?
                    .as_str()
                    .to_owned(),
            )
        })
        .collect())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Aliases {
    orgs: BTreeMap<String, String>,
}

pub fn print_usernames(usernames: &Vec<String>) {
    for username in usernames {
        println!("Username: {}", username)
    }
}
