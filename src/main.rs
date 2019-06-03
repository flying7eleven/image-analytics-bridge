use log::{info, LevelFilter};
use simplelog::{CombinedLogger, Config, TermLogger, WriteLogger};
use std::fs::File;

#[cfg(debug_assertions)]
const LOGGING_LEVEL: LevelFilter = LevelFilter::Trace;

#[cfg(not(debug_assertions))]
const LOGGING_LEVEL: LevelFilter = LevelFilter::Info;

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
    // be sure that we can create the logfile in the requested path
    let log_file_handle = File::create(LOGGING_PATH);
    if log_file_handle.is_err() {
        panic!(
            "Could no create the logfile {}. Terminating program!",
            LOGGING_PATH
        )
    }

    // be sure that we could create the terminal logger
    let terminal_logger = TermLogger::new(LOGGING_LEVEL, Config::default());

    // create the logger which logs into a file
    let write_logger = WriteLogger::new(LOGGING_LEVEL, Config::default(), log_file_handle.unwrap());

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
