#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("مرحبا {}! صفر أثر يعمل بنجاح", name)
  }
