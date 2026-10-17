use pantomim_lib::session::{fullscreen, is_quit_password, is_quit_url, js_api, user_agent};
use plist::{Dictionary, Value};

fn settings(entries: &[(&str, Value)]) -> Dictionary {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

#[test]
fn user_agent_follows_seb_windows() {
    let default = "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 SEB/3.10.2";
    let custom = |mode: i64| {
        [
            ("browserUserAgentWinDesktopMode", Value::from(mode)),
            ("browserUserAgentWinDesktopModeCustom", "Foo/1.0".into()),
        ]
    };
    let cases = [
        (settings(&[]), default.to_string()),
        (settings(&custom(0)), default.to_string()),
        (settings(&custom(1)), "Foo/1.0 SEB/3.10.2".into()),
        (
            settings(&[("browserUserAgent", "Bar/2".into())]),
            format!("{default} Bar/2"),
        ),
        (
            settings(&[("browserUserAgent", " ".into())]),
            default.into(),
        ),
    ];
    for (settings, expected) in cases {
        assert_eq!(user_agent(&settings), expected, "{settings:?}");
    }
}

#[test]
fn quit_url_matches_whole_url_ignoring_case_and_trailing_slash() {
    let quit = settings(&[("quitURL", "https://a.b/quit/".into())]);
    for url in ["https://a.b/quit", "https://a.b/quit/", "HTTPS://A.B/QUIT"] {
        assert!(is_quit_url(&quit, url), "{url}");
    }
    for url in [
        "https://a.b/quit/x",
        "https://a.b/quit?x",
        "https://a.b/quit//",
    ] {
        assert!(!is_quit_url(&quit, url), "{url}");
    }

    let blank = settings(&[("quitURL", " ".into())]);
    for settings in [settings(&[]), blank] {
        assert!(!is_quit_url(&settings, " "));
    }
}

#[test]
fn js_api_embeds_config_key_and_version() {
    let key = "866101fbdbcce2b3e9df1b4ca0a25e19e034f8b271f51b26bd884f162dd97d31";
    let script = js_api(key);
    assert!(script.contains(&format!("url + \"{key}\"")));
    assert!(script.contains("\"SEB_Windows_3.10.2\""));
}

#[test]
fn quit_password_is_sha256_hex_ignoring_case() {
    // SHA-256("abc") from FIPS 180-2.
    let hash = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    assert!(is_quit_password(hash, "abc"));
    assert!(is_quit_password(&hash.to_uppercase(), "abc"));
    assert!(!is_quit_password(hash, "ABC"));
    assert!(!is_quit_password(hash, ""));
}

#[test]
fn fullscreen_only_for_integer_view_mode_1() {
    // SEB BrowserDataMapper.MapMainWindowMode: FullScreenMode = value is int && value == 1.
    assert!(fullscreen(&settings(&[("browserViewMode", 1.into())])));
    for value in [
        Value::from(0),
        Value::from(2),
        Value::from("1"),
        Value::from(true),
    ] {
        assert!(!fullscreen(&settings(&[("browserViewMode", value)])));
    }
    assert!(!fullscreen(&settings(&[])));
}
