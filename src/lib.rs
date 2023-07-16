mod assertion;
mod config;
mod error;
mod factory;
mod hat_util;
mod http_file;
#[allow(dead_code)]
mod operator;
mod query;
mod runner;
#[cfg(test)]
mod test;

use clap::Parser;
use query::Content;
use runner::HatRunner;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// path to .toml configuration file
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
    let config = config::read(config_path)?;

    let environment = config
        .environment
        .into_iter()
        .map(|(key, value)| (key, Content::new(value)))
        .collect();

    let mut iter = config.tests.into_iter();
    let mut runner = HatRunner::new(
        hat_util::StoreUnion::MapStringToContent(environment),
        ureq::AgentBuilder::new().build(),
    );
    Ok(runner.test(&mut iter))
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("example/**/*.md")
        .case("README.md");
}
