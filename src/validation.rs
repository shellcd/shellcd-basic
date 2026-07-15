use std::{fs, net::IpAddr, path::Path};

use crate::error::AppError;

fn invalid(message: impl Into<String>) -> AppError {
    AppError::Validation(message.into())
}

pub fn host(value: &str) -> Result<(), AppError> {
    if value.is_empty() {
        return Err(invalid("host must not be empty"));
    }
    if value.starts_with('-') {
        return Err(invalid("host must not begin with '-'"));
    }
    if value.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(invalid(
            "host must not contain whitespace or control characters",
        ));
    }
    if value.contains("@") || value.contains("://") {
        return Err(invalid("host must be a bare IP address or DNS hostname"));
    }
    if value.parse::<IpAddr>().is_ok() {
        return Ok(());
    }

    let hostname = value.strip_suffix('.').unwrap_or(value);
    if hostname.is_empty() || value.len() > 253 {
        return Err(invalid("host is not a valid DNS hostname"));
    }
    let valid = hostname.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
    });
    if !valid {
        return Err(invalid("host is not a valid IP address or DNS hostname"));
    }
    Ok(())
}

pub fn username(value: &str, allow_root: bool) -> Result<(), AppError> {
    let mut chars = value.chars();
    let first_valid = chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c == '_');
    let rest_valid =
        chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    if value.len() > 32 || !first_valid || !rest_valid {
        return Err(invalid("SSH user must match ^[a-z_][a-z0-9_-]{0,31}$"));
    }
    if value == "root" && !allow_root {
        return Err(invalid("SSH user 'root' requires --allow-root"));
    }
    Ok(())
}

pub fn script_path(value: &str) -> Result<(), AppError> {
    const PREFIX: &str = "/opt/shellcd/scripts/";

    if !value.starts_with(PREFIX) || value.len() == PREFIX.len() {
        return Err(invalid(format!("script must be below {PREFIX}")));
    }
    if !value.ends_with(".sh") {
        return Err(invalid("script must end in .sh"));
    }
    if value.contains("..") {
        return Err(invalid("script must not contain '..'"));
    }
    if value.contains("//") {
        return Err(invalid("script must not contain repeated path separators"));
    }
    if !value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'_' | b'-' | b'.'))
    {
        return Err(invalid(
            "script may contain only ASCII letters, digits, '/', '_', '-', and '.'",
        ));
    }
    Ok(())
}

pub fn email(value: &str) -> Result<(), AppError> {
    if value.len() > 254
        || value.chars().any(|c| c.is_whitespace() || c.is_control())
        || value.matches('@').count() != 1
    {
        return Err(invalid("caller email is not a valid basic email address"));
    }
    let (local, domain) = value.split_once('@').unwrap_or_default();
    if local.is_empty() || domain.is_empty() {
        return Err(invalid(
            "caller email must have non-empty local and domain parts",
        ));
    }
    Ok(())
}

pub fn nonempty_regular_file(path: &Path, label: &str) -> Result<(), AppError> {
    let metadata = fs::metadata(path)
        .map_err(|error| invalid(format!("{label} file '{}': {error}", path.display())))?;
    if !metadata.is_file() {
        return Err(invalid(format!("{label} file must be a regular file")));
    }
    if metadata.len() == 0 {
        return Err(invalid(format!("{label} file must not be empty")));
    }
    Ok(())
}

pub fn private_key_file(path: &Path) -> Result<(), AppError> {
    nonempty_regular_file(path, "private key")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path)
            .map_err(|error| invalid(format!("private key file '{}': {error}", path.display())))?
            .permissions()
            .mode();
        if mode & 0o077 != 0 {
            return Err(invalid(
                "private key file permissions allow group or other access; use chmod 600",
            ));
        }
    }
    Ok(())
}

pub fn openssh_option_path(path: &Path, label: &str) -> Result<(), AppError> {
    let value = path
        .to_str()
        .ok_or_else(|| invalid(format!("{label} path must be valid UTF-8")))?;
    if value
        .chars()
        .any(|c| c.is_whitespace() || c.is_control() || matches!(c, '\\' | '\'' | '"'))
    {
        return Err(invalid(format!(
            "{label} path must not contain whitespace, quotes, backslashes, or control characters"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn accepts_valid_hosts() {
        for value in [
            "192.168.10.20",
            "2001:db8::1",
            "deploy-1.example.com",
            "localhost",
        ] {
            assert!(host(value).is_ok(), "{value}");
        }
    }

    #[test]
    fn rejects_invalid_hosts() {
        for value in [
            "",
            "bad host",
            "-host",
            "deploy@host",
            "ssh://host",
            "bad_host",
        ] {
            assert!(host(value).is_err(), "{value}");
        }
    }

    #[test]
    fn validates_usernames_and_root_policy() {
        for value in ["deploy", "_deploy", "deploy-01"] {
            assert!(username(value, false).is_ok(), "{value}");
        }
        for value in [
            "Deploy",
            "9deploy",
            "deploy.user",
            "",
            "abcdefghijklmnopqrstuvwxyzabcdefg",
        ] {
            assert!(username(value, false).is_err(), "{value}");
        }
        assert!(username("root", false).is_err());
        assert!(username("root", true).is_ok());
    }

    #[test]
    fn validates_script_paths() {
        assert!(script_path("/opt/shellcd/scripts/deploy-api.sh").is_ok());
        for value in [
            "deploy.sh",
            "/tmp/deploy.sh",
            "/opt/shellcd/scripts/../deploy.sh",
            "/opt/shellcd/scripts/deploy api.sh",
            "/opt/shellcd/scripts/deploy;id.sh",
            "/opt/shellcd/scripts/deploy'api.sh",
            "/opt/shellcd/scripts/deploy\n.sh",
            "/opt/shellcd/scripts//deploy.sh",
        ] {
            assert!(script_path(value).is_err(), "{value:?}");
        }
    }

    #[test]
    fn validates_basic_email_addresses() {
        assert!(email("developer@example.com").is_ok());
        for value in [
            "",
            "developer",
            "@example.com",
            "dev@",
            "a@b@c",
            "a b@example.com",
        ] {
            assert!(email(value).is_err(), "{value:?}");
        }
    }

    #[test]
    fn rejects_empty_files() -> Result<(), Box<dyn std::error::Error>> {
        let file = tempfile::NamedTempFile::new()?;
        assert!(nonempty_regular_file(file.path(), "test").is_err());
        Ok(())
    }

    #[test]
    fn rejects_ssh_config_syntax_in_option_paths() {
        assert!(openssh_option_path(Path::new("/run/secrets/known_hosts"), "test").is_ok());
        assert!(openssh_option_path(Path::new("/run/secrets/known hosts"), "test").is_err());
        assert!(openssh_option_path(Path::new("/run/secrets/known\\hosts"), "test").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_insecure_private_key_permissions() -> Result<(), Box<dyn std::error::Error>> {
        use std::os::unix::fs::PermissionsExt;
        let mut file = tempfile::NamedTempFile::new()?;
        file.write_all(b"not-a-real-secret")?;
        fs::set_permissions(file.path(), fs::Permissions::from_mode(0o640))?;
        assert!(private_key_file(file.path()).is_err());
        fs::set_permissions(file.path(), fs::Permissions::from_mode(0o600))?;
        assert!(private_key_file(file.path()).is_ok());
        Ok(())
    }
}
