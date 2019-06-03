use clap::{load_yaml, App};
use log::{info, LevelFilter};
use simplelog::{CombinedLogger, Config, TermLogger, WriteLogger};
use std::fs::File;

const LOGGING_PATH: &str = "image-analytics-bridge.log";

fn get_version_str() -> String {
    format!(
        "{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    )
}

fn main() {
    // configure the command line parser first (since we need the verbosity level for the logger)
    let cli_configuration_yaml = load_yaml!("cli.yml");
    let argument_matches = App::from_yaml(cli_configuration_yaml).get_matches();

    // determine the correct logging level
    let logging_level = match argument_matches.occurrences_of("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    // be sure that we can create the logfile in the requested path
    let log_file_handle = File::create(LOGGING_PATH);
    if log_file_handle.is_err() {
        panic!(
            "Could no create the logfile {}. Terminating program!",
            LOGGING_PATH
        )
    }

    // be sure that we could create the terminal logger
    let terminal_logger = TermLogger::new(logging_level, Config::default());

    // create the logger which logs into a file
    let write_logger = WriteLogger::new(logging_level, Config::default(), log_file_handle.unwrap());

    // configure the logging framework and set the corresponding log level
    if terminal_logger.is_some() {
        if CombinedLogger::init(vec![terminal_logger.unwrap(), write_logger]).is_err() {
            panic!("Could not create a combined logger. Terminating program!");
        }
    } else {
        if CombinedLogger::init(vec![write_logger]).is_err() {
            panic!("Could not create a 'writer only' logger. Terminating program!");
        }
    };

    // tell the user that we started to spin up the API
    info!("Image Analytics Bridge {} started", get_version_str());
}
