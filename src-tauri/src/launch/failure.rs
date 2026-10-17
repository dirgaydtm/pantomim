use crate::configuration::Error;

/// Text key of the message for a configuration that could not be loaded (SEB
/// `ConfigurationOperation.ShowFailureMessage`). `None`: not a configuration error (download, file).
pub fn failure_message(error: Option<&Error>) -> &'static str {
    match error {
        Some(Error::InvalidData(_)) => "MessageBox_InvalidConfigurationData",
        Some(Error::NotSupported(_)) => "MessageBox_NotSupportedConfigurationResource",
        // Passwords are asked until right, so they never fail here.
        _ => "MessageBox_UnexpectedConfigurationError",
    }
}
