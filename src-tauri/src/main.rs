#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use tauri::{Manager, WindowEvent};

struct Backend(Mutex<Option<Child>>);

#[tauri::command]
fn backend_running() -> bool {
    true
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // spawn: `python -m backend.app` (adjust path if needed)
            let child = Command::new("python")
                .args(["-m", "backend.app"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .ok();

            // store child in app state
            app.manage(Backend(Mutex::new(child)));
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                // grab child from state and kill it
                let handle = window.app_handle();
                let state = handle.state::<Backend>();
                if let Some(mut child) = state.0.lock().unwrap().take() {
                    let _ = child.kill();
                }
                // if you also want to quit here explicitly:
                // handle.exit(0);
            }
        })
        .invoke_handler(tauri::generate_handler![backend_running])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
