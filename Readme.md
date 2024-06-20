# lunar-logger

Simple logger, that can.. well... log stuff, both to console and to a file.

Works mostly like env_logger, except configured entirely from code and can write to files by
itself. Also supports logging on wasm.

## Usage:

 ```rs
    use lunar_logger::Logger;

    let mut logger = Logger::new();

    logger.add_filter("wgpu", lunar_logger::FilterType::Crate, log::LevelFilter::Warn);
    logger.set_default_filter(log::LevelFilter::Info);
    logger.enable_logger();

    log::info!("It works!");
 ```
