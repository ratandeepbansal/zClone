use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use std::sync::Arc;

/// Represents a request to the OpenAI API
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub session_id: String,
    pub message_id: String,
    pub messages: Vec<crate::models::Message>,
    pub model: String,
    pub temperature: f32,
    pub system_prompt: Option<String>,
}

/// Represents a chunk of streaming response
#[derive(Debug, Clone)]
pub struct ChatResponseChunk {
    pub session_id: String,
    pub message_id: String,
    pub content: String,
    pub is_final: bool,
}

/// Represents an error in the chat pipeline
#[derive(Debug, Clone)]
pub struct ChatError {
    pub session_id: String,
    pub message_id: String,
    pub error: String,
}

/// Events that can be emitted by the messaging pipeline
#[derive(Debug, Clone)]
pub enum ChatEvent {
    ResponseChunk(ChatResponseChunk),
    Error(ChatError),
    Cancelled { session_id: String, message_id: String },
}

/// Handle for cancelling an in-flight request
pub struct CancellationHandle {
    tx: mpsc::Sender<()>,
}

impl CancellationHandle {
    pub fn new(tx: mpsc::Sender<()>) -> Self {
        Self { tx }
    }

    /// Cancels the associated request
    pub async fn cancel(&self) -> Result<()> {
        self.tx.send(()).await?;
        Ok(())
    }
}

/// Trait for implementing a chat backend (OpenAI, local models, etc.)
#[async_trait]
pub trait ChatBackend: Send + Sync {
    /// Sends a chat request and returns a stream of response chunks
    async fn send_request(
        &self,
        request: ChatRequest,
        event_tx: mpsc::Sender<ChatEvent>,
        mut cancel_rx: mpsc::Receiver<()>,
    ) -> Result<()>;
}

/// The main messaging pipeline that manages chat requests
pub struct MessagingPipeline {
    backend: Arc<dyn ChatBackend>,
    event_tx: mpsc::Sender<ChatEvent>,
    event_rx: mpsc::Receiver<ChatEvent>,
}

impl MessagingPipeline {
    pub fn new(backend: Arc<dyn ChatBackend>) -> Self {
        let (event_tx, event_rx) = mpsc::channel(100);
        Self {
            backend,
            event_tx,
            event_rx,
        }
    }

    /// Sends a chat request and returns a cancellation handle
    pub fn send_request(&self, request: ChatRequest) -> CancellationHandle {
        let (cancel_tx, cancel_rx) = mpsc::channel(1);
        let backend = Arc::clone(&self.backend);
        let event_tx = self.event_tx.clone();

        tokio::spawn(async move {
            if let Err(e) = backend.send_request(request.clone(), event_tx.clone(), cancel_rx).await {
                let _ = event_tx.send(ChatEvent::Error(ChatError {
                    session_id: request.session_id,
                    message_id: request.message_id,
                    error: e.to_string(),
                })).await;
            }
        });

        CancellationHandle::new(cancel_tx)
    }

    /// Gets the receiver for chat events
    pub fn event_receiver(&mut self) -> &mut mpsc::Receiver<ChatEvent> {
        &mut self.event_rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Message, Role};

    #[derive(Clone)]
    struct MockBackend;

    #[async_trait]
    impl ChatBackend for MockBackend {
        async fn send_request(
            &self,
            request: ChatRequest,
            event_tx: mpsc::Sender<ChatEvent>,
            mut cancel_rx: mpsc::Receiver<()>,
        ) -> Result<()> {
            // Simulate streaming response
            for i in 0..5 {
                // Check for cancellation
                if cancel_rx.try_recv().is_ok() {
                    event_tx.send(ChatEvent::Cancelled {
                        session_id: request.session_id.clone(),
                        message_id: request.message_id.clone(),
                    }).await?;
                    return Ok(());
                }

                event_tx.send(ChatEvent::ResponseChunk(ChatResponseChunk {
                    session_id: request.session_id.clone(),
                    message_id: request.message_id.clone(),
                    content: format!("chunk {} ", i),
                    is_final: i == 4,
                })).await?;

                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_messaging_pipeline() {
        let backend = Arc::new(MockBackend);
        let mut pipeline = MessagingPipeline::new(backend);

        let request = ChatRequest {
            session_id: "test-session".to_string(),
            message_id: "test-message".to_string(),
            messages: vec![Message::new(Role::User, "Hello")],
            model: "gpt-4".to_string(),
            temperature: 0.7,
            system_prompt: None,
        };

        let _handle = pipeline.send_request(request);

        // Collect events
        let mut chunks = Vec::new();
        while let Some(event) = pipeline.event_receiver().recv().await {
            match event {
                ChatEvent::ResponseChunk(chunk) => {
                    chunks.push(chunk.content.clone());
                    if chunk.is_final {
                        break;
                    }
                }
                _ => {}
            }
        }

        assert_eq!(chunks.len(), 5);
    }
}
