# Spec: `launch` module

pantomim takes one argument: a `.seb` file (Flow B) or a `seb://` / `sebs://` link from Moodle (Flow A). This module holds the rules for turning that argument into a configuration. They copy SEB for Windows (`NetworkResourceLoader.cs`, `ConfigurationOperation.cs`, `DownloadHandler.cs`, `WindowHandlingResponsibility.cs`).

## Interface

```rust
pub fn download_url(arg: &str) -> Option<String>;
pub fn is_browser_resource(status: u16, content_type: Option<&str>) -> bool;
pub fn is_configuration_download(url: &str, file_name: &str) -> bool;
pub fn start_url(settings: &plist::Dictionary, link: &str) -> Option<String>;
```

## Flow A

1. `download_url` turns `seb://` into `http://` and `sebs://` into `https://`, ignoring the case of the scheme. `http(s)://` stays as it is. Anything else, such as a file path or `file://`, is not a link and gives `None`.
2. pantomim downloads that URL with user agent `SEB/3.10.2` and follows redirects.
3. A `401` response, or a `text/html` body with any status, is a web page (`is_browser_resource`). pantomim opens it in the browser with default settings, which SEB calls `LoadWithBrowser`. Moodle's `config.php` ends up here when the student is not logged in, because it redirects to the login page.
4. Any other response body is loaded as a `.seb` file.

After logging in during a `LoadWithBrowser` session, the student gets the `.seb` from Moodle as a download. `is_configuration_download` treats a download as a configuration when the suggested file name or the URL path (without query and fragment) ends in `.seb`, in any case. pantomim then opens the new configuration in a new window and closes the old one. Cookies stay, so the login is kept.

pantomim refuses every other download. It also refuses a `.seb` download in a session that started from a configuration, which stops an exam page from swapping the configuration.

## `start_url`

`start_url` returns `startURL`, or `None` when the settings have none. If `startURLAppendQueryParameter` is `true`, part of the link's query goes on the end, following SEB's `HandleStartUrlQuery` and `GenerateStartUrl`:

- Only the part from the last `?` of the link's query counts, and only when that query holds a second `?`. `sebs://h/x.seb?a=1` adds nothing, while `sebs://h/x.seb?a=1?b=2` adds `?b=2`. The fragment is ignored.
- When `startURL` already has a query and the extra part is more than a lone `?`, the two are joined with `&`: `…?id=5` becomes `…?id=5&b=2`.
- Otherwise the extra part is appended unchanged. `…/view.php` becomes `…/view.php?b=2`, and an extra part of just `?` turns `…?id=5` into `…?id=5?`.

In Flow A the link is the argument. After a reconfiguration it is the URL the `.seb` was downloaded from.

## Test vectors

1. `download_url`: `sebs://h.id/x/config.php?cmid=1` → `https://h.id/x/config.php?cmid=1`; `seb://h.id/x.seb` → `http://h.id/x.seb`; `SEBS://h.id/x.seb` → `https://h.id/x.seb`; `https://h.id/x.seb` unchanged; `/home/a/config.seb` and `file:///home/a/config.seb` → `None`.
2. `is_browser_resource`: true for `401` with no type, `200 text/html; charset=utf-8`, `404 text/html`. False for `200 application/seb`, `200 application/octet-stream`, `200` with no type.
3. `is_configuration_download`: true for (`…/config.php?cmid=1`, `config.seb`) and (`…/Exam.SEB?a=1`, `download`). False for (`…/x.pdf`, `x.pdf`), (`…/seb`, `seb`), (`…/a.seb/b`, `b`), (`…/b?f=a.seb`, `b`).
4. `start_url` with `startURL` `https://q.id/view.php` or `https://q.id/view.php?id=5`: the three rules above, plus no change when the flag is `false`, when the link has one `?`, or when the link is a file path. Without `startURL`: `None`.
5. End to end on brone: `sebs://brone.ub.ac.id/mod/quiz/accessrule/seb/config.php?cmid=68624` opens the SSO login. After login, the `.seb` download replaces the window with the quiz page, which passes Moodle's SEB check.
