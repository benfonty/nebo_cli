use log::LevelFilter;
use log4rs::Handle;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

pub fn init(level: u64) -> Handle {
    // level = 0, no output
    // level = 1 (-v), only info output from nebo_cli
    // level = 2 (-vv), debug output from nebo_cli
    // level = 3 (-vvv), debug output from all crates
    
    let debug_level_nebocli = match level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug
    };

    let pattern = if level >= 2 {
        "{h({M} - {m}{n})}"
    }
    else {
        "{m}{n}"
    };

    let debug_level_all = if level == 3 {
        LevelFilter::Debug
    }
    else {
        LevelFilter::Off
    };
    
    let stdout = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(pattern))).build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("nebo_cli", debug_level_nebocli))
        .build(Root::builder().appender("stdout").build(debug_level_all))
        .unwrap();
    log4rs::init_config(config).unwrap()
}