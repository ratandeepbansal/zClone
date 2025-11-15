// Placeholder for OpenAI client
// Will be implemented in Phase 6

use anyhow::Result;
use zclone_core::Message;

#[async_trait::async_trait]
pub trait ChatClient {
    async fn send_message(&self, messages: Vec<Message>) -> Result<String>;
    async fn stream_message(
        &self,
        messages: Vec<Message>,
    ) -> Result<futures::stream::BoxStream<'static, Result<String>>>;
}
