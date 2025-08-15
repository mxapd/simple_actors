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
    let ollama_addr = "http://127.0.0.1:11434/api/generate".to_string();

    let (prompt_actor_addr, rx) = tokio::sync::mpsc::channel(32);

    let actor = PromptActor::new(rx, ollama_addr);
    tokio::spawn(actor.run());

    let prompt = Prompt {
        model: "gpt-oss:20b".to_string(),
        prompt: "is it working?".to_string(),
        stream: false,
    };

    let (reply_tx, reply_rx) = oneshot::channel();

    if let Err(e) = prompt_actor_addr
        .send(PromptMessage {
            prompt,
            reply_addr: reply_tx,
        })
        .await
    {
        eprintln!("Failed to send prompt to actor: {}", e);
        return;
    }

    match reply_rx.await {
        Ok(response) => {
            let value: Value = serde_json::from_str(&response).expect("serde_json parse error");
            println!("\n\nAI response: \n\n{}\n", value["response"])
        }
        Err(e) => eprintln!("Failed to receive response: {}", e),
    }

    //println!("Response from actor: {}", result);
    //println!("\n\nAI response: \n\n{}\n", value["response"]);
}
