use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "shellcd-basic", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run one approved script on a remote host.
    Run(RunArgs),
}

#[derive(Clone, Debug, Args)]
pub struct RunArgs {
    #[arg(long, env = "SHELLCD_HOST")]
    pub host: String,

    #[arg(long, env = "SHELLCD_PORT", default_value_t = 22)]
    pub port: u16,

    #[arg(long = "user", env = "SHELLCD_SSH_USER")]
    pub user: String,

    #[arg(long, env = "SHELLCD_SCRIPT")]
    pub script: String,

    /// Optional single positional argument passed to the remote script.
    #[arg(long, env = "SHELLCD_SCRIPT_ARG")]
    pub script_arg: Option<String>,

    #[arg(long, env = "SHELLCD_PRIVATE_KEY_FILE")]
    pub private_key_file: PathBuf,

    #[arg(long, env = "SHELLCD_KNOWN_HOSTS_FILE")]
    pub known_hosts_file: PathBuf,

    #[arg(long, env = "GITLAB_USER_EMAIL")]
    pub caller_email: String,

    #[arg(long, env = "GITLAB_USER_LOGIN")]
    pub gitlab_user_login: Option<String>,

    #[arg(long, env = "CI_PROJECT_PATH")]
    pub project_path: Option<String>,

    #[arg(long, env = "CI_PIPELINE_ID")]
    pub pipeline_id: Option<String>,

    #[arg(long, env = "CI_JOB_ID")]
    pub job_id: Option<String>,

    #[arg(long, env = "CI_COMMIT_SHA")]
    pub commit_sha: Option<String>,

    #[arg(long, env = "SHELLCD_CONNECT_TIMEOUT_SECONDS", default_value_t = 15)]
    pub connect_timeout_seconds: u32,

    #[arg(long)]
    pub allow_root: bool,

    /// Enable the no-op Dorarion start and end hooks.
    #[arg(
        long,
        env = "SHELLCD_DORARION",
        default_value_t = false,
        action = clap::ArgAction::Set,
        value_parser = parse_dorarion
    )]
    pub dorarion: bool,
}

fn parse_dorarion(value: &str) -> Result<bool, String> {
    match value {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err("expected one of: true, false, 1, 0".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_dorarion;

    #[test]
    fn parses_dorarion_values() {
        assert_eq!(parse_dorarion("true"), Ok(true));
        assert_eq!(parse_dorarion("1"), Ok(true));
        assert_eq!(parse_dorarion("false"), Ok(false));
        assert_eq!(parse_dorarion("0"), Ok(false));
        assert!(parse_dorarion("yes").is_err());
    }
}
