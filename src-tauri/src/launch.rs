//! How pantomim gets its configuration: a `.seb` file or a `seb(s)://` link.

mod download;
mod failure;
mod start_url;

pub use download::{download_url, is_browser_resource, is_configuration_download};
pub use failure::failure_message;
pub use start_url::start_url;
