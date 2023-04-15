use std::path::PathBuf;

use clap::{builder::ValueParser, Arg, ArgAction, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Args {
    pub path: PathBuf,
    // pub git: bool,
}

impl Args {
    pub fn parse() -> Args {
        let command = Command::new("barrel")
        .about("Create barrel files for TS directories")
        .author("William Roberts")
        .version(VERSION)
        .arg(
          Arg::new("path")
              .action(ArgAction::Set)
              .default_value("./")
              .hide_default_value(true)
              .value_parser(ValueParser::path_buf())
              .help("The path where the file should be made or updated. Default is the current path")
        );
        // .arg(
        //   Arg::new("git")
        //       .long("git")
        //       .short('g')
        //       .action(ArgAction::SetTrue)
        //       .help("Checks all staged files and updates their directories")
        // );

        let matches = command.get_matches();
        let path = matches.get_one::<PathBuf>("path").unwrap();
        // let git = matches.get_one::<bool>("git").unwrap();

        Args {
            path: path.to_path_buf(),
            // git: *git == true,
        }
    }
}
