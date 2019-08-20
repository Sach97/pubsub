use crossbeam::{
    channel::{Iter, Receiver, Sender},
    unbounded,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct PubSub {
    topics: Arc<Mutex<HashMap<String, Topic>>>,
}

#[allow(dead_code)]
struct Topic {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Topic {
    fn new(sender: &Sender<Message>, receiver: &Receiver<Message>) -> Topic {
        Topic {
            sender: sender.to_owned(),
            receiver: receiver.to_owned(),
        }
    }
    fn send(&mut self, body: &str) {
        self.sender
            .send(Message::new(body))
            .expect("error sending message to topic");
    }
    #[allow(dead_code)]
    fn listen(&mut self) -> Iter<'_, Message> {
        self.receiver.iter()
    }
}

impl Drop for Topic {
    fn drop(&mut self) {
        drop(self.sender.to_owned());
    }
}

#[allow(dead_code)]
pub struct Message {
    body: String,
}

impl Message {
    fn new(body: &str) -> Message {
        Message {
            body: body.to_owned(),
        }
    }
}

// enum PublishStatus {
//     PublishSucess(Topic,Message),
//     PublishError(Topic,Message),
// }

// enum UnsubscribeStatus {
//     UnsubscribeSucess(Message),
//     UnsubscribeError(Message),
// }
// enum SubscribeStatus {
//     SubscribeSucess(Message),
//     SubscribeError(Message),
// }

pub trait PubSubTrait {
    fn new() -> PubSub;
    fn subscribe(&mut self, channel: &str) -> Option<Receiver<Message>>; //-> SubscribeStatus;
    fn unsubscribe(&mut self, channel: &str) -> Option<Receiver<Message>>; //-> UnsubscribeStatus;
    fn publish(&mut self, channel: &str, body: &str); //-> PublishStatus;
    fn topics(&mut self) -> Vec<String>;
}

//https://www.reddit.com/r/rust/comments/ay1t2i/cant_get_shared_hashmap_across_threads_to_work/
impl PubSubTrait for PubSub {
    fn topics(&mut self) -> Vec<String> {
        self.topics
            .lock()
            .unwrap()
            .keys()
            .map(|topic| topic.clone())
            .collect::<Vec<String>>()
    }

    fn new() -> PubSub {
        let topics = Arc::new(Mutex::new(HashMap::new()));
        PubSub { topics: topics }
    }

    fn subscribe(&mut self, channel: &str) -> Option<Receiver<Message>> {
        //TODO: launch a new thread
        let (s, r) = unbounded();
        loop {
            //https://stackoverflow.com/questions/39045636/how-do-i-have-one-thread-that-deletes-from-a-hashmap-and-another-that-inserts
            self.topics
                .lock()
                .unwrap()
                .insert(channel.to_owned(), Topic::new(&s, &r)); // lock the mutex, insert a value, unlock
        }
    }
    fn unsubscribe(&mut self, channel: &str) -> Option<Receiver<Message>> {
        //TODO: stop the thread we launched with subscribe method with join handle. Hmm but the drop sender should drop the thread but we'll see
        loop {
            drop(self.topics.lock().unwrap().get_mut(channel));
            self.topics.lock().unwrap().remove(channel);
        }
    }

    fn publish(&mut self, channel: &str, body: &str) {
        loop {
            self.topics
                .lock()
                .unwrap()
                .get_mut(channel)
                .unwrap()
                .send(body);
        }
    }
}

fn main() {
    let mut pubsub = PubSub::new();
    pubsub.subscribe("mytopic");
    let topics = pubsub.topics();
    println!("topics : {:?}", topics);
    pubsub.publish("mytopic", "hello");
}
