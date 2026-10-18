//! What the browser presents to the exam server, derived from the settings.

mod fullscreen;
mod js_api;
mod quit_password;
mod quit_url;
mod reload;
mod toolbar;
mod user_agent;
mod window_title;

pub use fullscreen::fullscreen;
pub use js_api::js_api;
pub use quit_password::is_quit_password;
pub use quit_url::is_quit_url;
pub use reload::{Reload, reload};
pub use toolbar::{Toolbar, toolbar, typed_address};
pub use user_agent::user_agent;
pub use window_title::window_title;

pub(crate) const SEB_VERSION: &str = "3.10.2";
