use crate::configuration::Error;

/// Title and message text keys for a configuration that could not be loaded
/// (SEB `ConfigurationOperation.ShowFailureMessage`). `None`: not a configuration error (download, file).
pub fn failure_message(error: Option<&Error>) -> (&'static str, &'static str) {
    match error {
        Some(Error::InvalidData(_)) => (
            "MessageBox_InvalidConfigurationDataTitle",
            "MessageBox_InvalidConfigurationData",
        ),
        Some(Error::NotSupported(_)) => (
            "MessageBox_NotSupportedConfigurationResourceTitle",
            "MessageBox_NotSupportedConfigurationResource",
        ),
        // Passwords are asked until right, so they never fail here.
        _ => (
            "MessageBox_UnexpectedConfigurationErrorTitle",
            "MessageBox_UnexpectedConfigurationError",
        ),
    }
}
