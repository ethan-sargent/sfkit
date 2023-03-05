pub mod auth;
pub mod completions;

use clap::Command;

fn cli() -> Command {
    Command::new("sfkit")
        .author("Ethan Sargent, ethan.sargent@icloud.com")
        .version("0.0.1")
        .about("Blazingly fast Salesforce developer tools, built with Rust.")
        .subcommand_required(true)
        .subcommands([
            auth::subcommand(),
            completions::subcommand()
        ])
}


pub fn main() {
    let app_m = cli().get_matches();

    match app_m.subcommand() {
        Some(("auth", sub_matches)) => {
            auth::run(&sub_matches);
        }
        Some(("completions", sub_matches)) => {
            let mut cli = cli();
            completions::print_completions(&sub_matches, &mut cli)
        }
        _ => {
            unreachable!()
        }
    }
}
