mod assertion;
mod config;
mod error;
mod factory;
mod hat_util;
mod http_file_parser;
#[allow(dead_code)]
mod operator;
mod runner;
#[cfg(test)]
mod test;

use anyhow::Context;
use clap::Parser;
use runner::HatRunner;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// path to test file
    path: String,

    /// verbose level: DEBUG, INFO, ERROR
    #[arg(short, long, default_value_t = String::from("DEBUG"))]
    verbose: String,
}

pub fn start() -> anyhow::Result<bool> {
    let args = Cli::parse();
    test(&args.path)
}

fn test(config_path: &str) -> anyhow::Result<bool> {
    let config =
        config::read(config_path).context(format!("cannot find or load {}", config_path))?;

    let mut iter = config.tests.into_iter();
    let mut runner = HatRunner::new(
        hat_util::StoreUnion::MapStringToJsonValue(config.environment),
        reqwest::blocking::Client::new(),
    );
    Ok(runner.test(&mut iter))
}
