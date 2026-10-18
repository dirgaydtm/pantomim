//! The exam window: the exam page with the SEB taskbar below it.

pub mod dialog;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use plist::{Dictionary, Value};
use tauri::webview::{DownloadEvent, PageLoadEvent};
use tauri::window::WindowBuilder;
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, State, Webview, WebviewBuilder, WebviewUrl,
    Window,
};
use tauri::{Emitter, EventTarget};

use crate::{Session, Start, configuration, launch, session};

const TASKBAR_HEIGHT: i32 = 40;
// BrowserWindow.xaml toolbar: 30 px buttons with a 5 px margin, the 1 px progress row and border.
const TOOLBAR_HEIGHT: i32 = 42;

// Dialog pages (src/dialogs); `text` is a text key, whose title is the same key + "Title".
const CONFIRM_QUIT: &str = "screen=message&text=MessageBox_Quit&icon=question&buttons=yesno";
const CONFIRM_QUIT_URL: &str =
    "screen=message&text=MessageBox_BrowserQuitUrlConfirmation&icon=question&buttons=yesno";
const QUIT_PASSWORD: &str = "screen=password&text=PasswordDialog_QuitPasswordRequired";
const INVALID_QUIT_PASSWORD: &str =
    "screen=message&text=MessageBox_InvalidQuitPassword&icon=warning&buttons=ok";
const SETTINGS_PASSWORD: &str = "screen=password&text=PasswordDialog_SettingsPasswordRequired";
const NO_CONFIGURATION: &str = "screen=message&text=NoConfiguration&icon=information&buttons=ok";
const CONFIRM_HOME: &str =
    "screen=message&text=MessageBox_BrowserHomeQuestion&icon=question&buttons=yesno";
const HOME_PASSWORD: &str = "screen=password&text=PasswordDialog_BrowserHomePasswordRequired";
const INVALID_HOME_PASSWORD: &str =
    "screen=message&text=MessageBox_InvalidHomePassword&icon=warning&buttons=ok";
const CONFIRM_RELOAD: &str =
    "screen=message&text=MessageBox_PageReloadConfirmation&icon=question&buttons=yesno";

/// Opens what the configuration leads to, replacing `old` (a reconfigured exam window) if it opens.
/// Without `old` this is startup, where a failure or a cancelled password ends pantomim like SEB.
pub fn start(app: &AppHandle, start: Start, old: Option<Window>) {
    match start {
        Start::Exam(session) => match open_window(app, session) {
            Ok(()) => {
                if let Some(old) = old {
                    let _ = old.destroy();
                }
            }
            Err(e) => self::start(app, Start::failed(String::new(), e.to_string()), old),
        },
        // SEB ConfigurationBaseOperation asks at most 5 times; pantomim asks until right or cancelled.
        Start::Locked { data, link } => dialog::open(
            app,
            old.clone().as_ref(),
            SETTINGS_PASSWORD,
            move |app, password| match password {
                Some(password) => {
                    self::start(app, crate::start_from(data, link, Some(&password)), old)
                }
                None if old.is_none() => app.exit(0),
                None => {}
            },
        ),
        Start::Failed { error, uri, detail } => {
            eprintln!("pantomim: {detail}");
            let text = launch::failure_message(error.as_ref());
            let mut page = tauri::Url::parse("app:?screen=message&icon=error&buttons=ok").unwrap();
            page.query_pairs_mut()
                .append_pair("text", text)
                .append_pair("uri", &uri);
            dialog::open(
                app,
                old.clone().as_ref(),
                page.query().unwrap(),
                move |app, _| {
                    if old.is_none() {
                        app.exit(1);
                    }
                },
            );
        }
        Start::NoArgument => dialog::open(app, None, NO_CONFIGURATION, |app, _| app.exit(1)),
    }
}

/// Settings and start URL of each exam window, for the taskbar and toolbar commands.
#[derive(Default)]
pub struct Exams(Mutex<HashMap<String, (Dictionary, String)>>);

fn quit_password(settings: &Dictionary) -> Option<String> {
    let hash = settings
        .get("hashedQuitPassword")
        .and_then(Value::as_string)?;
    (!hash.is_empty()).then(|| hash.to_string())
}

