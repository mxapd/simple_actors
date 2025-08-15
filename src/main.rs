mod actor;
mod message;
mod prompt;

use actor::PromptActor;
use message::PromptMessage;
use prompt::Prompt;

use serde_json::Value;
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

    let value: Value = serde_json::from_str(&result);

    //println!("Response from actor: {}", result);
    println!("\n\nAI response: \n\n{}\n", value["response"]);
}
