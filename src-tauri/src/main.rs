#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowUrl};

const GEMINI_URL: &str = "https://gemini.google.com/app";

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                let _ = window.eval(&format!("window.location.replace('{}');", GEMINI_URL));
            } else {
                tauri::WindowBuilder::new(
                    app,
                    "main",
                    WindowUrl::External(GEMINI_URL.parse().expect("valid URL")),
                )
                .title("GeminiDesk")
                .inner_size(1400.0, 900.0)
                .resizable(true)
                .build()?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
