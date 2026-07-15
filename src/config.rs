use std::path::PathBuf;

use crate::{
    cli::{Command, RunArgs},
    error::AppError,
    validation,
};

#[derive(Debug)]
pub struct RunConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub script: String,
    pub script_arg: Option<String>,
    pub private_key_file: PathBuf,
    pub known_hosts_file: PathBuf,
    pub metadata: Metadata,
    pub connect_timeout_seconds: u32,
    pub dorarion: bool,
}

#[derive(Debug)]
pub struct Metadata {
    pub caller_email: String,
    pub gitlab_user_login: String,
    pub project_path: String,
    pub pipeline_id: String,
    pub job_id: String,
    pub commit_sha: String,
}

impl TryFrom<Command> for RunConfig {
    type Error = AppError;

    fn try_from(command: Command) -> Result<Self, Self::Error> {
        let Command::Run(args) = command;
        Self::try_from(args)
    }
}

impl TryFrom<RunArgs> for RunConfig {
    type Error = AppError;

    fn try_from(args: RunArgs) -> Result<Self, Self::Error> {
        validation::host(&args.host)?;
        validation::username(&args.user, args.allow_root)?;
        validation::script_path(&args.script)?;
        validation::email(&args.caller_email)?;
        validation::private_key_file(&args.private_key_file)?;
        validation::nonempty_regular_file(&args.known_hosts_file, "known-hosts")?;
        validation::openssh_option_path(&args.known_hosts_file, "known-hosts")?;

        if args.port == 0 {
            return Err(AppError::Validation(
                "port must be between 1 and 65535".into(),
            ));
        }
        if args.connect_timeout_seconds == 0 {
            return Err(AppError::Validation(
                "connect timeout must be at least one second".into(),
            ));
        }

        Ok(Self {
            host: args.host,
            port: args.port,
            user: args.user,
            script: args.script,
            script_arg: args.script_arg,
            private_key_file: args.private_key_file,
            known_hosts_file: args.known_hosts_file,
            metadata: Metadata {
                caller_email: args.caller_email,
                gitlab_user_login: args.gitlab_user_login.unwrap_or_default(),
                project_path: args.project_path.unwrap_or_default(),
                pipeline_id: args.pipeline_id.unwrap_or_default(),
                job_id: args.job_id.unwrap_or_default(),
                commit_sha: args.commit_sha.unwrap_or_default(),
            },
            connect_timeout_seconds: args.connect_timeout_seconds,
            dorarion: args.dorarion,
        })
    }
}
