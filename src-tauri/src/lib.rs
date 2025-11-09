// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{fs, path::PathBuf, thread, time::Duration};
use tauri::{command, AppHandle, Emitter, Manager};

#[command]
fn start_long_task(app: AppHandle) {
    thread::spawn(move || {
        for i in 0..=100 {
            // 100ミリ秒ごとに1ずつ進捗
            thread::sleep(Duration::from_millis(100));

            // フロントにイベント送信
            app.emit("progress", i).unwrap();
        }

        // 完了通知
        app.emit("progress_done", true).unwrap();
    });
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![start_long_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
