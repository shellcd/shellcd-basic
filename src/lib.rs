pub mod cli;
pub mod config;
pub mod dorarion;
pub mod error;
pub mod logging;
pub mod ssh;
pub mod validation;

use std::time::Instant;

use config::RunConfig;
use error::AppError;

pub fn execute(config: RunConfig) -> Result<i32, AppError> {
    if config.dorarion {
        dorarion::call_dorarion_api_cd_start();
    }

    let started = Instant::now();

    tracing::info!(
        event = "shellcd.execution.start",
        host = %config.host,
        port = config.port,
        ssh_user = %config.user,
        script = %config.script,
        caller_email = %config.metadata.caller_email,
        gitlab_user_login = %config.metadata.gitlab_user_login,
        project_path = %config.metadata.project_path,
        pipeline_id = %config.metadata.pipeline_id,
        job_id = %config.metadata.job_id,
        commit_sha = %config.metadata.commit_sha,
        "starting remote script"
    );

    let result = ssh::run(&config);
    let (outcome, exit_code) = match &result {
        Ok(0) => ("success", 0),
        Ok(code) => ("remote_failure", *code),
        Err(error) => (error.outcome(), error.exit_code()),
    };

    tracing::info!(
        event = "shellcd.execution.finish",
        result = outcome,
        exit_code,
        duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        "remote script finished"
    );

    if config.dorarion {
        dorarion::call_dorarion_api_cd_end();
    }

    result
}
