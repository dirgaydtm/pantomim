//! What the browser presents to the exam server, derived from the settings.

mod fullscreen;
mod js_api;
mod quit_password;
mod quit_url;
mod user_agent;

pub use fullscreen::fullscreen;
pub use js_api::js_api;
pub use quit_password::is_quit_password;
pub use quit_url::is_quit_url;
pub use user_agent::user_agent;

pub(crate) const SEB_VERSION: &str = "3.10.2";
