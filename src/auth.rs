use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::{env, fs};

#[derive(Debug, clap::Subcommand)]
enum Actions {
    List,
    Login,
    Logout,
}

pub fn subcommand() -> Command {
    Command::new("auth")
        .about("Authorisation management and configuration.")
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

pub fn run(args: &ArgMatches) {
    match args.subcommand() {
        Some(("set", sub_m)) => match set_auth(sub_m) {
            Ok(()) => println!("Successfully set new default username."),
            Err(message) => println!("{}", message),
        },
        Some(("list", _sub_m)) => {
            match get_usernames() {
                Ok(usernames) => print_usernames(&usernames),
                Err(m) => println!("Unable to get usernames. Details: {}", m),
            };
        }
        _ => {
            println!("{} not found", args.subcommand_name().unwrap_or("?"));
        }
    }
}

pub fn set_auth(args: &ArgMatches) -> Result<(), &str> {
    let Ok(mut config_path) = env::current_dir() else {
        return Err("Unable to get current working directory.");
    };
    config_path.push(".sfdx/sfdx-config.json");
    let Ok(contents) = fs::read_to_string(config_path.as_path()) else {
        return Err("Unable to read sfdx-config file.");
    };
    let Ok(mut cfg) = serde_json::from_str::<serde_json::Value>(&contents) else {
        return Err("Unable to deserialize sfdx-config file.");
    };
    let Some(cfg_map) = cfg.as_object_mut() else {
        return Err("sfdx-config is not a valid JSON object.")
    };

    cfg_map.insert(
        "defaultusername".to_owned(),
        serde_json::json!(args.get_one::<String>("target-org")),
    );

    fs::write(config_path, cfg.to_string() + "\n").expect("Unable to write to sfdx-config file");

    Ok(())
}

pub fn get_usernames() -> Result<Vec<String>, &'static str> {
    let mut sfdx_dir = dirs::home_dir().unwrap();
    sfdx_dir.push(".sfdx");

    let Ok(files) = fs::read_dir(sfdx_dir) else {
        return Err("Cannot read ~/.sfdx files")
    };
    let username_regex = Regex::new(r"(?P<username>.*@.*)\.json").unwrap();
    let usernames: Vec<String> = files
        .filter_map(|file| {
                Some(username_regex.captures(file.unwrap().file_name().to_str().unwrap())?                
                .name("username")?
                .as_str()
                .to_owned())
        })
        .collect();
    Ok(usernames)
}

pub fn print_usernames(usernames: &Vec<String>) {
    for username in usernames {
        println!("Username: {}", username)
    }
}
