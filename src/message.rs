use crate::prompt::Prompt;
use tokio::sync::oneshot;

pub struct PromptMessage {
    pub prompt: Prompt,
    pub reply_addr: oneshot::Sender<String>,
}
