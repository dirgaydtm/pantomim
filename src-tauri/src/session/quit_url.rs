use plist::Dictionary;

pub fn is_quit_url(settings: &Dictionary, url: &str) -> bool {
    match settings.get("quitURL").and_then(|v| v.as_string()) {
        Some(quit) if !quit.trim().is_empty() => {
            let url = url.strip_suffix('/').unwrap_or(url);
            url.eq_ignore_ascii_case(quit.trim_end_matches('/'))
        }
        _ => false,
    }
}
