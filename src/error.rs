use thiserror::Error;

pub const VALIDATION_EXIT_CODE: i32 = 2;
pub const SSH_START_EXIT_CODE: i32 = 3;
pub const SSH_CONNECTION_EXIT_CODE: i32 = 4;
pub const SIGNAL_EXIT_CODE: i32 = 5;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid input: {0}")]
    Validation(String),

    #[error("could not start the OpenSSH client: {0}")]
    SshStart(String),

    #[error("SSH connection failed before a remote exit status was available")]
    SshConnection,

    #[error("SSH client was terminated by a signal")]
    Signaled,
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => VALIDATION_EXIT_CODE,
            Self::SshStart(_) => SSH_START_EXIT_CODE,
            Self::SshConnection => SSH_CONNECTION_EXIT_CODE,
            Self::Signaled => SIGNAL_EXIT_CODE,
        }
    }

    pub fn outcome(&self) -> &'static str {
        match self {
            Self::Validation(_) => "validation_failure",
            Self::SshStart(_) => "ssh_start_failure",
            Self::SshConnection => "connection_failure",
            Self::Signaled => "signal_failure",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_have_stable_exit_codes() {
        assert_eq!(AppError::Validation("bad".into()).exit_code(), 2);
        assert_eq!(AppError::SshStart("missing".into()).exit_code(), 3);
        assert_eq!(AppError::SshConnection.exit_code(), 4);
        assert_eq!(AppError::Signaled.exit_code(), 5);
    }
}
