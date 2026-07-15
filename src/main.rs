use std::process::ExitCode;

use clap::Parser;
use shellcd_basic::{cli::Cli, config::RunConfig};

fn main() -> ExitCode {
    shellcd_basic::logging::init();

    let result = RunConfig::try_from(Cli::parse().command).and_then(shellcd_basic::execute);

    let code = match result {
        Ok(code) => code,
        Err(error) => {
            tracing::error!(
                event = "shellcd.execution.error",
                result = error.outcome(),
                exit_code = error.exit_code(),
                error = %error
            );
            error.exit_code()
        }
    };

    ExitCode::from(u8::try_from(code).unwrap_or(5))
}
