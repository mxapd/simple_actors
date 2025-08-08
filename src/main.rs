use serde::Serialize;
use std::sync::mpsc;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(32);

    let actor = PromptActor::new(rx);
    tokio::spawn(actor.run());

    let prompt = Prompt {
        model: "gpt-oss:20b".to_string(),
        prompt: "is it working?".to_string(),
        stream: false,
    };

    let (reply_tx, reply_rx) = oneshot::channel();
    tx.send(PromptMessage {
        prompt,
        reply: reply_tx,
    })
    .await
    .unwrap();

    let result = reply_rx.await.unwrap();
    println!("Response from actor: {}", result);
}

enum Message {
    Increment,
    GetCount(mpsc::Sender<u32>),
}

struct Counter {
    count: u32,
}

impl Counter {
    fn run(&mut self, receiver: mpsc::Receiver<Message>) {
        for msg in receiver {
            match msg {
                Message::Increment => {
                    self.count += 1;
                }
                Message::GetCount(reply_to) => {
                    let _ = reply_to.send(self.count);
                }
            }
        }
    }
}

struct PromptActor {
    receiver: tokio::sync::mpsc::Receiver<PromptMessage>,
}

impl PromptActor {
    fn new(receiver: tokio::sync::mpsc::Receiver<PromptMessage>) -> Self {
        Self { receiver }
    }

    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            let PromptMessage { prompt, reply } = msg;

            let prompt_json = serde_json::to_string(&prompt).unwrap_or_default();

            let result = super::send_post(prompt_json)
                .await
                .unwrap_or_else(|_| "Error".to_string());

            let _ = reply.send(result);
        }
    }
}

struct PromptMessage {
    prompt: Prompt,
    reply: oneshot::Sender<String>,
}

#[derive(Debug, Serialize)]
struct Prompt {
    model: String,
    prompt: String,
    stream: bool,
}
