//! The exam window: the exam page with the SEB taskbar below it.

pub mod dialog;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use plist::Value;
use tauri::webview::DownloadEvent;
use tauri::window::WindowBuilder;
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, State, Webview, WebviewBuilder, WebviewUrl,
    Window,
};

use crate::{Session, configuration, launch, session};

const TASKBAR_HEIGHT: i32 = 40;

const CONFIRM_QUIT: &str =
    "screen=message&title=MessageBox_QuitTitle&message=MessageBox_Quit&icon=question&buttons=yesno";
const CONFIRM_QUIT_URL: &str = "screen=message&title=MessageBox_BrowserQuitUrlConfirmationTitle&message=MessageBox_BrowserQuitUrlConfirmation&icon=question&buttons=yesno";
const QUIT_PASSWORD: &str = "screen=password&title=PasswordDialog_QuitPasswordRequiredTitle&message=PasswordDialog_QuitPasswordRequired";
const INVALID_QUIT_PASSWORD: &str = "screen=message&title=MessageBox_InvalidQuitPasswordTitle&message=MessageBox_InvalidQuitPassword&icon=warning&buttons=ok";

/// `hashedQuitPassword` of each exam window that has one.
#[derive(Default)]
pub struct QuitPasswords(Mutex<HashMap<String, String>>);

/// The taskbar quit button (SEB `ShellResponsibility.TryInitiateShutdown`).
#[tauri::command]
pub fn quit(window: Window, passwords: State<QuitPasswords>) {
    match passwords.0.lock().unwrap().get(window.label()).cloned() {
        Some(hash) => dialog::open(
            &window,
            QUIT_PASSWORD,
            move |window, password| match password {
                Some(p) if session::is_quit_password(&hash, &p) => window.app_handle().exit(0),
                Some(_) => dialog::open(window, INVALID_QUIT_PASSWORD, |_, _| {}),
                None => {}
            },
        ),
        None => dialog::open(&window, CONFIRM_QUIT, exit_on_yes),
    }
}

fn exit_on_yes(window: &Window, answer: Option<String>) {
    if answer.as_deref() == Some("yes") {
        window.app_handle().exit(0);
    }
}

pub fn open_window(app: &AppHandle, session: Session) -> Result<(), Box<dyn Error>> {
    static WINDOWS: AtomicUsize = AtomicUsize::new(0);
    let label = format!("exam-{}", WINDOWS.fetch_add(1, Ordering::Relaxed));
    let flag = |key| {
        session
            .settings
            .get(key)
            .and_then(Value::as_boolean)
            .unwrap_or(true)
    };
    let (show_taskbar, show_clock, show_quit) =
        (flag("showTaskBar"), flag("showTime"), flag("allowQuit"));

    let quit_password = session.settings.get("hashedQuitPassword");
    if let Some(hash) = quit_password
        .and_then(Value::as_string)
        .filter(|h| !h.is_empty())
    {
        let passwords = app.state::<QuitPasswords>();
        passwords
            .0
            .lock()
            .unwrap()
            .insert(label.clone(), hash.into());
    }

    let window = WindowBuilder::new(app, &label)
        .title("pantomim")
        .inner_size(1100.0, 800.0)
        .build()?;
    // Linux lays child webviews out itself (see pin_height), so these bounds are placeholders.
    let (origin, size) = (LogicalPosition::new(0, 0), LogicalSize::new(1, 1));
    window.add_child(exam_page(app, &label, session)?, origin, size)?;
    if show_taskbar {
        let page = format!("index.html#screen=taskbar&clock={show_clock}&quit={show_quit}");
        let taskbar = WebviewBuilder::new(format!("{label}-taskbar"), WebviewUrl::App(page.into()));
        pin_height(&window.add_child(taskbar, origin, size)?, TASKBAR_HEIGHT)?;
    }
    Ok(())
}

/// The exam page as SEB shows it to the server: user agent, JS API, quit URL and configuration downloads.
fn exam_page(
    app: &AppHandle,
    label: &str,
    session: Session,
) -> Result<WebviewBuilder<tauri::Wry>, Box<dyn Error>> {
    let Session {
        settings,
        start_url,
        from_browser,
    } = session;
    let config_key = configuration::config_key(&settings);
    let quit_app = app.clone();
    let window = label.to_string();
    let download_app = app.clone();

    Ok(WebviewBuilder::new(
        format!("{label}-exam"),
        WebviewUrl::External(start_url.parse()?),
    )
    .user_agent(&session::user_agent(&settings))
    .initialization_script_for_all_frames(session::js_api(&config_key))
    .on_navigation(move |url| {
        // SEB cancels the navigation, then asks only if quitURLConfirm is set (RequestResponsibility).
        let quit = session::is_quit_url(&settings, url.as_str());
        if quit {
            let confirm = settings.get("quitURLConfirm").and_then(Value::as_boolean);
            let (app, window) = (quit_app.clone(), window.clone());
            // Deferred so the dialog is not opened inside the webview's navigation callback.
            let _ = quit_app.run_on_main_thread(move || match app.get_window(&window) {
                Some(window) if confirm == Some(true) => {
                    dialog::open(&window, CONFIRM_QUIT_URL, exit_on_yes)
                }
                _ => app.exit(0),
            });
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
            let old = webview.window().label().to_string();
            let _ = download_app
                .run_on_main_thread(move || reconfigure(&app, &old, path, url.as_str()));
            true
        }
        _ => true,
    }))
}

/// Tauri packs child webviews into the window's vertical GtkBox, sharing the height equally.
/// A pinned webview keeps its own height; the exam page takes the rest.
fn pin_height(webview: &Webview, height: i32) -> tauri::Result<()> {
    webview.with_webview(move |webview| {
        use gtk::prelude::*;
        let webview = webview.inner();
        webview.set_size_request(-1, height);
        if let Some(vbox) = webview.parent().and_then(|p| p.downcast::<gtk::Box>().ok()) {
            vbox.set_child_packing(&webview, false, true, 0, gtk::PackType::Start);
        }
    })
}

/// A `.seb` downloaded in a browser-resource session replaces it; cookies stay, so the login is kept.
fn reconfigure(app: &AppHandle, old_window: &str, path: PathBuf, link: &str) {
    let data = std::fs::read(&path);
    let _ = std::fs::remove_file(&path);
    let opened = data
        .map_err(|e| e.to_string())
        .and_then(|data| crate::session_from(&data, link))
        .and_then(|session| open_window(app, session).map_err(|e| e.to_string()));
    match opened {
        Ok(()) => {
            if let Some(window) = app.get_window(old_window) {
                let _ = window.destroy();
            }
        }
        Err(message) => eprintln!("pantomim: {message}"),
    }
}
