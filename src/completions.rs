use clap_complete::{generate, Shell};
use anyhow::Result;
use clap::{Command, Arg, value_parser, ArgMatches};


pub fn print_completions(matches: &ArgMatches, cmd: &mut Command) -> Result<()> {
    if let Some(gen) = matches.get_one::<Shell>("shell").copied() {
        generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
    }
    Ok(())
}

pub fn subcommand() -> Command {
    Command::new("completions")
        .about("Generate shell completions.")
        .arg(Arg::new("shell")
            .value_parser(value_parser!(Shell))
            .required(true)
        )
}

