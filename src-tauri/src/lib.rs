pub mod configuration;
pub mod launch;
pub mod session;
mod shell;

use plist::Dictionary;

struct Session {
    settings: Dictionary,
    start_url: String,
    from_browser: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let session = match open(std::env::args().nth(1)) {
        Ok(session) => session,
        Err(message) => {
            eprintln!("pantomim: {message}");
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shell::quit])
        .setup(move |app| shell::open_window(app.handle(), session))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn open(arg: Option<String>) -> Result<Session, String> {
    let arg = arg.ok_or("open pantomim from Moodle or with a .seb file")?;
    let data = match launch::download_url(&arg) {
        Some(url) => match fetch(&url)? {
            Some(data) => data,
            None => {
                return Ok(Session {
                    settings: Dictionary::new(),
                    start_url: url,
                    from_browser: true,
                });
            }
        },
        None => std::fs::read(&arg).map_err(|e| format!("{arg}: {e}"))?,
    };
    session_from(&data, &arg)
}

/// `None` means the link is a web page (login, HTML) to open in the browser instead.
fn fetch(url: &str) -> Result<Option<Vec<u8>>, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .build()
        .into();
    let mut response = agent
        .get(url)
        .header("User-Agent", format!("SEB/{}", session::SEB_VERSION))
        .call()
        .map_err(|e| format!("{url}: {e}"))?;
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok());
    if launch::is_browser_resource(response.status().as_u16(), content_type) {
        return Ok(None);
    }
    let data = response.body_mut().read_to_vec();
    data.map(Some).map_err(|e| format!("{url}: {e}"))
}

fn session_from(data: &[u8], link: &str) -> Result<Session, String> {
    let settings = configuration::load(data, None).map_err(|e| e.to_string())?;
    let start_url = launch::start_url(&settings, link).ok_or("configuration has no startURL")?;
    Ok(Session {
        settings,
        start_url,
        from_browser: false,
    })
}
