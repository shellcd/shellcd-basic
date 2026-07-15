use std::{
    env,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{config::RunConfig, error::AppError};

pub fn posix_escape(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

pub fn remote_command(config: &RunConfig) -> String {
    let metadata = &config.metadata;
    let mut arguments = vec![
        config.script.clone(),
        "--caller-email".into(),
        posix_escape(&metadata.caller_email),
        "--gitlab-user-login".into(),
        posix_escape(&metadata.gitlab_user_login),
        "--project-path".into(),
        posix_escape(&metadata.project_path),
        "--pipeline-id".into(),
        posix_escape(&metadata.pipeline_id),
        "--job-id".into(),
        posix_escape(&metadata.job_id),
        "--commit-sha".into(),
        posix_escape(&metadata.commit_sha),
    ];
    if let Some(script_arg) = &config.script_arg {
        arguments.push(posix_escape(script_arg));
    }
    arguments.join(" ")
}

pub fn run(config: &RunConfig) -> Result<i32, AppError> {
    let ssh = find_executable("ssh")?;
    let status = Command::new(ssh)
        .args(ssh_arguments(config))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| AppError::SshStart(error.to_string()))?;

    match status.code() {
        Some(255) => Err(AppError::SshConnection),
        Some(code) => Ok(code),
        None => Err(AppError::Signaled),
    }
}

pub fn ssh_arguments(config: &RunConfig) -> Vec<OsString> {
    let options = [
        "BatchMode=yes".into(),
        "IdentitiesOnly=yes".into(),
        "IdentityAgent=none".into(),
        "StrictHostKeyChecking=yes".into(),
        path_option("UserKnownHostsFile=", &config.known_hosts_file),
        format!("ConnectTimeout={}", config.connect_timeout_seconds).into(),
        "ConnectionAttempts=1".into(),
        "LogLevel=ERROR".into(),
        "RequestTTY=no".into(),
        "ClearAllForwardings=yes".into(),
        "ExitOnForwardFailure=yes".into(),
        "ForwardAgent=no".into(),
        "ForwardX11=no".into(),
        "PasswordAuthentication=no".into(),
        "KbdInteractiveAuthentication=no".into(),
        "PreferredAuthentications=publickey".into(),
    ];
    let mut args = Vec::with_capacity(options.len() * 2 + 11);
    args.extend(["-F".into(), "/dev/null".into()]);
    for option in options {
        args.push("-o".into());
        args.push(option);
    }
    args.extend([
        "-p".into(),
        config.port.to_string().into(),
        "-i".into(),
        config.private_key_file.as_os_str().into(),
        "-l".into(),
        config.user.clone().into(),
        "--".into(),
        config.host.clone().into(),
        remote_command(config).into(),
    ]);
    args
}

fn path_option(prefix: &str, path: &Path) -> OsString {
    let mut option = OsString::from(prefix);
    option.push(path.as_os_str());
    option
}

fn find_executable(name: &str) -> Result<PathBuf, AppError> {
    let path = env::var_os("PATH")
        .ok_or_else(|| AppError::SshStart("PATH is not set; cannot locate 'ssh'".into()))?;
    for directory in env::split_paths(&path) {
        let candidate = directory.join(name);
        if is_executable(&candidate) {
            return Ok(candidate);
        }
    }
    Err(AppError::SshStart(
        "'ssh' was not found as an executable in PATH".into(),
    ))
}

fn is_executable(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::posix_escape;

    #[test]
    fn escapes_every_posix_argument() {
        assert_eq!(posix_escape(""), "''");
        assert_eq!(posix_escape("simple"), "'simple'");
        assert_eq!(posix_escape("two words"), "'two words'");
        assert_eq!(posix_escape("a'b"), "'a'\\''b'");
        assert_eq!(posix_escape("$HOME; id"), "'$HOME; id'");
        assert_eq!(posix_escape("line\nfeed"), "'line\nfeed'");
    }
}
