use serde::{Deserialize, Serialize};
use clap::{Arg, ArgMatches, Command};
use std::{env, fs, path};

#[derive(Debug, clap::Subcommand)]
enum Actions {
    List,
    Login,
    Logout,
}


pub fn get_auth_command() -> Command {
    Command::new("auth").subcommands([
        Command::new("list").about("Lists available usernames and aliases"),
        Command::new("set")
            .about("Change the default org for the current project.")
            .args([Arg::new("target-org")
                .short('o')
                .help("Username or alias for the current org.")
                .required(true)]),
    ])
}

pub fn run(args: &ArgMatches) -> () {
    match args.subcommand() {
        Some(("set", sub_m)) => set_auth(sub_m),
        Some(("list", sub_m)) => {
            println!("Results:");
            println!("--------");
        }
        _ => {
            println!("{} not found", args.subcommand_name().unwrap_or("?"));
        }
    }
}

pub fn set_auth(args: &ArgMatches) {
    let mut config_path: path::PathBuf = env::current_dir().expect("Unable to get current working directory.");
    config_path.push(".sfdx/sfdx-config.json");
    let contents = fs::read_to_string(config_path.as_path()).expect("Unable to read sfdx-config file.");

    let mut cfg : serde_json::Value = serde_json::from_str(&contents).expect("Unable to deserialize sfdx-config file.");

    *cfg.get_mut("defaultusername").unwrap() = serde_json::json!(args.get_one::<String>("target-org").to_owned());

    fs::write(config_path, cfg.to_string() + "\n").expect("Unable to write to sfdx-config file");
}

