use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    env_logger::init();

    hat::start().map(|s| {
        if s {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    })
}
