use rusqlite::{params, Connection, Result};
use crate::entities;

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
