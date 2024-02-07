use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use log::{debug, error, info, trace, warn, LevelFilter};

pub fn setup(level: LevelFilter) {
    let stdio = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{({d(%H:%M:%S%.6f)}):15.15} {h({({l}):5})} [{target}] - {message}\n",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdio", Box::new(stdio)))
        .build(Root::builder().appender("stdio").build(level))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    debug!("Logging Started");
}
