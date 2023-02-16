use std::path::PathBuf;

use clap::{builder::ValueParser, Arg, ArgAction, Command};
use globset::{Glob, GlobMatcher};
use regex::Regex;

pub struct Arguments {
    pub path: PathBuf,
    pub options: Options,
}

impl Arguments {
    pub fn parse() -> Arguments {
        let command = Command::new("barrel")
          .about("Create barrel files for TS directories")
          .author("William Roberts")
          .arg(
            Arg::new("path")
                .action(ArgAction::Set)
                .default_value("./")
                .hide_default_value(true)
                .value_parser(ValueParser::path_buf())
                .help("The path where the file should be made or updated. Default is the current path")
          );

        let matches = command.get_matches();
        let path = matches.get_one::<PathBuf>("path").unwrap();

        Arguments {
            path: path.to_path_buf(),
            options: Options::new(),
        }
    }
}

pub struct Options {
    pub ignore: Regex,
    pub glob: GlobMatcher,
}

impl Options {
    fn new() -> Options {
        let glob = Glob::new("*.{ts,tsx}").unwrap().compile_matcher();
        let ignore = Regex::new("(.test|.stories|index)").unwrap();

        Options { ignore, glob }
    }
}

pub enum Export {
    None,
    Named,
    Default(String),
    Module,
}

impl Export {
    pub fn to_value(&self) -> Option<String> {
        match self {
            Export::Named | Export::Module => Some("export * from".to_string()),
            Export::Default(name) => {
                let export = format!("export {{ default as {} }} from", name);
                Some(export)
            }
            _ => None,
        }
    }
}
