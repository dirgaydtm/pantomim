# Spec: `session` module

Everything the browser shows the exam server, worked out from the settings. It copies SEB for Windows 3.10.2 in desktop mode.

## Interface

```rust
pub fn user_agent(settings: &plist::Dictionary) -> String;
pub fn is_quit_url(settings: &plist::Dictionary, url: &str) -> bool;
pub fn js_api(config_key: &str) -> String;
pub fn is_quit_password(hash: &str, password: &str) -> bool;
pub fn fullscreen(settings: &plist::Dictionary) -> bool;
pub fn window_title<'a>(settings: &plist::Dictionary, address: Option<&'a str>, page_title: Option<&'a str>) -> Option<&'a str>;
```

## `user_agent`

1. If `browserUserAgentWinDesktopMode` is present and is anything other than integer `0`, the agent is `browserUserAgentWinDesktopModeCustom` followed by ` SEB/3.10.2`. SEB counts a non-integer value as custom too.
2. Otherwise it is `Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 SEB/3.10.2`.
3. If `browserUserAgent` is not blank, a space and its value go on the end.

## `is_quit_url`

SEB matches the URL against `^{quitURL with trailing slashes removed}/?$`, ignoring case:

- A missing or blank `quitURL` never matches.
- Otherwise the whole URL must equal `quitURL` without its trailing slashes, with at most one `/` after it.

## `js_api`

The script runs in every frame when its document starts (`initialization_script_for_all_frames`) and defines:

```js
SafeExamBrowser = {
  version: "SEB_Windows_3.10.2",
  security: { browserExamKey: "", configKey: "<hash>", updateKeys: (callback) => callback() }
}
```

- `configKey` is the SHA-256 hex of the frame's URL without its fragment, followed by the Config Key. This is the same value `config_key_hash` gives.
- The hash is computed synchronously by a small SHA-256 inside the script, so `configKey` is set before any page script runs, as in SEB for Windows. Moodle (`quizaccess_seb`, `validate_quiz_access.js`) then validates it straight away.
- Keys must never start as `":"`. Moodle reads that as "not ready" and calls `updateKeys(safeExamBrowserKeysUpdated)`, which runs without the quiz `cmid`, so validation never finishes and the page stays on "Checking access to Safe Exam Browser...".
- `browserExamKey` is empty because pantomim cannot produce a Browser Exam Key.

## `is_quit_password`

SEB `ClientResponsibility.IsValidQuitPassword`: the hex SHA-256 of the UTF-8 password equals `hashedQuitPassword`, ignoring case.

## `fullscreen`

SEB `BrowserDataMapper.MapMainWindowMode`: the exam window is fullscreen only when `browserViewMode` is the integer `1`.

## `window_title`

SEB `ControlResponsibility`: the exam window title follows `browserWindowShowURL` (default 0). When the page moves to a new address, the title becomes the address for 3 (always) and 2 (before the title). When the page names itself, the title becomes that name unless the policy is 3. `None` keeps the current title.

## Test vectors

1. `user_agent`:
   - empty settings, or mode `0` with a custom string: the default agent from rule 2.
   - mode `1` with custom `Foo/1.0`: `Foo/1.0 SEB/3.10.2`.
   - `browserUserAgent` = `Bar/2`: the default agent followed by ` Bar/2`. A value of `" "` adds nothing.
2. `is_quit_url` with `quitURL` = `https://a.b/quit/`:
   - matches `https://a.b/quit`, `https://a.b/quit/`, `HTTPS://A.B/QUIT`.
   - does not match `https://a.b/quit/x`, `https://a.b/quit?x`, `https://a.b/quit//`.
   - with no `quitURL`, or `quitURL` = `" "`, nothing matches.
3. `js_api` with Config Key `866101fb…7d31` in a frame at `https://brone.ub.ac.id/mod/quiz/view.php?id=68624#x`: `configKey` is `415a4047804b9b5162b8d1190176319c210927ca5423aded7c4b91b5465fef71`, the value Moodle accepted.
4. `is_quit_password` with hash `ba7816bf…15ad` (SHA-256 of `abc`, FIPS 180-2): `abc` matches, also against the upper-case hash; `ABC` and `` do not. The Moodle Sandbox writes this same hash for quit password `abc`.
5. `fullscreen`: `browserViewMode` = integer `1` → true; `0`, `2`, string `"1"`, `true`, or missing → false.
6. `window_title` with address `https://a.b/x` and title `Quiz`: no setting, 0 or 1 → address keeps the title, title `Quiz`; 2 → address, then `Quiz`; 3 → address, and the page title is ignored.
