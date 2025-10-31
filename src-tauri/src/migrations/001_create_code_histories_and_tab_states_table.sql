CREATE TABLE IF NOT EXISTS code_histories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tab_id INTEGER NOT NULL,
    code TEXT NOT NULL,
    cursor_line INTEGER NOT NULL,
    cursor_column INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tab_states (
    tab_id INTEGER PRIMARY KEY,
    current_history_id INTEGER,
    FOREIGN KEY (current_history_id) REFERENCES code_histories(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_code_histories_tab_id ON code_histories(tab_id);


