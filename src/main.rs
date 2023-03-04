pub mod auth;

use clap::Command;


pub fn main() {
    let app_m = Command::new("sfkit")
        .author("Ethan Sargent, ethan.sargent@icloud.com")
        .version("0.0.1")
        .about("Blazingly fast Salesforce developer tools, built with Rust.")
        .subcommand(auth::get_auth_command())
        .get_matches();

    if let Some(auth_m) = app_m.subcommand_matches("auth") {
        auth::run(&auth_m);
    }

}
