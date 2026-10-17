use plist::{Dictionary, Value};

/// SEB `BrowserDataMapper.MapMainWindowMode`: only the integer 1 means fullscreen.
pub fn fullscreen(settings: &Dictionary) -> bool {
    settings
        .get("browserViewMode")
        .and_then(Value::as_signed_integer)
        == Some(1)
}
