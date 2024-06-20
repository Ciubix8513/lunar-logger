use std::path::{Path, PathBuf};

///Builder struct for easier [Logger](crate::Logger) creation
///
///Example:
///
///```
///use lunar_logger::Builder;
///
///Builder::new()
///     .add_crate_filter("wgpu",log::LevelFilter::Warn)
///     .default_filter(log::LevelFilter::Info)
///     .log_to_file()
///     .create()
///     .enable_logger();
///
///log::info!("It works!");
///```
pub struct Builder {
    crate_filters: Vec<(String, log::LevelFilter)>,
    mod_filters: Vec<(String, log::LevelFilter)>,
    default_level: log::LevelFilter,
    log_to_file: bool,
    log_filename: Option<PathBuf>,
    time_format: String,
    use_color: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    ///Creates a new builder
    #[must_use]
    pub const fn new() -> Self {
        Self {
            crate_filters: Vec::new(),
            mod_filters: Vec::new(),
            default_level: log::LevelFilter::Info,
            log_to_file: false,
            log_filename: None,
            time_format: String::new(),
            use_color: true,
        }
    }

    ///Adds a filter for a crate
    #[must_use]
    pub fn add_crate_filter(mut self, crate_name: &str, level: log::LevelFilter) -> Self {
        self.crate_filters.push((crate_name.to_owned(), level));
        self
    }

    ///Adds a filter for a module
    #[must_use]
    pub fn add_mod_filter(mut self, crate_name: &str, level: log::LevelFilter) -> Self {
        self.mod_filters.push((crate_name.to_owned(), level));
        self
    }

    ///Sets the default logging level
    #[must_use]
    pub const fn default_filter(mut self, level: log::LevelFilter) -> Self {
        self.default_level = level;
        self
    }

    ///Enables logging to file
    #[must_use]
    pub const fn log_to_file(mut self) -> Self {
        self.log_to_file = true;
        self
    }

    ///Sets the filename of the log file
    #[must_use]
    pub fn log_filname(mut self, filename: &Path) -> Self {
        self.log_filename = Some(filename.to_owned());
        self
    }

    ///Sets the time stamp format
    #[must_use]
    pub fn time_format(mut self, format: &str) -> Self {
        format.clone_into(&mut self.time_format);
        self
    }

    ///Sets whether the logger will use color
    ///
    ///Default is true
    #[must_use]
    pub const fn use_color(mut self, value: bool) -> Self {
        self.use_color = value;
        self
    }

    ///Crates the [Logger](crate::Logger) from the builder
    ///
    ///# Panics
    ///
    ///Will panic if the log filename is not a valid filename
    #[must_use]
    pub fn create(self) -> super::Logger {
        let mut logger = crate::Logger::new();

        //100 disable color on wasm
        #[cfg(not(target_arch = "wasm32"))]
        {
            logger.use_color = self.use_color;
        }

        logger.set_default_filter(self.default_level);

        for (name, level) in self.crate_filters {
            logger.add_filter(&name, crate::FilterType::Crate, level);
        }
        for (name, level) in self.mod_filters {
            logger.add_filter(&name, crate::FilterType::Module, level);
        }

        if !self.time_format.is_empty() {
            logger.set_timestamp_format(&self.time_format);
        }

        if self.log_to_file {
            logger.set_log_to_file();

            if let Some(f) = self.log_filename {
                logger.set_log_file_name(&f).unwrap();
            }
        }

        logger
    }

    ///Creates the logger and sets it to be the logger of the program
    ///
    ///# Errors
    ///
    ///see [`enable_logger`](crate::Logger::enable_logger)
    ///
    ///# Panics
    ///
    ///Will panic if the log filename is not a valid filename
    pub fn init(self) -> Result<(), crate::LoggerError> {
        self.create().enable_logger()
    }
}
