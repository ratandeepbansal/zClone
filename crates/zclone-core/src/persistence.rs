// Placeholder for persistence layer
// Will be implemented in Phase 2

use crate::models::{AppSettings, ChatSession};
use anyhow::Result;

pub trait PersistenceStore {
    fn save_session(&self, session: &ChatSession) -> Result<()>;
    fn load_session(&self, id: &str) -> Result<Option<ChatSession>>;
    fn load_all_sessions(&self) -> Result<Vec<ChatSession>>;
    fn delete_session(&self, id: &str) -> Result<()>;
    fn save_settings(&self, settings: &AppSettings) -> Result<()>;
    fn load_settings(&self) -> Result<Option<AppSettings>>;
}
