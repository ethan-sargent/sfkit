use clap_complete::{generate, Shell};
use clap::{Command, Arg, value_parser, ArgMatches};

pub fn print_completions(matches: &ArgMatches, cmd: &mut Command) {
    if let Some(gen) = matches.get_one::<Shell>("shell").copied() {
        generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
    }
}

pub fn subcommand() -> Command {
    Command::new("completions")
        .about("Generate shell completions.")
        .arg(Arg::new("shell")
            .value_parser(value_parser!(Shell))
            .required(true)
        )
}

