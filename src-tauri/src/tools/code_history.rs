use crate::db;
use rusqlite::{params, Connection, OptionalExtension};
use tauri::{command, AppHandle};

fn get_connection(app: &AppHandle) -> Result<Connection, String> {
    // Use the centralized database connection function with AppHandle for path resolution
    db::get_connection(app)
}

#[derive(serde::Deserialize, Debug)]
pub struct CursorPosition {
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "column")]
    pub column: i64,
}

#[command]
pub async fn code_add(
    app: AppHandle,
    tab_id: i64,
    code: String,
    cursor: CursorPosition,
) -> Result<serde_json::Value, String> {
    let conn = get_connection(&app)?;

    // Validate inputs
    if tab_id <= 0 {
        return Err("tab_id must be positive".to_string());
    }
    if cursor.line_number <= 0 {
        return Err("cursor lineNumber must be positive".to_string());
    }
    if cursor.column <= 0 {
        return Err("cursor column must be positive".to_string());
    }

    // Use transaction to ensure atomicity (matching original behavior)
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // Get current state
    let current_state: Option<i64> = tx
        .query_row(
            "SELECT current_history_id FROM tab_states WHERE tab_id = ?1",
            params![tab_id],
            |row| {
                let val: Option<i64> = row.get(0)?;
                Ok(val)
            },
        )
        .optional()
        .map_err(|e| format!("Failed to query current state: {}", e))?
        .flatten();

    // If we have a current state, check if code hasn't changed
    if let Some(current_history_id) = current_state {
        if current_history_id > 0 {
            let last_history: Option<String> = tx
                .query_row(
                    "SELECT code FROM code_histories WHERE id = ?1",
                    params![current_history_id],
                    |row| Ok(row.get(0)?),
                )
                .optional()
                .map_err(|e| format!("Failed to query last history: {}", e))?;

            // If code hasn't changed, just update cursor position
            if let Some(last_code) = last_history {
                if last_code == code {
                    tx.execute(
                        "UPDATE code_histories SET cursor_line = ?1, cursor_column = ?2 WHERE id = ?3",
                        params![cursor.line_number, cursor.column, current_history_id],
                    )
                    .map_err(|e| format!("Failed to update cursor: {}", e))?;

                    tx.commit()
                        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

                    return Ok(serde_json::json!({
                        "success": true
                    }));
                }
            }

            // Delete future history (branches) when adding new code
            tx.execute(
                "DELETE FROM code_histories WHERE tab_id = ?1 AND id > ?2",
                params![tab_id, current_history_id],
            )
            .map_err(|e| format!("Failed to delete future history: {}", e))?;
        }
    }

    // Insert new history row
    tx.execute(
        "INSERT INTO code_histories (tab_id, code, cursor_line, cursor_column, created_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        params![tab_id, code, cursor.line_number, cursor.column],
    )
    .map_err(|e| format!("Failed to insert code history: {}", e))?;

    let new_history_id = tx.last_insert_rowid();

    // Update or insert tab state
    tx.execute(
        "INSERT OR REPLACE INTO tab_states (tab_id, current_history_id) VALUES (?1, ?2)",
        params![tab_id, new_history_id],
    )
    .map_err(|e| format!("Failed to update tab state: {}", e))?;

    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(serde_json::json!({
        "success": true
    }))
}

#[command]
pub async fn code_undo(app: AppHandle, tab_id: i64) -> Result<serde_json::Value, String> {
    // Validate input
    if tab_id <= 0 {
        return Err("tab_id must be positive".to_string());
    }

    let conn = get_connection(&app)?;

    // Get current state - must exist to undo
    let current_state: Option<i64> = conn
        .query_row(
            "SELECT current_history_id FROM tab_states WHERE tab_id = ?1",
            params![tab_id],
            |row| {
                let val: Option<i64> = row.get(0)?;
                Ok(val)
            },
        )
        .optional()
        .map_err(|e| format!("Failed to read tab state: {}", e))?
        .flatten()
        .filter(|&id| id > 0);

    let Some(current_id) = current_state else {
        return Ok(serde_json::json!({
            "data": serde_json::Value::Null,
            "error": "No previous state to undo."
        }));
    };

    // Find previous snapshot (id < current_id)
    let previous_state: Option<(i64, String, i64, i64)> = conn
        .query_row(
            "SELECT id, code, cursor_line, cursor_column FROM code_histories
             WHERE tab_id = ?1 AND id < ?2 ORDER BY id DESC LIMIT 1",
            params![tab_id, current_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .optional()
        .map_err(|e| format!("Failed to query previous history: {}", e))?;

    match previous_state {
        Some((id, code, cursor_line, cursor_column)) => {
            // Update tab state to point to previous history
            conn.execute(
                "UPDATE tab_states SET current_history_id = ?1 WHERE tab_id = ?2",
                params![id, tab_id],
            )
            .map_err(|e| format!("Failed to update tab state: {}", e))?;

            Ok(serde_json::json!({
                "data": {
                    "code": code,
                    "cursor": {
                        "lineNumber": cursor_line,
                        "column": cursor_column
                    }
                },
                "error": serde_json::Value::Null
            }))
        }
        None => Ok(serde_json::json!({
            "data": serde_json::Value::Null,
            "error": "No previous state to undo."
        })),
    }
}

#[command]
pub async fn code_redo(app: AppHandle, tab_id: i64) -> Result<serde_json::Value, String> {
    // Validate input
    if tab_id <= 0 {
        return Err("tab_id must be positive".to_string());
    }

    let conn = get_connection(&app)?;

    // Get current state - must exist to redo
    let current_state: Option<i64> = conn
        .query_row(
            "SELECT current_history_id FROM tab_states WHERE tab_id = ?1",
            params![tab_id],
            |row| {
                let val: Option<i64> = row.get(0)?;
                Ok(val)
            },
        )
        .optional()
        .map_err(|e| format!("Failed to read tab state: {}", e))?
        .flatten()
        .filter(|&id| id > 0);

    let Some(current_id) = current_state else {
        return Ok(serde_json::json!({
            "data": serde_json::Value::Null,
            "error": "No next state to redo."
        }));
    };

    // Find next snapshot (id > current_id)
    let next_state: Option<(i64, String, i64, i64)> = conn
        .query_row(
            "SELECT id, code, cursor_line, cursor_column FROM code_histories
             WHERE tab_id = ?1 AND id > ?2 ORDER BY id ASC LIMIT 1",
            params![tab_id, current_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .optional()
        .map_err(|e| format!("Failed to query next history: {}", e))?;

    match next_state {
        Some((id, code, cursor_line, cursor_column)) => {
            // Update tab state to point to next history
            conn.execute(
                "UPDATE tab_states SET current_history_id = ?1 WHERE tab_id = ?2",
                params![id, tab_id],
            )
            .map_err(|e| format!("Failed to update tab state: {}", e))?;

            Ok(serde_json::json!({
                "data": {
                    "code": code,
                    "cursor": {
                        "lineNumber": cursor_line,
                        "column": cursor_column
                    }
                },
                "error": serde_json::Value::Null
            }))
        }
        None => Ok(serde_json::json!({
            "data": serde_json::Value::Null,
            "error": "No next state to redo."
        })),
    }
}
