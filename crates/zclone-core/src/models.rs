use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: Role,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub messages: Vec<Message>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ChatSession {
    pub fn new(title: impl Into<String>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_message(&mut self, role: Role, content: impl Into<String>) {
        let message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            role,
            content: content.into(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        self.messages.push(message);
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub openai_api_key: Option<String>,
    pub theme: Theme,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            openai_api_key: None,
            theme: Theme::Dark,
            model: "gpt-4".to_string(),
        }
    }
}
