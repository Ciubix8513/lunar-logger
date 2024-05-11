use std::str::FromStr;

use log::LevelFilter;

use super::*;

#[test]
fn test_filter() {
    let target = "tests::something::something1::something2";
    assert!(filter("something", FilterType::Module, target));
    assert!(filter("tests", FilterType::Crate, target));

    assert!(!filter("something", FilterType::Crate, target));
    assert!(!filter("tests", FilterType::Module, target));
}

#[test]
fn test_builder() {
    crate::Builder::new()
        .add_mod_filter("stuff", log::LevelFilter::Info)
        .add_crate_filter("wgpu", LevelFilter::Warn)
        .log_to_file()
        .log_filname(&(PathBuf::from_str("./test.log").unwrap()))
        .time_format("%Y")
        .default_filter(LevelFilter::Trace)
        .create()
        .enable_logger()
        .unwrap();

    log::trace!("TEST");
    log::debug!("TEST");
    log::info!("TEST");
    log::warn!("TEST");
    log::error!("TEST");
}
