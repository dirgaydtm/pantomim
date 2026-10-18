use plist::{Dictionary, Value};
use tauri::Url;

/// Which toolbar buttons the exam window shows (SEB `BrowserWindow.ApplySettings`).
#[derive(Debug, PartialEq)]
pub struct Toolbar {
    pub home: bool,
    pub navigation: bool,
    pub reload: bool,
    pub reload_enabled: bool,
    pub address_bar: bool,
}

pub fn toolbar(settings: &Dictionary) -> Toolbar {
    let flag = |key, default| {
        settings
            .get(key)
            .and_then(Value::as_boolean)
            .unwrap_or(default)
    };
    let home_url = settings.get("restartExamURL").and_then(Value::as_string);
    Toolbar {
        // DataProcessor: a home URL, or the start URL used as one.
        home: flag("restartExamUseStartURL", false)
            || home_url.is_some_and(|u| !u.trim().is_empty()),
        navigation: flag("allowBrowsingBackForward", false),
        reload: flag("showReloadButton", false),
        reload_enabled: flag("browserWindowAllowReload", true),
        address_bar: flag("browserWindowAllowAddressBar", false),
    }
}

/// Where the address bar navigates to (SEB `DisplayResponsibility.Window_AddressChanged`).
/// SEB hands a scheme-less address to Chromium as is; WebKit needs the https:// it was checked with.
pub fn typed_address(text: &str) -> Option<String> {
    let text = text.trim();
    Url::parse(text)
        .or_else(|_| Url::parse(&format!("https://{text}")))
        .ok()
        .filter(|_| !text.is_empty())
        .map(String::from)
}
