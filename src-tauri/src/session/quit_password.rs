use sha2::{Digest, Sha256};

/// SEB `ClientResponsibility.IsValidQuitPassword`: hex SHA-256 of the UTF-8 password, ignoring case.
pub fn is_quit_password(hash: &str, password: &str) -> bool {
    hash.eq_ignore_ascii_case(&hex::encode(Sha256::digest(password)))
}
