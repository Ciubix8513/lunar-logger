# lunar-logger

Simple logger, that can.. well... log stuff, both to console and to a file.

Works mostly like env_logger, except configured entirely from code and can write to files by
itself.

## Usage:

```rs
use lunar_logging::*;

fn main() {
    let logger = Logger::new();

    logger.add_filter("wgpu", FilterType::Crate, log::LevelFilter::Warn);
    logger.set_default_filter(log::LevelFilter::Info);
    logger.set_logger();

    log::info!("It works!");
}
```
