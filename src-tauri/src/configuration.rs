//! Reading SEB configuration files and computing the Config Key sent to the exam server.

mod container;
mod rncryptor;
mod seb_json;

use std::fmt;

use sha2::{Digest, Sha256};

pub use container::load;
pub use seb_json::to_seb_json;

pub fn config_key(settings: &plist::Dictionary) -> String {
    hex::encode(Sha256::digest(to_seb_json(settings)))
}

/// Hash of the URL (without fragment) followed by the Config Key.
pub fn config_key_hash(config_key: &str, url: &str) -> String {
    let url = url.split('#').next().unwrap_or(url);
    hex::encode(Sha256::digest(format!("{url}{config_key}")))
}

#[derive(Debug, PartialEq)]
pub enum Error {
    PasswordNeeded,
    WrongPassword,
    NotSupported(String),
    InvalidData(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PasswordNeeded => f.write_str("a password is needed to open this configuration"),
            Error::WrongPassword => f.write_str("wrong password"),
            Error::NotSupported(what) => write!(f, "not supported: {what}"),
            Error::InvalidData(why) => write!(f, "invalid configuration data: {why}"),
        }
    }
}

impl std::error::Error for Error {}
