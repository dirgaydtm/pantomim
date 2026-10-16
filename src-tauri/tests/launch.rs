use pantomim_lib::launch::{
    download_url, is_browser_resource, is_configuration_download, start_url,
};
use plist::{Dictionary, Value};

#[test]
fn download_url_maps_seb_schemes_to_http() {
    let cases = [
        (
            "sebs://h.id/x/config.php?cmid=1",
            Some("https://h.id/x/config.php?cmid=1"),
        ),
        ("seb://h.id/x.seb", Some("http://h.id/x.seb")),
        ("SEBS://h.id/x.seb", Some("https://h.id/x.seb")),
        ("https://h.id/x.seb", Some("https://h.id/x.seb")),
        ("/home/a/config.seb", None),
        ("file:///home/a/config.seb", None),
    ];
    for (arg, expected) in cases {
        assert_eq!(download_url(arg).as_deref(), expected, "{arg}");
    }
}

#[test]
fn html_or_unauthorized_response_opens_in_browser() {
    let cases = [
        (401, None, true),
        (200, Some("text/html; charset=utf-8"), true),
        (404, Some("text/html"), true),
        (200, Some("application/seb"), false),
        (200, Some("application/octet-stream"), false),
        (200, None, false),
    ];
    for (status, content_type, expected) in cases {
        assert_eq!(
            is_browser_resource(status, content_type),
            expected,
            "{status} {content_type:?}"
        );
    }
}

#[test]
fn seb_extension_in_file_name_or_url_path_is_a_configuration() {
    let cases = [
        (
            "https://h.id/mod/quiz/accessrule/seb/config.php?cmid=1",
            "config.seb",
            true,
        ),
        ("https://h.id/x/Exam.SEB?a=1", "download", true),
        ("https://h.id/x.pdf", "x.pdf", false),
        ("https://h.id/seb", "seb", false),
        ("https://h.id/a.seb/b", "b", false),
        ("https://h.id/b?f=a.seb", "b", false),
    ];
    for (url, file_name, expected) in cases {
        assert_eq!(
            is_configuration_download(url, file_name),
            expected,
            "{url} {file_name}"
        );
    }
}

#[test]
fn start_url_appends_query_after_the_last_question_mark_of_the_link() {
    let settings = |start: &str, append: bool| -> Dictionary {
        [
            ("startURL".to_string(), Value::from(start)),
            ("startURLAppendQueryParameter".to_string(), append.into()),
        ]
        .into_iter()
        .collect()
    };
    let plain = "https://q.id/view.php";
    let with_query = "https://q.id/view.php?id=5";
    let cases = [
        (
            settings(plain, true),
            "sebs://h.id/x.seb?a=1?b=2",
            "https://q.id/view.php?b=2",
        ),
        (
            settings(with_query, true),
            "sebs://h.id/x.seb?a=1?b=2#f",
            "https://q.id/view.php?id=5&b=2",
        ),
        (
            settings(with_query, true),
            "sebs://h.id/x.seb?a=1?",
            "https://q.id/view.php?id=5?",
        ),
        (
            settings(with_query, true),
            "sebs://h.id/x.seb?a=1",
            with_query,
        ),
        (
            settings(with_query, false),
            "sebs://h.id/x.seb?a=1?b=2",
            with_query,
        ),
        (settings(plain, true), "/home/a/config.seb", plain),
    ];
    for (settings, link, expected) in cases {
        assert_eq!(
            start_url(&settings, link).as_deref(),
            Some(expected),
            "{link}"
        );
    }
    assert_eq!(start_url(&Dictionary::new(), "/a.seb"), None);
}
