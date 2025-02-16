// use tauri::async_runtime::Mutex;
use std::sync::{Mutex};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(Mutex::new(User{
            id: 0,
            username: "".to_string(),
            password: "".to_string(),
        }))
    .invoke_handler(tauri::generate_handler![get_user, login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_user(state: tauri::State<Mutex<User>>) -> User {
    let user =&*state.lock().unwrap();
    user.clone()
}

#[tauri::command]
fn login(state: tauri::State<Mutex<User>>, username: String, password: String) -> bool {
    *state.lock().unwrap() = User{
        username, 
        password, 
        id: 1
    };
    true
}

#[derive(serde::Serialize, Clone)]
struct User {
    id: u32,
    username: String,
    password: String
}
