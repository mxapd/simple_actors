use crate::message::PromptMessage;
use reqwest::ClientBuilder;

pub struct PromptActor {
    receiver: tokio::sync::mpsc::Receiver<PromptMessage>,
}

impl PromptActor {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<PromptMessage>) -> Self {
        Self { receiver }
    }

    pub async fn run(mut self) {
        let http_client = ClientBuilder::new().build().unwrap();

        while let Some(msg) = self.receiver.recv().await {
            let PromptMessage { prompt, reply } = msg;

            let prompt_json = serde_json::to_string(&prompt).unwrap_or_default();

            let response = http_client
                .post("http://127.0.0.1:11434/api/generate")
                .header("Content-Type", "application/json")
                .body(prompt_json.clone())
                .send()
                .await;

            let response_text = match response {
                Ok(resp) => match resp.text().await {
                    Ok(text) => text,
                    Err(err) => format!("Failed to read response: {}", err),
                },
                Err(err) => format!("HTTP request failed: {}", err),
            };

            //          println!("PromptActor | Sent promt: {}", prompt_json);
            //          println!("PromptActor | Got response: {}", response_text);

            let _ = reply.send(response_text);
        }
    }
}
