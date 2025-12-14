#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, WindowBuilder, WindowUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Settings {
    show_close_button: bool,
    show_export_button: bool,
    show_fullscreen_button: bool,
    show_new_window_button: bool,
    show_minimize_button: bool,
    show_chat_title: bool,
    always_on_top: bool,
    theme: String, // "light", "dark", "system"
    language: String,
    app_mode: String, // "gemini" or "aistudio"
    zoom_level: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_close_button: true,
            show_export_button: true,
            show_fullscreen_button: true,
            show_new_window_button: true,
            show_minimize_button: true,
            show_chat_title: true,
            always_on_top: false,
            theme: "system".to_string(),
            language: "en".to_string(),
            app_mode: "gemini".to_string(),
            zoom_level: 1.0,
        }
    }
}

struct AppState {
    settings: Mutex<Settings>,
    settings_path: Mutex<PathBuf>,
}

fn get_settings_path(app_handle: &AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_config_dir()
        .map(|dir| dir.join("settings.json"))
        .unwrap_or_else(|| PathBuf::from("settings.json"))
}

fn load_settings(path: &PathBuf) -> Settings {
    if path.exists() {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str(&content) {
                return settings;
            }
        }
    }
    Settings::default()
}

fn save_settings(path: &PathBuf, settings: &Settings) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, serde_json::to_string_pretty(settings).unwrap());
}

#[tauri::command]
fn get_settings(state: tauri::State<AppState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_setting(
    state: tauri::State<AppState>,
    app_handle: AppHandle,
    key: String,
    value: serde_json::Value,
) {
    let mut settings = state.settings.lock().unwrap();
    let settings_path = state.settings_path.lock().unwrap();
    let mut save = true;

    match key.as_str() {
        "alwaysOnTop" => {
            if let Some(v) = value.as_bool() {
                settings.always_on_top = v;
                if let Some(window) = app_handle.get_window("main") {
                    let _ = window.set_always_on_top(v);
                }
            }
        }
        "appMode" => {
             if let Some(v) = value.as_str() {
                settings.app_mode = v.to_string();
             }
        }
        "theme" => {
            if let Some(v) = value.as_str() {
                settings.theme = v.to_string();
                app_handle.emit_all("theme-updated", v).unwrap();
            }
        }
        "language" => {
            if let Some(v) = value.as_str() {
                settings.language = v.to_string();
                app_handle.emit_all("language-changed", v).unwrap();
            }
        }
        // Handle other keys generically if they match struct fields (requires more complex serde logic or manual mapping)
        // For this demo, we'll assume the frontend sends individual updates but we save the whole struct.
        // To properly update arbitrary fields without a huge match block, we'd typically update the struct from a JSON merge.
        // But here we'll just allow saving whatever state we have.
        _ => {
             // For now, we only persist specific typed fields in the struct.
             // If we want to support dynamic settings updates from JS, we should implement a merge strategy.
             // However, since we are mimicking specific logic, let's just log.
             println!("Update setting: {} = {:?}", key, value);
        }
    }

    if save {
        save_settings(&settings_path, &settings);
        app_handle.emit_all("settings-updated", settings.clone()).unwrap();
    }
}

#[tauri::command]
fn minimize_window(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn toggle_maximize(window: tauri::Window) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    let _ = window.close();
}

#[tauri::command]
fn open_external(url: String) {
    let _ = open::that(url);
}

#[tauri::command]
fn get_app_mode(state: tauri::State<AppState>) -> String {
    state.settings.lock().unwrap().app_mode.clone()
}

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let window = app.get_window("main").unwrap();
                match id.as_str() {
                    "show" => {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    "hide" => {
                        window.hide().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .manage(AppState {
            settings: Mutex::new(Settings::default()),
            settings_path: Mutex::new(PathBuf::new()),
        })
        .setup(|app| {
            let app_handle = app.handle();
            let path = get_settings_path(&app_handle);
            let settings = load_settings(&path);

            // Set initial state
            {
                let state = app.state::<AppState>();
                let mut s = state.settings.lock().unwrap();
                *s = settings.clone();
                let mut p = state.settings_path.lock().unwrap();
                *p = path;
            }

            let window = app.get_window("main").unwrap();

            // Apply initial settings
            if settings.always_on_top {
                window.set_always_on_top(true)?;
            }

            // Init Scripts
            let init_script = include_str!("init.js");
            let inject_script = include_str!("inject.js");
            window.eval(init_script)?;
            window.eval(inject_script)?;

            // Register shortcuts
            let mut shortcut_manager = app.global_shortcut_manager();

            // Alt+G: Toggle visibility
            let w = window.clone();
            shortcut_manager.register("Alt+G", move || {
                if w.is_visible().unwrap_or(true) {
                    let _ = w.hide();
                } else {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            })?;

            // Alt+Q: Close (hide)
            let w = window.clone();
            shortcut_manager.register("Alt+Q", move || {
               let _ = w.close();
            })?;

            #[cfg(debug_assertions)]
            {
                // window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_setting,
            minimize_window,
            toggle_maximize,
            close_window,
            open_external,
            get_app_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
