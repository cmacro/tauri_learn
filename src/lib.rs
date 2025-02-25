use tauri_plugin_dialog::DialogExt;
use tauri::{AppHandle, Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![save_file, open_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(app: AppHandle, contents: String) -> String {
    println!("{:?}", contents);
    std::thread::spawn(move || {
        let file_path = app.dialog().file().blocking_save_file().unwrap();
        let file = file_path.to_string();
        std::fs::write(file, contents).unwrap();

        app.emit("save_state", "saved").unwrap();
    });
    "file open".to_string()
}

#[tauri::command]
fn open_file(app: AppHandle) {
    std::thread::spawn(move || {
        let file_path = app.dialog().file().blocking_pick_file().unwrap();
        let file = file_path.to_string();
        let contents = std::fs::read_to_string(file).unwrap();

        app.emit("contents", contents).unwrap()
    });
}
