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
    /// Indicates if this message is still being streamed
    pub is_streaming: bool,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role,
            content: content.into(),
            timestamp: chrono::Utc::now().timestamp(),
            is_streaming: false,
        }
    }

    /// Creates a new message that starts in streaming mode
    pub fn new_streaming(role: Role) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role,
            content: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
            is_streaming: true,
        }
    }

    /// Appends content to this message (for streaming)
    pub fn append_content(&mut self, content: impl Into<String>) {
        self.content.push_str(&content.into());
    }

    /// Marks streaming as complete
    pub fn complete_streaming(&mut self) {
        self.is_streaming = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub messages: Vec<Message>,
    pub created_at: i64,
    pub updated_at: i64,
    /// Preview text for sidebar (first user message or custom)
    pub preview: Option<String>,
    /// Whether this session is archived
    pub is_archived: bool,
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
            preview: None,
            is_archived: false,
        }
    }

    pub fn add_message(&mut self, role: Role, content: impl Into<String>) {
        let message = Message::new(role, content);
        self.messages.push(message);
        self.updated_at = chrono::Utc::now().timestamp();
        self.update_preview();
    }

    /// Updates the preview based on message content
    fn update_preview(&mut self) {
        if self.preview.is_none() {
            // Use first user message as preview
            if let Some(msg) = self.messages.iter().find(|m| matches!(m.role, Role::User)) {
                let preview = msg.content.chars().take(100).collect::<String>();
                self.preview = Some(if msg.content.len() > 100 {
                    format!("{}...", preview)
                } else {
                    preview
                });
            }
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn unarchive(&mut self) {
        self.is_archived = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub openai_api_key: Option<String>,
    pub theme: Theme,
    pub model: String,
    pub temperature: f32,
    pub system_prompt: Option<String>,
    /// Sidebar collapsed state
    pub sidebar_collapsed: bool,
    /// Window dimensions
    pub window_width: Option<u32>,
    pub window_height: Option<u32>,
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
            temperature: 0.7,
            system_prompt: None,
            sidebar_collapsed: false,
            window_width: None,
            window_height: None,
        }
    }
}

/// Application state container
#[derive(Debug, Clone)]
pub struct AppState {
    pub sessions: Vec<ChatSession>,
    pub active_session_id: Option<String>,
    pub settings: AppSettings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sessions: Vec::new(),
            active_session_id: None,
            settings: AppSettings::default(),
        }
    }
}
