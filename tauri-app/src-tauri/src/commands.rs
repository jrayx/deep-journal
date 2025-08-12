use rusqlite::{params, Connection, Result};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::entities;

const DB_PATH: &str = "../resources/journal.db";


#[tauri::command(rename_all="snake_case")]
pub fn setup_database() -> Result<(), String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    // List of SQL files to run
    let sql_files = [
        "../resources/queries/1_create_db.sql",
        "../resources/queries/2_fill_data.sql",
    ];

    for file in &sql_files {
        let sql = fs::read_to_string(file)
            .map_err(|e| format!("Failed to read {}: {}", file, e))?;
        conn.execute_batch(&sql)
            .map_err(|e| format!("Failed to execute {}: {}", file, e))?;
    }

    Ok(())
}

#[tauri::command(rename_all="snake_case")]
pub fn run_llm(model_name: String, chat_id: i32) -> String {
    
    // Get chat history
    let chat_history_result = get_messages_by_chat(chat_id);
    let chat_history = match chat_history_result {
        Ok(messages) => {
            messages
        },
        Err(e) => return format!("Failed to get chat history: {}", e),
    };
    if chat_history.is_empty() {
        // handle empty case
        return "No chat history found".to_string();
    } else {
        // Find the most recent message (greatest id)
        let mut messages = chat_history.clone();
        messages.sort_by_key(|m| m.id); // ascending order
        let most_recent_message = messages.last().unwrap(); // greatest id
        let most_recent_message_text = &most_recent_message.text;
        let previous_messages: Vec<entities::Message> = messages[..messages.len()-1].to_vec();
        // Combine chat history and current message
        let system_instruction = "You are a concise assistant. Do not reveal your internal reasoning or thinking process. Only give the final answer unless explicitly asked to explain.";
        // Format messages as a single string
        let previous_messages_text = previous_messages.iter()
                .map(|m| format!("{}: {}", m.sender, m.text))
                .collect::<Vec<_>>()
                .join("\n");
        let full_input = format!(
            "System:{}\n{}\nUser: {}",
            system_instruction,
            previous_messages_text,
            most_recent_message_text
        );

        // init Ollama child process
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
                if let Err(e) = stdin.write_all(full_input.as_bytes()) {
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
    
    
}


// Models

#[tauri::command(rename_all = "snake_case")]
pub fn get_models() -> Result<Vec<entities::Model>, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    let mut stmt = conn.prepare("SELECT id, name FROM models")
        .map_err(|e| format!("Prepare error: {}", e))?;

    let model_iter = stmt.query_map([], |row| {
        Ok(entities::Model {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).map_err(|e| format!("Query error: {}", e))?;

    let mut models = Vec::new();
    for model_result in model_iter {
        let model = model_result.map_err(|e| format!("Row error: {}", e))?;
        models.push(model);
    }

    return Ok(models);
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_model(new_model_name: String) -> Result<entities::Model, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("INSERT INTO models (name) VALUES (?)", params![new_model_name])
        .map_err(|e| format!("Insert error: {}", e))?;

    let last_id = conn.last_insert_rowid();
    
    Ok(entities::Model {
        id: last_id as i32,
        name: new_model_name,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_model(model_id: i32) -> Result<(), String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("DELETE FROM models WHERE id = ?", params![model_id])
        .map_err(|e| format!("Delete error: {}", e))?;

    Ok(())
}


// Chats

#[tauri::command(rename_all = "snake_case")]
pub fn get_chats() -> Result<Vec<entities::Chat>, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    let mut stmt = conn.prepare("SELECT id, title FROM chats ORDER BY id DESC")
        .map_err(|e| format!("Prepare error: {}", e))?;

    let model_iter = stmt.query_map([], |row| {
        Ok(entities::Chat {
            id: row.get(0)?,
            title: row.get(1)?,
        })
    }).map_err(|e| format!("Query error: {}", e))?;

    let mut chats = Vec::new();
    for chat_result in model_iter {
        let chat = chat_result.map_err(|e| format!("Row error: {}", e))?;
        chats.push(chat);
    }

    return Ok(chats);
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_chat() -> Result<entities::Chat, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    let mut max_id = conn.query_row("SELECT MAX(id) FROM chats", [], |row| {
        row.get(0)
    }).unwrap_or(0);
    max_id += 1;

    let title = format!("New Chat (#{})", max_id);
    conn.execute("INSERT INTO chats (title) VALUES (?)", params![title])
        .map_err(|e| format!("Insert error: {}", e))?;

    let last_id = conn.last_insert_rowid();

    Ok(entities::Chat {
        id: last_id as i32,
        title: title,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_chat_title(chat_id: i32, new_title: String) -> Result<(), String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("UPDATE chats SET title = ? WHERE id = ?", params![new_title, chat_id])
        .map_err(|e| format!("Update error: {}", e))?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_chat(chat_id: i32) -> Result<(), String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("DELETE FROM chats WHERE id = ?", params![chat_id])
        .map_err(|e| format!("Delete error: {}", e))?;

    Ok(())
}

// Messages

#[tauri::command(rename_all = "snake_case")]
pub fn get_messages_by_chat(chat_id: i32) -> Result<Vec<entities::Message>, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    let mut stmt = conn.prepare("SELECT id, chat_id, model_id, text, sender FROM messages WHERE chat_id = ?")
        .map_err(|e| format!("Prepare error: {}", e))?;

    let message_iter = stmt.query_map(params![chat_id], |row| {
        Ok(entities::Message {
            id: row.get(0)?,
            chat_id: row.get(1)?,
            model_id: row.get(2)?,
            text: row.get(3)?,
            sender: row.get(4)?
        })
    }).map_err(|e| format!("Query error: {}", e))?;

    let mut messages = Vec::new();
    for message_result in message_iter {
        let message = message_result.map_err(|e| format!("Row error: {}", e))?;
        messages.push(message);
    }

    return Ok(messages);
}

#[tauri::command(rename_all = "snake_case")]
pub fn create_message(chat_id: i32, model_id: i32, text: String, sender: i32) -> Result<entities::Message, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("INSERT INTO messages (chat_id, model_id, text, sender) VALUES (?, ?, ?, ?)",
                 params![chat_id, model_id, text, sender])
        .map_err(|e| format!("Insert error: {}", e))?;

    let last_id = conn.last_insert_rowid();

    Ok(entities::Message {
        id: last_id as i32,
        chat_id,
        model_id,
        text,
        sender
    })
}

// rename_all=snake_case forces attribute names in JavaScript to match Rust,
// otherwise will convert attribute names from snake_case to camelCase.
// I chose to use snake_case option to align between frontend and backend
// to reduce confusion.

// use tauri::ipc::Response;
// use crate::error_handler;

// #[tauri::command(rename_all="snake_case")]
// pub fn hello_world(invoke_message: String) -> String {
//     println!("I was invoked from JavaScript, with this message: {}", invoke_message);
//     return "Hello from Rust!".into();
// }

// #[tauri::command(rename_all="snake_case")]
// pub fn read_file() -> String {
//     let data = std::fs::read("../resources/test_file.txt");
//     match data {
//         Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
//         Err(e) => format!("Failed to read file: {}", e),
//     }
// }
