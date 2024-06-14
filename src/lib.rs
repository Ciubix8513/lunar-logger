//!
//! # lunar-logging
//!
//! Simple logger, that can.. well... log stuff, both to console and to a file.
//!
//! Works mostly like `env_logger`, except configured entirely from code and can write to files by
//! itself.
//!
//! Usage:
//! ```
//!use lunar_logger::Logger;
//!
//!let mut logger = Logger::new();
//!
//!logger.add_filter("wgpu", lunar_logger::FilterType::Crate, log::LevelFilter::Warn);
//!logger.set_default_filter(log::LevelFilter::Info);
//!logger.enable_logger();
//!
//!log::info!("It works!");
//! ```
mod builder;

pub use builder::Builder;
#[cfg(test)]
mod tests;

use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock, RwLock},
};

///Errors of the logger
#[derive(Debug)]
pub enum LoggerError {
    LoggerAlreadySet,
    FileError(std::io::Error),
    InvalidFiname,
}

///Logger structure
pub struct Logger {
    filters: Vec<(String, FilterType, log::LevelFilter)>,
    log_to_file: bool,
    log_filename: PathBuf,
    default_level: log::LevelFilter,
    time_format: String,
    log_file: Option<RwLock<std::fs::File>>,
    use_color: bool,
}

///Types of filter that can be added
#[derive(Clone, Copy)]
pub enum FilterType {
    ///Filters by the name of the module
    Module,
    ///Filters by the crate name
    Crate,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    ///Crates a new `Logger`
    #[must_use]
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            log_to_file: false,
            log_filename: generate_log_name(),
            default_level: log::LevelFilter::Info,
            time_format: "%Y-%m-%d %H:%M:%S".into(),
            log_file: None,
            use_color: true,
        }
    }

    ///Consumes the logger and sets it as the program logger
    ///
    /// # Errors
    ///
    /// returns an error if a logger is already in use or if failed to create a log file
    pub fn enable_logger(mut self) -> Result<(), LoggerError> {
        if self.log_to_file {
            if let Err(e) = create_file(&self.log_filename) {
                return Err(LoggerError::FileError(e));
            }

            match std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false)
                .open(&self.log_filename)
            {
                Ok(f) => self.log_file = Some(RwLock::new(f)),
                Err(e) => return Err(LoggerError::FileError(e)),
            }
        }

        //Figure out the max level
        let max_level = self
            .filters
            .iter()
            .map(|i| i.2)
            .max()
            .unwrap_or(self.default_level)
            .max(self.default_level);

        log::set_max_level(max_level);

        if INTERNAL_LOGGER.set(Arc::new(self)).is_err() {
            return Err(LoggerError::LoggerAlreadySet);
        }
        let Some(logger) = INTERNAL_LOGGER.get() else {
            return Err(LoggerError::LoggerAlreadySet);
        };
        if log::set_logger(logger.as_ref() as &dyn log::Log).is_err() {
            Err(LoggerError::LoggerAlreadySet)
        } else {
            Ok(())
        }
    }

    ///Adds a filter for a specified module/crate, with the specified level
    pub fn add_filter(
        &mut self,
        module_name: &str,
        filter_type: FilterType,
        level: log::LevelFilter,
    ) {
        self.filters
            .push((module_name.to_owned(), filter_type, level));
    }

    ///Sets the filename of the log file.
    ///
    ///Logging to a file must be enabled separately
    ///
    /// # Errors
    ///
    /// returns an error if the provided filename is a directory
    pub fn set_log_file_name(&mut self, filename: &Path) -> Result<(), LoggerError> {
        if filename.is_dir() {
            return Err(LoggerError::InvalidFiname);
        }
        filename.clone_into(&mut self.log_filename);
        Ok(())
    }

    ///Enables logging to a file
    pub fn set_log_to_file(&mut self) {
        self.log_to_file = true;
    }

    ///Sets the time stamp format when logging
    pub fn set_timestamp_format(&mut self, format: &str) {
        format.clone_into(&mut self.time_format);
    }

    ///Sets the default logging level, that filters everything that does not have a dedicated filter
    pub fn set_default_filter(&mut self, level: log::LevelFilter) {
        self.default_level = level;
    }

    ///Sets the logger will use color when logging
    pub fn use_color(&mut self, value: bool) {
        self.use_color = value;
    }
}

fn create_file(path: &Path) -> Result<(), std::io::Error> {
    let Some(parent) = path.parent() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File is a directory",
        ));
    };
    std::fs::create_dir_all(parent)?;
    std::fs::File::create(path)?;

    Ok(())
}

fn generate_log_name() -> PathBuf {
    //ISO-8601 time
    let time = get_time("%Y-%m-%dT%H:%M:%S");
    //TODO Think about windows
    let user = std::env::vars().find(|i| i.0 == "USER").unwrap().1;

    format!("/home/{user}/.local/share/lunar-logging/log-{time}.log").into()
}

fn filter(filter: &str, filter_type: FilterType, data: &str) -> bool {
    //crate_name::module::module::module:: ...
    let mut split = data.split("::");

    let crate_name = split.next().unwrap();

    match filter_type {
        FilterType::Module => split.any(|x| x == filter),
        FilterType::Crate => crate_name == filter,
    }
}

fn get_time(format: &str) -> String {
    let time = chrono::Local::now();
    format!("{}", time.format(format))
}

const fn get_color(level: log::LevelFilter) -> &'static str {
    match level {
        log::LevelFilter::Off => "",
        log::LevelFilter::Error => "\x1b[31m",
        log::LevelFilter::Warn => "\x1b[33m",
        log::LevelFilter::Info => "\x1b[32m",
        log::LevelFilter::Debug => "\x1b[35m",
        log::LevelFilter::Trace => "\x1b[36m",
    }
}

const fn format_level(level: log::LevelFilter) -> &'static str {
    match level {
        log::LevelFilter::Off => "",
        log::LevelFilter::Error => "ERROR",
        log::LevelFilter::Warn => "WARN ",
        log::LevelFilter::Info => "INFO ",
        log::LevelFilter::Debug => "DEBUG",
        log::LevelFilter::Trace => "TRACE",
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let msg = record.args();
        let metadata = record.metadata();
        let target = metadata.target();
        let msg_level = metadata.level().to_level_filter();

        let mut filtered = false;

        for (name, filter_type, level) in &self.filters {
            if filter(name, *filter_type, target) {
                //Test if the msg level msg is less severe than the filter level
                if msg_level > *level {
                    return;
                }
                filtered = true;
                break;
            }
        }

        //If not filtered and less severe than default level return
        if !filtered && msg_level > self.default_level {
            return;
        }

        //Passed all checks and can log stuff

        //Format:
        //[TIMESTAMP TARGET LEVEL] MESSAGE
        //

        let time = get_time(&self.time_format);
        let color = get_color(msg_level);
        let msg_level = format_level(msg_level);

        let output = if self.use_color {
            format!(
                "\x1b[90m[\x1b[0m{time} {color}{msg_level} \x1b[0m{target}\x1b[90m]\x1b[0m {msg}\n"
            )
        } else {
            format!("[{time} {msg_level} {target}] {msg}\n")
        };

        if let Some(f) = &self.log_file {
            if let Err(e) = f.write().unwrap().write(output.as_bytes()) {
                log::error!("Failed to write to a file {e}");
            }
        }

        print!("{output}");
    }

    fn flush(&self) {}
}

static INTERNAL_LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();
