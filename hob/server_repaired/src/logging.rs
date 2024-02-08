use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use log::{debug, LevelFilter};

pub fn setup(level: LevelFilter) {
    let stdio = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S:%3f)} {h({l})}] - {message}\n",
        )))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdio", Box::new(stdio)))
        .build(Root::builder().appender("stdio").build(level))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    debug!("Logging Started");
}
