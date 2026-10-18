use plist::{Dictionary, Value};

#[derive(Debug, PartialEq)]
pub enum Reload {
    Blocked,
    Ask,
    Now,
}

/// What F5 does on the exam page (SEB `WindowResponsibility.RequestPageReload`).
/// Both settings default to true and only booleans count, as in `BrowserDataMapper`.
pub fn reload(settings: &Dictionary) -> Reload {
    let flag = |key| {
        settings
            .get(key)
            .and_then(Value::as_boolean)
            .unwrap_or(true)
    };
    match (flag("browserWindowAllowReload"), flag("showReloadWarning")) {
        (false, _) => Reload::Blocked,
        (true, true) => Reload::Ask,
        (true, false) => Reload::Now,
    }
}
