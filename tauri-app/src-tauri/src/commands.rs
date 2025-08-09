use tauri::ipc::Response;

// use crate::error_handler;

// rename_all=snake_case forces attribute names in JavaScript to match Rust,
// otherwise will convert attribute names from snake_case to camelCase.
// I chose to use snake_case option to align between frontend and backend
// to reduce confusion.

#[tauri::command(rename_all="snake_case")]
pub fn hello_world(invoke_message: String) -> String{
    println!("I was invoked from JavaScript, with this message: {}", invoke_message);
    return "Hello from Rust!".into();
}

// example returning Array Buffers

#[tauri::command(rename_all="snake_case")]
pub fn read_file() -> Response {
    // std::env::current_dir()
    let data = std::fs::read("../../data/test_file.txt").unwrap();
    return tauri::ipc::Response::new(data);
}

// example error handling
// #[tauri::command(rename_all="snake_case")]
// fn login(user: String, password: String) -> Result<String, String> {
//   if user == "tauri" && password == "tauri" {
//     // resolve
//     Ok("logged_in".to_string())
//   } else {
//     // reject
//     Err("invalid credentials".to_string())
//   }
// }

