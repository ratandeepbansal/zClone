use crate::models::{ChatSession, Role};
use std::collections::HashMap;

pub struct SessionManager {
    sessions: HashMap<String, ChatSession>,
    active_session_id: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            active_session_id: None,
        }
    }

    pub fn create_session(&mut self, title: impl Into<String>) -> String {
        let session = ChatSession::new(title);
        let id = session.id.clone();
        self.sessions.insert(id.clone(), session);
        self.active_session_id = Some(id.clone());
        id
    }

    pub fn get_session(&self, id: &str) -> Option<&ChatSession> {
        self.sessions.get(id)
    }

    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut ChatSession> {
        self.sessions.get_mut(id)
    }

    pub fn set_active_session(&mut self, id: String) {
        if self.sessions.contains_key(&id) {
            self.active_session_id = Some(id);
        }
    }

    pub fn get_active_session(&self) -> Option<&ChatSession> {
        self.active_session_id
            .as_ref()
            .and_then(|id| self.sessions.get(id))
    }

    pub fn get_active_session_mut(&mut self) -> Option<&mut ChatSession> {
        let id = self.active_session_id.clone()?;
        self.sessions.get_mut(&id)
    }

    pub fn list_sessions(&self) -> Vec<&ChatSession> {
        let mut sessions: Vec<_> = self.sessions.values().collect();
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        sessions
    }

    pub fn delete_session(&mut self, id: &str) -> bool {
        if let Some(active_id) = &self.active_session_id {
            if active_id == id {
                self.active_session_id = None;
            }
        }
        self.sessions.remove(id).is_some()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
