#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::sync::Mutex;

struct Backend(Mutex<Option<std::process::Child>>);

#[tauri::command]
fn backend_running() -> bool {
    true // placeholder if you want to expose IPC methods later
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // spawn python backend as sidecar on dev & prod
            // assumes user has python + deps installed; in prod you can bundle a venv or use a packaged binary
            let backend = Command::new("python")
                .arg("-m")
                .arg("backend.app")
                .current_dir(
                    app.path()
                        .resource_dir()
                        .unwrap_or(app.path().app_dir().unwrap()),
                )
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .ok();

            app.manage(Backend(Mutex::new(backend)));
            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                // optionally: kill backend when main window closes
            }
        })
        .invoke_handler(tauri::generate_handler![backend_running])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
