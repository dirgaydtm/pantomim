use plist::Dictionary;

use super::SEB_VERSION;

pub fn user_agent(settings: &Dictionary) -> String {
    let text = |key| settings.get(key).and_then(|v| v.as_string()).unwrap_or("");
    let custom = settings
        .get("browserUserAgentWinDesktopMode")
        .is_some_and(|mode| mode.as_signed_integer() != Some(0));

    let agent = if custom {
        format!(
            "{} SEB/{SEB_VERSION}",
            text("browserUserAgentWinDesktopModeCustom")
        )
    } else {
        format!(
            "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 SEB/{SEB_VERSION}"
        )
    };
    match text("browserUserAgent") {
        suffix if suffix.trim().is_empty() => agent,
        suffix => format!("{agent} {suffix}"),
    }
}
