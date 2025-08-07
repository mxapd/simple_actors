use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut actor = Counter { count: 0 };

    thread::spawn(move || {
        actor.run(rx);
    });

    tx.send(Message::Increment).unwrap();
    tx.send(Message::Increment).unwrap();

    let (reply_tx, reply_rx) = mpsc::channel();
    tx.send(Message::GetCount(reply_tx)).unwrap();

    let count = reply_rx.recv().unwrap();
    println!("Count {}", count);
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
