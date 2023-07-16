use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match hat::start() {
        Ok(s) => {
            if s {
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
