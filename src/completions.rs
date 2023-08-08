use anyhow::Result;
use clap::{value_parser, Arg, ArgMatches, Command};
use clap_complete::{generate, Shell};

pub fn print(matches: &ArgMatches, cmd: &mut Command) -> Result<()> {
    if let Some(gen) = matches.get_one::<Shell>("shell").copied() {
        generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
    }
    Ok(())
}

#[must_use]
pub fn subcommand() -> Command {
    Command::new("completions")
        .about("Generate shell completions.")
        .arg(
            Arg::new("shell")
                .value_parser(value_parser!(Shell))
                .required(true),
        )
}
