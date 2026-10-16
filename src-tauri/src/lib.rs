pub mod configuration;
pub mod launch;
pub mod session;

use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use plist::Dictionary;
use tauri::webview::DownloadEvent;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| open_window(app.handle(), session))
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

fn open_window(app: &AppHandle, session: Session) -> Result<(), Box<dyn Error>> {
    static WINDOWS: AtomicUsize = AtomicUsize::new(0);
    let label = format!("exam-{}", WINDOWS.fetch_add(1, Ordering::Relaxed));
    let Session {
        settings,
        start_url,
        from_browser,
    } = session;
    let config_key = configuration::config_key(&settings);
    let quit_app = app.clone();
    let download_app = app.clone();

    WebviewWindowBuilder::new(app, &label, WebviewUrl::External(start_url.parse()?))
        .title("pantomim")
        .user_agent(&session::user_agent(&settings))
        .initialization_script_for_all_frames(session::js_api(&config_key))
        .on_navigation(move |url| {
            let quit = session::is_quit_url(&settings, url.as_str());
            if quit {
                quit_app.exit(0);
            }
            !quit
        })
        .on_download(move |webview, event| match event {
            DownloadEvent::Requested { url, destination } => {
                let name = destination
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                let allowed = from_browser && launch::is_configuration_download(url.as_str(), name);
                if allowed {
                    let file = format!("pantomim-{}.seb", std::process::id());
                    *destination = std::env::temp_dir().join(file);
                }
                allowed
            }
            DownloadEvent::Finished {
                url,
                path: Some(path),
                success: true,
            } => {
                let app = download_app.clone();
                let old = webview.label().to_string();
                let _ = download_app
                    .run_on_main_thread(move || reconfigure(&app, &old, path, url.as_str()));
                true
            }
            _ => true,
        })
        .build()?;
    Ok(())
}

/// A `.seb` downloaded in a browser-resource session replaces it; cookies stay, so the login is kept.
fn reconfigure(app: &AppHandle, old_window: &str, path: PathBuf, link: &str) {
    let data = std::fs::read(&path);
    let _ = std::fs::remove_file(&path);
    let opened = data
        .map_err(|e| e.to_string())
        .and_then(|data| session_from(&data, link))
        .and_then(|session| open_window(app, session).map_err(|e| e.to_string()));
    match opened {
        Ok(()) => {
            if let Some(window) = app.get_webview_window(old_window) {
                let _ = window.destroy();
            }
        }
        Err(message) => eprintln!("pantomim: {message}"),
    }
}
