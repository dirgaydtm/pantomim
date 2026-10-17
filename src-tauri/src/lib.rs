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

/// Where the configuration argument leads.
enum Start {
    Exam(Session),
    /// An encrypted `.seb`: ask for its password, then `start_from` again.
    Locked {
        data: Vec<u8>,
        link: String,
    },
    /// `error` is `None` when it is not the configuration's fault (download, file).
    Failed {
        error: Option<configuration::Error>,
        uri: String,
        detail: String,
    },
    NoArgument,
}

pub fn run() {
    let start = open(std::env::args().nth(1));

    tauri::Builder::default()
        .manage(shell::dialog::Dialogs::default())
        .manage(shell::QuitPasswords::default())
        .invoke_handler(tauri::generate_handler![
            shell::quit,
            shell::dialog::answer,
            shell::dialog::fit
        ])
        .setup(move |app| {
            shell::start(app.handle(), start, None);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn open(arg: Option<String>) -> Start {
    let Some(arg) = arg else {
        return Start::NoArgument;
    };
    let failed = |uri: String, detail: String| Start::Failed {
        error: None,
        uri,
        detail,
    };
    let (data, link) = match launch::download_url(&arg) {
        Some(url) => match fetch(&url) {
            Ok(Some(data)) => (data, arg),
            Ok(None) => {
                return Start::Exam(Session {
                    settings: Dictionary::new(),
                    start_url: url,
                    from_browser: true,
                });
            }
            Err(detail) => return failed(arg, detail),
        },
        None => {
            // SEB names a file argument by its file URI.
            let path = std::path::absolute(&arg).unwrap_or_else(|_| arg.clone().into());
            let uri = format!("file://{}", path.display());
            match std::fs::read(&path) {
                Ok(data) => (data, uri),
                Err(e) => return failed(uri, format!("{arg}: {e}")),
            }
        }
    };
    start_from(data, link, None)
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

fn start_from(data: Vec<u8>, link: String, password: Option<&str>) -> Start {
    match configuration::load(&data, password) {
        Ok(settings) => match launch::start_url(&settings, &link) {
            Some(start_url) => Start::Exam(Session {
                settings,
                start_url,
                from_browser: false,
            }),
            None => Start::Failed {
                error: None,
                detail: format!("{link}: configuration has no startURL"),
                uri: link,
            },
        },
        Err(configuration::Error::PasswordNeeded | configuration::Error::WrongPassword) => {
            Start::Locked { data, link }
        }
        Err(error) => Start::Failed {
            detail: format!("{link}: {error}"),
            error: Some(error),
            uri: link,
        },
    }
}