/// The taskbar quit button (SEB `ShellResponsibility.TryInitiateShutdown`).
#[tauri::command]
pub fn quit(window: Window, exams: State<Exams>) {
    let app = window.app_handle().clone();
    let settings = exams
        .0
        .lock()
        .unwrap()
        .get(window.label())
        .map(|(s, _)| s.clone());
    match settings.as_ref().and_then(quit_password) {
        Some(hash) => dialog::open(
            &app,
            Some(&window.clone()),
            QUIT_PASSWORD,
            move |app, password| match password {
                Some(p) if session::is_quit_password(&hash, &p) => app.exit(0),
                Some(_) => dialog::open(app, Some(&window), INVALID_QUIT_PASSWORD, |_, _| {}),
                None => {}
            },
        ),
        None => dialog::open(&app, Some(&window), CONFIRM_QUIT, exit_on_yes),
    }
}

fn exit_on_yes(app: &AppHandle, answer: Option<String>) {
    if answer.as_deref() == Some("yes") {
        app.exit(0);
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

    let exams = app.state::<Exams>();
    let exam = (session.settings.clone(), session.start_url.clone());
    exams.0.lock().unwrap().insert(label.clone(), exam);

    let window = WindowBuilder::new(app, &label)
        .title("")
        .inner_size(1100.0, 800.0)
        .fullscreen(session::fullscreen(&session.settings))
        .build()?;
    // Linux lays child webviews out itself (see pin_height), so these bounds are placeholders.
    let (origin, size) = (LogicalPosition::new(0, 0), LogicalSize::new(1, 1));
    let bar = session::toolbar(&session.settings);
    let mut page = tauri::Url::parse("app:?screen=toolbar").unwrap();
    page.query_pairs_mut()
        .append_pair("home", &bar.home.to_string())
        .append_pair("navigation", &bar.navigation.to_string())
        .append_pair("reload", &bar.reload.to_string())
        .append_pair("reloadEnabled", &bar.reload_enabled.to_string())
        .append_pair("addressBar", &bar.address_bar.to_string())
        .append_pair("address", &session.start_url);
    let page = format!("index.html#{}", page.query().unwrap());
    let toolbar = WebviewBuilder::new(format!("{label}-toolbar"), WebviewUrl::App(page.into()));
    pin_height(&window.add_child(toolbar, origin, size)?, TOOLBAR_HEIGHT)?;
    let reload = session::reload(&session.settings);
    let exam = window.add_child(exam_page(app, &label, session)?, origin, size)?;
    reload_on_f5(&exam, reload)?;
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
    let (address_settings, title_settings) = (settings.clone(), settings.clone());
    let quit_app = app.clone();
    let window = label.to_string();
    let download_app = app.clone();

    Ok(WebviewBuilder::new(
        format!("{label}-exam"),
        WebviewUrl::External(start_url.parse()?),
    )
    .user_agent(&session::user_agent(&settings))
    .initialization_script_for_all_frames(session::js_api(&config_key))
    .on_page_load(move |webview, load| {
        let address = load.url().as_str();
        // Sent again when the page has loaded: the toolbar may not be listening yet at the start.
        let toolbar = EventTarget::webview(format!("{}-toolbar", webview.window().label()));
        let _ = webview.emit_to(toolbar, "address", address);
        if load.event() == PageLoadEvent::Started
            && let Some(title) = session::window_title(&address_settings, Some(address), None)
        {
            let _ = webview.window().set_title(title);
        }
    })
    .on_document_title_changed(move |webview, page_title| {
        if let Some(title) = session::window_title(&title_settings, None, Some(&page_title)) {
            let _ = webview.window().set_title(title);
        }
    })
    .on_navigation(move |url| {
        // SEB cancels the navigation, then asks only if quitURLConfirm is set (RequestResponsibility).
        let quit = session::is_quit_url(&settings, url.as_str());
        if quit {
            let confirm = settings.get("quitURLConfirm").and_then(Value::as_boolean);
            let (app, window) = (quit_app.clone(), window.clone());
            // Deferred so the dialog is not opened inside the webview's navigation callback.
            let _ = quit_app.run_on_main_thread(move || match app.get_window(&window) {
                Some(window) if confirm == Some(true) => {
                    dialog::open(&app, Some(&window), CONFIRM_QUIT_URL, exit_on_yes)
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

/// F5 on the exam page, as SEB's `KeyboardHandler` handles it: on key release, through
/// `session::reload`. WebKitGTK has no reload shortcut of its own.
fn reload_on_f5(exam: &Webview, reload: session::Reload) -> tauri::Result<()> {
    let page = exam.clone();
    exam.with_webview(move |webview| {
        use gtk::prelude::*;
        webview.inner().connect_key_release_event(move |_, key| {
            if key.keyval() != gtk::gdk::keys::constants::F5 {
                return gtk::glib::Propagation::Proceed;
            }
            reload_page(&page, &reload);
            gtk::glib::Propagation::Stop
        });
    })
}

/// The toolbar buttons and address bar (SEB `BrowserWindow` events, handled by the
/// `WindowResponsibility` and `DisplayResponsibility` methods named below).
#[tauri::command]
pub fn browse(window: Window, exams: State<Exams>, action: String, address: Option<String>) {
    let app = window.app_handle().clone();
    let exam = app.get_webview(&format!("{}-exam", window.label()));
    let Some((settings, start_url)) = exams.0.lock().unwrap().get(window.label()).cloned() else {
        return;
    };
    let (Some(exam), bar) = (exam, session::toolbar(&settings)) else {
        return;
    };
    match action.as_str() {
        "back" if bar.navigation => {
            let _ = exam.eval("history.back()");
        }
        "forward" if bar.navigation => {
            let _ = exam.eval("history.forward()");
        }
        "reload" if bar.reload_enabled => reload_page(&exam, &session::reload(&settings)),
        "home" if bar.home => go_home(&window, exam, &settings, start_url),
        // Window_AddressChanged: an invalid address only clears the bar.
        "address" if bar.address_bar => match address.as_deref().and_then(session::typed_address) {
            Some(url) => {
                if let Ok(url) = url.parse() {
                    let _ = exam.navigate(url);
                }
            }
            None => {
                let toolbar = EventTarget::webview(format!("{}-toolbar", window.label()));
                let _ = app.emit_to(toolbar, "address", "");
            }
        },
        _ => {}
    }
}

/// `HomeNavigationRequested`: back to the home URL after the quit password or a confirmation;
/// `restartExamText` replaces the dialog title.
fn go_home(window: &Window, exam: Webview, settings: &Dictionary, start_url: String) {
    let text = |key| settings.get(key).and_then(Value::as_string).unwrap_or("");
    let flag = |key| {
        settings
            .get(key)
            .and_then(Value::as_boolean)
            .unwrap_or(false)
    };
    let url = if flag("restartExamUseStartURL") {
        start_url
    } else {
        text("restartExamURL").to_string()
    };
    let Ok(url) = url.parse::<tauri::Url>() else {
        return;
    };
    let mut title = tauri::Url::parse("app:").unwrap();
    if !text("restartExamText").trim().is_empty() {
        title
            .query_pairs_mut()
            .append_pair("title", text("restartExamText"));
    }
    let title = title.query().map(|q| format!("&{q}")).unwrap_or_default();
    let app = window.app_handle().clone();
    let password = quit_password(settings).filter(|_| flag("restartExamPasswordProtected"));
    match password {
        Some(hash) => {
            let parent = window.clone();
            dialog::open(
                &app,
                Some(window),
                &format!("{HOME_PASSWORD}{title}"),
                move |app, answer| match answer {
                    Some(p) if session::is_quit_password(&hash, &p) => {
                        let _ = exam.navigate(url);
                    }
                    Some(_) => dialog::open(app, Some(&parent), INVALID_HOME_PASSWORD, |_, _| {}),
                    None => {}
                },
            )
        }
        None => dialog::open(
            &app,
            Some(window),
            &format!("{CONFIRM_HOME}{title}"),
            move |_, answer| {
                if answer.as_deref() == Some("yes") {
                    let _ = exam.navigate(url);
                }
            },
        ),
    }
}

fn reload_page(page: &Webview, reload: &session::Reload) {
    match reload {
        session::Reload::Blocked => {}
        session::Reload::Now => {
            let _ = page.reload();
        }
        session::Reload::Ask => {
            let (app, window, page) = (page.app_handle().clone(), page.window(), page.clone());
            dialog::open(&app, Some(&window), CONFIRM_RELOAD, move |_, answer| {
                if answer.as_deref() == Some("yes") {
                    let _ = page.reload();
                }
            });
        }
    }
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
    let started = match data {
        Ok(data) => crate::start_from(data, link.to_string(), None),
        Err(e) => Start::failed(link.to_string(), format!("{link}: {e}")),
    };
    start(app, started, app.get_window(old_window));
}
