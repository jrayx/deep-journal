use std::io::Write;
use std::process::{Command, Stdio};
// use tauri::ipc::Response;

// use crate::error_handler;

// rename_all=snake_case forces attribute names in JavaScript to match Rust,
// otherwise will convert attribute names from snake_case to camelCase.
// I chose to use snake_case option to align between frontend and backend
// to reduce confusion.

#[tauri::command(rename_all="snake_case")]
pub fn hello_world(invoke_message: String) -> String {
    println!("I was invoked from JavaScript, with this message: {}", invoke_message);
    return "Hello from Rust!".into();
}

#[tauri::command(rename_all="snake_case")]
pub fn read_file() -> String {
    let data = std::fs::read("../../data/test_file.txt");
    match data {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(e) => format!("Failed to read file: {}", e),
    }
}

// example returning Array Buffers
// #[tauri::command(rename_all="snake_case")]
// pub fn read_file() -> Response {
//     // std::env::current_dir()
//     let data = std::fs::read("../../data/test_file.txt").unwrap();
//     return tauri::ipc::Response::new(data);
// }

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

#[tauri::command(rename_all="snake_case")]
pub fn run_llm(model_name: String, message: String) -> String {
    let input: &str = &message;

    let mut child = match Command::new("ollama")
        .arg("run")
        .arg(model_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Ok(child) => child,
            Err(e) => return format!("Failed to start process: {}", e),
        };

    {
        let stdin = child.stdin.as_mut();
        if let Some(stdin) = stdin {
            if let Err(e) = stdin.write_all(input.as_bytes()) {
                return format!("Failed to write to stdin: {}", e);
            }
        } else {
            return "Failed to open stdin".to_string();
        }
    }

    let output = match child.wait_with_output() {
        Ok(output) => output,
        Err(e) => return format!("Failed to read output: {}", e),
    };

    match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => format!("Invalid UTF-8 output: {}", e),
    }
}