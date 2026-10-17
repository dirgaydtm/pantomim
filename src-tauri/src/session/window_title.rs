use plist::{Dictionary, Value};

/// The exam window title after the page moved to `address` or named itself `page_title` (one of
/// the two), or `None` to keep it. SEB `ControlResponsibility` with `browserWindowShowURL`:
/// 3 always shows the address, 2 shows it until the page has a title, anything else never.
pub fn window_title<'a>(
    settings: &Dictionary,
    address: Option<&'a str>,
    page_title: Option<&'a str>,
) -> Option<&'a str> {
    let policy = settings
        .get("browserWindowShowURL")
        .and_then(Value::as_signed_integer);
    match policy {
        Some(3) => address,
        Some(2) => address.or(page_title),
        _ => page_title,
    }
}
