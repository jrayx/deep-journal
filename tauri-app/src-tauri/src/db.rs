use rusqlite::{params, Connection, Result};
use crate::entities;

// Models

#[tauri::command(rename_all = "snake_case")]
pub fn get_models() -> Result<Vec<entities::Model>, String> {
    let conn = Connection::open("../../data/journal.db")
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
    let conn = Connection::open("../../data/journal.db")
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
    let conn = Connection::open("../../data/journal.db")
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("DELETE FROM models WHERE id = ?", params![model_id])
        .map_err(|e| format!("Delete error: {}", e))?;

    Ok(())
}


// Chats

#[tauri::command(rename_all = "snake_case")]
pub fn get_chats() -> Result<Vec<entities::Chat>, String> {
    let conn = Connection::open("../../data/journal.db")
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
    let conn = Connection::open("../../data/journal.db")
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
pub fn update_chat_title(chat_id: i32, new_chat_title: String) -> Result<(), String> {
    let conn = Connection::open("../../data/journal.db")
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("UPDATE chats SET title = ? WHERE id = ?", params![new_chat_title, chat_id])
        .map_err(|e| format!("Update error: {}", e))?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_chat(chat_id: i32) -> Result<(), String> {
    let conn = Connection::open("../../data/journal.db")
        .map_err(|e| format!("DB open error: {}", e))?;

    conn.execute("DELETE FROM chats WHERE id = ?", params![chat_id])
        .map_err(|e| format!("Delete error: {}", e))?;

    Ok(())
}

// Messages

#[tauri::command(rename_all = "snake_case")]
pub fn get_messages_by_chat(chat_id: i32) -> Result<Vec<entities::Message>, String> {
    let conn = Connection::open("../../data/journal.db")
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
    let conn = Connection::open("../../data/journal.db")
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
