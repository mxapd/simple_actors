use crate::message::PromptMessage;
use crate::prompt::Prompt;
use reqwest::Client;

pub struct PromptActor {
    receiver: tokio::sync::mpsc::Receiver<PromptMessage>,
    http_client: reqwest::Client,
    api_url: String,
}

impl PromptActor {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<PromptMessage>, api_url: String) -> Self {
        Self {
            receiver,
            http_client: Client::new(),
            api_url: api_url,
        }
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            let PromptMessage { prompt, reply_addr } = msg;

            let response = match self.handle_prompt(prompt).await {
                Ok(response) => response,
                Err(e) => format!("Error sending or parsing response: {}", e),
            };

            let _ = reply_addr.send(response);
        }
    }

    async fn handle_prompt(&self, prompt: Prompt) -> Result<String, String> {
        let prompt_json = match serde_json::to_string(&prompt) {
            Ok(value) => value,
            Err(e) => return Err(format!("Failed to serialize prompt: {}", e)),
        };

        let response = match self
            .http_client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .body(prompt_json)
            .send()
            .await
        {
            Ok(value) => value,
            Err(e) => return Err(format!("Failed to get a response: {}", e)),
        };

        let response_text = match response.text().await {
            Ok(text) => text,
            Err(err) => return Err(format!("Failed to read response: {}", err)),
        };

        Ok(response_text)
    }
}
