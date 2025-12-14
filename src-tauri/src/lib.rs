use tauri::{Manager, Window, WebviewUrl, WebviewWindowBuilder};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use std::time::{SystemTime, UNIX_EPOCH};

// Helper to get current timestamp in milliseconds
fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

// Commands that will be called from the frontend
#[tauri::command]
fn create_window(app: tauri::AppHandle, url: String, title: Option<String>) -> Result<(), String> {
    let window_label = format!("window_{}", current_timestamp_ms());
    
    WebviewWindowBuilder::new(&app, &window_label, WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {:?}", e))?))
        .title(title.unwrap_or_else(|| "GeminiDesk".to_string()))
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .decorations(false)
        .transparent(true)
        .build()
        .map_err(|e| format!("Failed to create window: {:?}", e))?;
    
    Ok(())
}

#[tauri::command]
fn show_window(window: Window) -> Result<(), String> {
    window.show().map_err(|e| format!("Failed to show window: {:?}", e))?;
    let _ = window.set_focus(); // Ignore errors as this may not be supported on all platforms
    Ok(())
}

#[tauri::command]
fn hide_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| format!("Failed to hide window: {:?}", e))?;
    Ok(())
}

#[tauri::command]
fn minimize_window(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| format!("Failed to minimize window: {:?}", e))?;
    Ok(())
}

#[tauri::command]
fn maximize_window(window: Window) -> Result<(), String> {
    window.maximize().map_err(|e| format!("Failed to maximize window: {:?}", e))?;
    Ok(())
}

#[tauri::command]
fn close_window(window: Window) -> Result<(), String> {
    window.close().map_err(|e| format!("Failed to close window: {:?}", e))?;
    Ok(())
}

#[tauri::command]
fn set_always_on_top(window: Window, always_on_top: bool) -> Result<(), String> {
    window.set_always_on_top(always_on_top).map_err(|e| format!("Failed to set always on top: {:?}", e))?;
    Ok(())
}

#[tauri::command]
async fn get_settings(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    use tauri_plugin_store::StoreExt;
    
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {:?}", e))?;
    
    // Return all settings as JSON
    let settings: std::collections::HashMap<String, serde_json::Value> = store.entries()
        .into_iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    serde_json::to_value(settings).map_err(|e| format!("Failed to serialize settings: {:?}", e))
}

#[tauri::command]
async fn save_setting(app: tauri::AppHandle, key: String, value: serde_json::Value) -> Result<(), String> {
    use tauri_plugin_store::StoreExt;
    
    let store = app.store("settings.json").map_err(|e| format!("Failed to access store: {:?}", e))?;
    
    // Set the value (store.set doesn't return an error)
    store.set(key.clone(), value.clone());
    
    // Save to disk
    store.save().map_err(|e| format!("Failed to save settings for key '{}': {:?}", key, e))?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Setup logging
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            
            // Create system tray
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let new_window_item = MenuItem::with_id(app, "new_window", "New Window", true, None::<&str>)?;
            
            let menu = Menu::with_items(app, &[&show_item, &new_window_item, &quit_item])?;
            
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "new_window" => {
                            let _ = WebviewWindowBuilder::new(
                                app,
                                format!("window_{}", current_timestamp_ms()),
                                WebviewUrl::External("https://gemini.google.com".parse().unwrap())
                            )
                            .title("GeminiDesk")
                            .inner_size(1200.0, 800.0)
                            .build();
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            create_window,
            show_window,
            hide_window,
            minimize_window,
            maximize_window,
            close_window,
            set_always_on_top,
            get_settings,
            save_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
