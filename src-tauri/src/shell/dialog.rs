//! Dialog windows (src/dialogs), modal on the exam window like SEB's `ShowDialog`, or on their own at startup.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use tauri::{
    AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window, WindowEvent,
};

type Answer = Box<dyn FnOnce(&AppHandle, Option<String>) + Send>;

/// Open dialogs by label: the window they belong to and what to do with the answer.
#[derive(Default)]
pub struct Dialogs(Mutex<HashMap<String, (Option<String>, Answer)>>);

/// Shows `index.html#{page}` over `parent`. `answer` gets the dialog's value, or `None` if it was closed.
pub fn open(
    app: &AppHandle,
    parent: Option<&Window>,
    page: &str,
    answer: impl FnOnce(&AppHandle, Option<String>) + Send + 'static,
) {
    static DIALOGS: AtomicUsize = AtomicUsize::new(0);
    let parent_label = parent.map(|p| p.label().to_string());
    // Modal like SEB: Wayland does not block the parent, so a second quit click must not stack dialogs.
    let dialogs = app.state::<Dialogs>();
    if dialogs
        .0
        .lock()
        .unwrap()
        .values()
        .any(|(p, _)| *p == parent_label)
    {
        return;
    }
    let label = format!("dialog-{}", DIALOGS.fetch_add(1, Ordering::Relaxed));
    let page = WebviewUrl::App(format!("index.html#{page}").into());
    // Hidden until the page has set its title and size (src/dialogs/dialog.ts).
    let mut builder = WebviewWindowBuilder::new(app, &label, page)
        .title("pantomim")
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .visible(false);
    if let Some(gtk_parent) = parent.and_then(|p| p.gtk_window().ok()) {
        builder = builder.transient_for_raw(&gtk_parent);
    }
    let dialog = match builder.build() {
        Ok(dialog) => dialog,
        Err(e) => return eprintln!("pantomim: dialog: {e}"),
    };
    if let Ok(window) = dialog.gtk_window() {
        use gtk::prelude::{GtkWindowExt, WidgetExt};
        window.set_modal(true);
        // A fixed-size GTK window takes the size its content asks for (see `fit`), unless a default size is bigger.
        window.set_size_request(-1, -1);
        window.set_default_size(-1, -1);
    }
    let pending = (parent_label, Box::new(answer) as Answer);
    dialogs.0.lock().unwrap().insert(label.clone(), pending);
    let handle = app.clone();
    dialog.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            finish(&handle, &label, None);
        }
    });
}

/// Sizes the dialog to its page (CSS pixels), the way `pin_height` sizes the taskbar.
#[tauri::command]
pub fn fit(dialog: WebviewWindow, width: i32, height: i32) -> tauri::Result<()> {
    dialog.with_webview(move |webview| {
        use gtk::prelude::WidgetExt;
        webview.inner().set_size_request(width, height);
    })
}

#[tauri::command]
pub fn answer(dialog: WebviewWindow, value: Option<String>) {
    finish(dialog.app_handle(), dialog.label(), value);
    let _ = dialog.destroy();
}

fn finish(app: &AppHandle, label: &str, value: Option<String>) {
    let pending = app.state::<Dialogs>().0.lock().unwrap().remove(label);
    if let Some((_, answer)) = pending {
        answer(app, value);
    }
}
