use crate::models::{AppSettings, ChatSession};
use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};

pub trait PersistenceStore {
    fn save_session(&mut self, session: &ChatSession) -> Result<()>;
    fn load_session(&self, id: &str) -> Result<Option<ChatSession>>;
    fn load_all_sessions(&self) -> Result<Vec<ChatSession>>;
    fn delete_session(&mut self, id: &str) -> Result<()>;
    fn save_settings(&mut self, settings: &AppSettings) -> Result<()>;
    fn load_settings(&self) -> Result<Option<AppSettings>>;
}

/// SQLite-based persistence implementation
pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    /// Creates a new SQLite store at the given path
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let mut store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    /// Creates an in-memory SQLite store (useful for testing)
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let mut store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    /// Initializes the database schema
    fn init_schema(&mut self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                messages TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                preview TEXT,
                is_archived INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_updated_at
            ON sessions(updated_at DESC);

            CREATE INDEX IF NOT EXISTS idx_sessions_archived
            ON sessions(is_archived, updated_at DESC);
            "#,
        )?;
        Ok(())
    }

    /// Returns the default database path for the application
    pub fn default_db_path() -> Result<PathBuf> {
        let app_dir = dirs::data_local_dir()
            .context("Failed to get local data directory")?
            .join("zclone");

        std::fs::create_dir_all(&app_dir)?;
        Ok(app_dir.join("zclone.db"))
    }
}

impl PersistenceStore for SqliteStore {
    fn save_session(&mut self, session: &ChatSession) -> Result<()> {
        let messages_json = serde_json::to_string(&session.messages)?;

        self.conn.execute(
            r#"
            INSERT INTO sessions (id, title, messages, created_at, updated_at, preview, is_archived)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                messages = excluded.messages,
                updated_at = excluded.updated_at,
                preview = excluded.preview,
                is_archived = excluded.is_archived
            "#,
            params![
                session.id,
                session.title,
                messages_json,
                session.created_at,
                session.updated_at,
                session.preview,
                session.is_archived as i32,
            ],
        )?;

        Ok(())
    }

    fn load_session(&self, id: &str) -> Result<Option<ChatSession>> {
        let result: Option<ChatSession> = self.conn
            .query_row(
                "SELECT id, title, messages, created_at, updated_at, preview, is_archived
                 FROM sessions WHERE id = ?1",
                params![id],
                |row| {
                    let messages_json: String = row.get(2)?;
                    let messages = serde_json::from_str(&messages_json)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                    Ok(ChatSession {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        messages,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                        preview: row.get(5)?,
                        is_archived: row.get::<_, i32>(6)? != 0,
                    })
                },
            )
            .optional()?;

        Ok(result)
    }

    fn load_all_sessions(&self) -> Result<Vec<ChatSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, messages, created_at, updated_at, preview, is_archived
             FROM sessions
             ORDER BY updated_at DESC"
        )?;

        let sessions = stmt.query_map([], |row| {
            let messages_json: String = row.get(2)?;
            let messages = serde_json::from_str(&messages_json)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            Ok(ChatSession {
                id: row.get(0)?,
                title: row.get(1)?,
                messages,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                preview: row.get(5)?,
                is_archived: row.get::<_, i32>(6)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    fn delete_session(&mut self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn save_settings(&mut self, settings: &AppSettings) -> Result<()> {
        let settings_json = serde_json::to_string(settings)?;

        self.conn.execute(
            "INSERT INTO settings (key, value) VALUES ('app_settings', ?1)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![settings_json],
        )?;

        Ok(())
    }

    fn load_settings(&self) -> Result<Option<AppSettings>> {
        let result: Option<String> = self.conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'app_settings'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        match result {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Role, Theme};

    #[test]
    fn test_session_persistence() {
        let mut store = SqliteStore::new_in_memory().unwrap();

        let mut session = ChatSession::new("Test Session");
        session.add_message(Role::User, "Hello");
        session.add_message(Role::Assistant, "Hi there!");

        // Save session
        store.save_session(&session).unwrap();

        // Load session
        let loaded = store.load_session(&session.id).unwrap().unwrap();
        assert_eq!(loaded.id, session.id);
        assert_eq!(loaded.title, session.title);
        assert_eq!(loaded.messages.len(), 2);
    }

    #[test]
    fn test_settings_persistence() {
        let mut store = SqliteStore::new_in_memory().unwrap();

        let mut settings = AppSettings::default();
        settings.theme = Theme::Light;
        settings.model = "gpt-3.5-turbo".to_string();

        // Save settings
        store.save_settings(&settings).unwrap();

        // Load settings
        let loaded = store.load_settings().unwrap().unwrap();
        assert_eq!(loaded.theme, Theme::Light);
        assert_eq!(loaded.model, "gpt-3.5-turbo");
    }

    #[test]
    fn test_delete_session() {
        let mut store = SqliteStore::new_in_memory().unwrap();

        let session = ChatSession::new("Test");
        store.save_session(&session).unwrap();

        // Verify it exists
        assert!(store.load_session(&session.id).unwrap().is_some());

        // Delete it
        store.delete_session(&session.id).unwrap();

        // Verify it's gone
        assert!(store.load_session(&session.id).unwrap().is_none());
    }
}
