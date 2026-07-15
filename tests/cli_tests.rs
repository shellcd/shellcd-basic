#![cfg(unix)]

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Output},
};

struct Fixture {
    _directory: tempfile::TempDir,
    ssh_dir: PathBuf,
    arguments: PathBuf,
    key: PathBuf,
    known_hosts: PathBuf,
}

impl Fixture {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let ssh_dir = directory.path().join("bin");
        fs::create_dir(&ssh_dir)?;
        let ssh = ssh_dir.join("ssh");
        fs::write(
            &ssh,
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$FAKE_SSH_ARGS\"\nprintf 'remote stdout\\n'\nprintf 'remote stderr\\n' >&2\nexit \"${FAKE_SSH_EXIT:-0}\"\n",
        )?;
        fs::set_permissions(&ssh, fs::Permissions::from_mode(0o755))?;

        let key = directory.path().join("id ed25519");
        fs::write(&key, "TOP_SECRET_PRIVATE_KEY_CONTENT")?;
        fs::set_permissions(&key, fs::Permissions::from_mode(0o600))?;
        let known_hosts = directory.path().join("known_hosts");
        fs::write(&known_hosts, "TOP_SECRET_HOST_KEY_CONTENT")?;

        Ok(Self {
            arguments: directory.path().join("arguments"),
            _directory: directory,
            ssh_dir,
            key,
            known_hosts,
        })
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_shellcd-basic"));
        command
            .env("PATH", &self.ssh_dir)
            .env("FAKE_SSH_ARGS", &self.arguments)
            .args([
                "run",
                "--host",
                "server.example.com",
                "--user",
                "deploy",
                "--script",
                "/opt/shellcd/scripts/deploy-api.sh",
                "--private-key-file",
            ])
            .arg(&self.key)
            .arg("--known-hosts-file")
            .arg(&self.known_hosts)
            .args([
                "--caller-email",
                "developer@example.com",
                "--project-path",
                "example/api service",
            ]);
        command
    }

    fn recorded_arguments(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(&self.arguments)?
            .lines()
            .map(str::to_owned)
            .collect())
    }
}

fn combined_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn invokes_ssh_directly_with_secure_options_and_clean_streams()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = Fixture::new()?;
    let output = fixture
        .command()
        .args([
            "--script-arg",
            "release 'candidate'; echo unsafe",
            "--dorarion",
            "1",
        ])
        .output()?;

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout)?, "remote stdout\n");
    let stderr = String::from_utf8(output.stderr)?;
    assert!(stderr.contains("remote stderr\n"));
    assert!(stderr.contains("shellcd.execution.start"));
    assert!(stderr.contains("shellcd.execution.finish"));
    assert!(!stderr.contains("TOP_SECRET_PRIVATE_KEY_CONTENT"));
    assert!(!stderr.contains("TOP_SECRET_HOST_KEY_CONTENT"));

    let args = fixture.recorded_arguments()?;
    for required in [
        "BatchMode=yes",
        "IdentitiesOnly=yes",
        "IdentityAgent=none",
        "StrictHostKeyChecking=yes",
        "ConnectionAttempts=1",
        "RequestTTY=no",
        "ClearAllForwardings=yes",
        "ForwardAgent=no",
        "ForwardX11=no",
        "PasswordAuthentication=no",
        "KbdInteractiveAuthentication=no",
        "PreferredAuthentications=publickey",
    ] {
        assert!(
            args.iter().any(|argument| argument == required),
            "{required}"
        );
    }
    assert!(args.windows(2).any(|pair| pair == ["-F", "/dev/null"]));
    assert!(
        args.windows(2)
            .any(|pair| pair == ["-i", fixture.key.to_string_lossy().as_ref()])
    );
    assert!(args.iter().any(|argument| {
        argument == &format!("UserKnownHostsFile={}", fixture.known_hosts.display())
    }));
    let remote = args.last().ok_or("missing remote command")?;
    assert!(remote.starts_with("/opt/shellcd/scripts/deploy-api.sh "));
    assert!(remote.contains("--project-path 'example/api service'"));
    assert!(remote.ends_with("'release '\\''candidate'\\''; echo unsafe'"));
    assert!(
        !args
            .iter()
            .any(|argument| argument == "sh" || argument == "-c")
    );
    Ok(())
}

#[test]
fn maps_remote_and_transport_exit_codes() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = Fixture::new()?;
    let remote = fixture.command().env("FAKE_SSH_EXIT", "42").output()?;
    assert_eq!(remote.status.code(), Some(42));

    let transport = fixture.command().env("FAKE_SSH_EXIT", "255").output()?;
    assert_eq!(transport.status.code(), Some(4));
    assert!(combined_output(&transport).contains("connection_failure"));
    Ok(())
}

#[test]
fn command_line_overrides_environment() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = Fixture::new()?;
    let output = fixture
        .command()
        .env("SHELLCD_HOST", "wrong.example.com")
        .output()?;
    assert_eq!(output.status.code(), Some(0));
    let args = fixture.recorded_arguments()?;
    assert!(args.iter().any(|argument| argument == "server.example.com"));
    assert!(!args.iter().any(|argument| argument == "wrong.example.com"));
    Ok(())
}

#[test]
fn new_arguments_are_optional() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = Fixture::new()?;
    let output = fixture.command().output()?;
    assert_eq!(output.status.code(), Some(0));
    let remote = fixture
        .recorded_arguments()?
        .pop()
        .ok_or("missing command")?;
    assert!(remote.ends_with("--commit-sha ''"));
    Ok(())
}

#[test]
fn maps_validation_and_missing_client_errors() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = Fixture::new()?;
    let invalid = fixture.command().arg("--port").arg("0").output()?;
    assert_eq!(invalid.status.code(), Some(2));

    let invalid_dorarion = fixture.command().args(["--dorarion", "yes"]).output()?;
    assert_eq!(invalid_dorarion.status.code(), Some(2));

    let empty_path = tempfile::tempdir()?;
    let missing = fixture.command().env("PATH", empty_path.path()).output()?;
    assert_eq!(missing.status.code(), Some(3));
    assert!(combined_output(&missing).contains("ssh_start_failure"));
    Ok(())
}
