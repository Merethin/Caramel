use simplelog::{ConfigBuilder, TermLogger, TerminalMode, ColorChoice};
use log::STATIC_MAX_LEVEL;

pub fn setup_log(exclude_modules: Vec<&'static str>) { 
    let mut builder = ConfigBuilder::new();
    for module in exclude_modules {
        builder.add_filter_ignore_str(module);
    }

    TermLogger::init(
        STATIC_MAX_LEVEL, builder.build(), TerminalMode::Stderr, ColorChoice::Auto
    ).ok();
}