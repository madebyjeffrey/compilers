
use clap::{Arg, ArgGroup, ArgMatches, Command, FromArgMatches, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Lexer,
    Parse,
    CodeGen,
}

impl Mode {
    fn clap_args() -> Vec<Arg> {
        vec![
            Arg::new("lexer")
                .long("lex")
                .help("Run the lexer only")
                .action(clap::ArgAction::SetTrue),
            Arg::new("parse")
                .long("parse")
                .help("Run the lexer, then the parser.")
                .action(clap::ArgAction::SetTrue),
            Arg::new("codegen")
                .long("code-gen")
                .help("Run all stages")
                .action(clap::ArgAction::SetTrue),
        ]
    }
}

impl FromArgMatches for Mode {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        if matches.get_flag("lexer") {
            Ok(Self::Lexer)
        } else if matches.get_flag("parse") {
            Ok(Self::Parse)
        } else if matches.get_flag("codegen") {
            Ok(Self::CodeGen)
        } else {
            Ok(Self::CodeGen)
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Mode::from_arg_matches(matches)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Cli {
    pub filename: String,
    pub mode: Mode
}

impl FromArgMatches for Cli {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self {
            filename: matches
                .get_one::<String>("filename")
                .expect("file is required")
                .clone(),
            mode: Mode::from_arg_matches(matches)?,
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        self.filename = matches
            .get_one::<String>("file")
            .expect("file is required")
            .clone();
        self.mode = Mode::from_arg_matches(matches)?;
        Ok(())
    }
}

impl Cli {
    pub(crate) fn command() -> Command {
        let mut cmd = Command::new("file_tool")
            .version("0.1.0")
            .about("Never in a million Cs compiler")
            .arg(Arg::new("file").required(true).help("File to operate on"))
            .group(
                ArgGroup::new("mode")
                    .args(["lexer", "parse", "codegen"])
                    .required(false),
            );

        for arg in Mode::clap_args() {
            cmd = cmd.arg(arg);
        }

        cmd
    }
}
