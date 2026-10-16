pub mod configuration;
pub mod session;

use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (settings, config_key) = match open_configuration() {
        Ok(opened) => opened,
        Err(message) => {
            eprintln!("pantomim: {message}");
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let start_url = settings
                .get("startURL")
                .and_then(|v| v.as_string())
                .ok_or("configuration has no startURL")?
                .parse()?;
            let handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(start_url))
                .title("pantomim")
                .user_agent(&session::user_agent(&settings))
                .initialization_script_for_all_frames(session::js_api(&config_key))
                .on_navigation(move |url| {
                    let quit = session::is_quit_url(&settings, url.as_str());
                    if quit {
                        handle.exit(0);
                    }
                    !quit
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn open_configuration() -> Result<(plist::Dictionary, String), String> {
    let path = std::env::args()
        .nth(1)
        .ok_or("open pantomim from Moodle or with a .seb file")?;
    let data = std::fs::read(&path).map_err(|e| format!("{path}: {e}"))?;
    let settings = configuration::load(&data, None).map_err(|e| e.to_string())?;
    let config_key = configuration::config_key(&settings);
    Ok((settings, config_key))
}
