use pantomim_lib::session::{
    Reload, Toolbar, fullscreen, is_quit_password, is_quit_url, js_api, reload, toolbar,
    typed_address, user_agent, window_title,
};
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

#[test]
fn window_title_follows_browser_window_show_url() {
    // SEB ControlResponsibility: Control_AddressChanged shows the address for policy Always (3) and
    // BeforeTitle (2); Control_TitleChanged shows the page title unless the policy is Always.
    let policy = |p: i64| settings(&[("browserWindowShowURL", p.into())]);
    let (address, title) = (Some("https://a.b/x"), Some("Quiz"));
    for (settings, on_address, on_title) in [
        (settings(&[]), None, title),
        (policy(0), None, title),
        (policy(1), None, title),
        (policy(2), address, title),
        (policy(3), address, None),
    ] {
        assert_eq!(window_title(&settings, address, None), on_address);
        assert_eq!(window_title(&settings, None, title), on_title);
    }
}

#[test]
fn reload_follows_browser_window_settings() {
    // SEB WindowResponsibility.RequestPageReload; DataValues: AllowReloading and ShowReloadWarning
    // default to true, and BrowserDataMapper only takes booleans.
    let cases = [
        (settings(&[]), Reload::Ask),
        (
            settings(&[("browserWindowAllowReload", true.into())]),
            Reload::Ask,
        ),
        (
            settings(&[("showReloadWarning", false.into())]),
            Reload::Now,
        ),
        (
            settings(&[("browserWindowAllowReload", false.into())]),
            Reload::Blocked,
        ),
        (
            settings(&[
                ("browserWindowAllowReload", false.into()),
                ("showReloadWarning", false.into()),
            ]),
            Reload::Blocked,
        ),
        (
            settings(&[("browserWindowAllowReload", 0.into())]),
            Reload::Ask,
        ),
    ];
    for (settings, expected) in cases {
        assert_eq!(reload(&settings), expected);
    }
}

#[test]
fn toolbar_buttons_follow_seb_settings() {
    // SEB DataValues (all off by default, reloading allowed), DataProcessor (home button when a
    // home URL is set) and BrowserWindow.ApplySettings.
    assert_eq!(
        toolbar(&settings(&[])),
        Toolbar {
            home: false,
            navigation: false,
            reload: false,
            reload_enabled: true,
            address_bar: false
        }
    );
    let all = settings(&[
        ("restartExamUseStartURL", true.into()),
        ("allowBrowsingBackForward", true.into()),
        ("showReloadButton", true.into()),
        ("browserWindowAllowReload", false.into()),
        ("browserWindowAllowAddressBar", true.into()),
    ]);
    assert_eq!(
        toolbar(&all),
        Toolbar {
            home: true,
            navigation: true,
            reload: true,
            reload_enabled: false,
            address_bar: true
        }
    );
    assert!(toolbar(&settings(&[("restartExamURL", "https://a.b/".into())])).home);
    assert!(!toolbar(&settings(&[("restartExamURL", " ".into())])).home);
}

#[test]
fn typed_address_is_absolute_or_https() {
    // SEB DisplayResponsibility.Window_AddressChanged: an absolute URI, or one that becomes valid
    // with https:// in front; anything else clears the address bar.
    assert_eq!(typed_address("https://a.b/x"), Some("https://a.b/x".into()));
    assert_eq!(
        typed_address("moodle.ub.ac.id/my"),
        Some("https://moodle.ub.ac.id/my".into())
    );
    assert_eq!(typed_address(""), None);
    assert_eq!(typed_address("not a url"), None);
}
