use rusqlite::{params, Connection, Result};

#[tauri::command(rename_all="snake_case")]
pub fn get_models() -> String {
    let conn = match Connection::open("../../data/journal.db") {
        Ok(c) => c,
        Err(e) => return format!("DB open error: {}", e),
    };

    let mut stmt = match conn.prepare("SELECT id, name FROM models") {
        Ok(s) => s,
        Err(e) => return format!("Prepare error: {}", e),
    };

    let model_iter = match stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    }) {
        Ok(iter) => iter,
        Err(e) => return format!("Query error: {}", e),
    };

    for model in model_iter {
        match model {
            Ok((id, name)) => println!("Model {}: {}", id, name),
            Err(e) => return format!("Row error: {}", e),
        }
    }

    "Okay!".to_string()
}

