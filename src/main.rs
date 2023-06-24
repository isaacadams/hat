use std::process::ExitCode;

fn main() -> ExitCode {
    init_logging();

    match hat::start() {
        Ok(success) => {
            if success {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}

pub fn init_logging() {
    use chrono::{DateTime, Utc};
    use simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
    };

    let datetime: DateTime<Utc> = chrono::offset::Utc::now();

    CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Error,
            ConfigBuilder::default().build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            log::LevelFilter::Debug,
            ConfigBuilder::default()
                //.add_filter_ignore_str("yup_oauth2")
                .build(),
            std::fs::File::create(format!("{}.log", datetime.format("%Y-%m-%dT%H"))).unwrap(),
        ),
    ])
    .unwrap();
}
