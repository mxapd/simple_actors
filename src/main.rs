fn main() {
    let mut actor = Counter { count: 0 };

    let mut mailbox = Vec::new();

    mailbox.push(Message::Increment);
    mailbox.push(Message::Increment);
    mailbox.push(Message::GetCount);

    for msg in mailbox {
        if let Some(count) = actor.handle(msg) {
            println!("Current count {}", count);
        }
    }
}

enum Message {
    Increment,
    GetCount,
}

struct Counter {
    count: u32,
}

impl Counter {
    fn handle(&mut self, msg: Message) -> Option<u32> {
        match msg {
            Message::Increment => {
                self.count += 1;
                None
            }

            Message::GetCount => Some(self.count),
        }
    }
}
