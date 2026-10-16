use super::SEB_VERSION;

pub fn js_api(config_key: &str) -> String {
    include_str!("api.js")
        .replace("__CONFIG_KEY__", config_key)
        .replace("__SEB_VERSION__", SEB_VERSION)
}
