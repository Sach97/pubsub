use crossbeam::{
    channel::{Iter, Receiver, Sender},
    unbounded,
};

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex, MutexGuard},
};

pub struct PubSub {
    topics: Arc<Mutex<HashMap<String, Topic>>>,
}

#[allow(dead_code)]
pub struct Topic {
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

    fn listen(&mut self) -> Iter<'_, Message> {
        self.receiver.iter()
    }
}

impl Drop for Topic {
    fn drop(&mut self) {
        drop(self.sender.to_owned());
    }
}

pub struct Message {
    body: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.body)
    }
}

impl Message {
    fn new(body: &str) -> Message {
        Message {
            body: body.to_owned(),
        }
    }
}

pub trait PubSubTrait {
    fn new() -> PubSub;
    fn subscribe(&mut self, channel: &str);
    fn unsubscribe(&mut self, channel: &str);
    fn publish(&mut self, channel: &str, body: &str);
    fn get_topics(&mut self) -> Vec<String>;
    fn topics(
        &mut self,
    ) -> std::sync::MutexGuard<'_, std::collections::HashMap<std::string::String, Topic>>;
    //fn listen(&self,channel: &str)-> Iter<'_, Message>;
}

//type Topics = MutexGuard<'_, HashMap<String, Topic>>;
//https://www.reddit.com/r/rust/comments/ay1t2i/cant_get_shared_hashmap_across_threads_to_work/
impl PubSubTrait for PubSub {
    fn get_topics(&mut self) -> Vec<String> {
        self.topics()
            .keys()
            .map(|topic| topic.clone())
            .collect::<Vec<String>>()
    }

    fn topics(&mut self) -> MutexGuard<'_, HashMap<String, Topic>> {
        return self.topics.lock().unwrap();
    }

    fn new() -> PubSub {
        let topics = Arc::new(Mutex::new(HashMap::new()));
        PubSub { topics: topics }
    }

    fn subscribe(&mut self, channel: &str) {
        //TODO: launch a new thread
        let (s, r) = unbounded();
        //loop {
        //https://stackoverflow.com/questions/39045636/how-do-i-have-one-thread-that-deletes-from-a-hashmap-and-another-that-inserts
        self.topics().insert(channel.to_owned(), Topic::new(&s, &r)); // lock the mutex, insert a value, unlock
                                                                      // }
    }
    fn unsubscribe(&mut self, channel: &str) {
        //TODO: stop the thread we launched with subscribe method with join handle. Hmm but the drop sender should drop the thread but we'll see
        //loop {
        drop(self.topics().get_mut(channel)); //drop sender
        self.topics().remove(channel);
        //}
    }

    fn publish(&mut self, channel: &str, body: &str) {
        // loop {
        self.topics().get_mut(channel).unwrap().send(body);
        //}
    }

    // fn listen(&self,channel: &str)-> Iter<'_, Message> {
    //        let mut topics = self.topics();
    //         topics.get_mut(channel).unwrap().listen()
    // }
}

fn main() {
    let mut pubsub = PubSub::new();
    pubsub.subscribe("mytopic");
    let topics = pubsub.get_topics();
    println!("topics : {:?}", topics);
    pubsub.publish("mytopic", "hello");
    let mut mutex = pubsub.topics.lock().unwrap();
    let messages = mutex.get_mut("mytopic").unwrap().listen();
    for message in messages {
        println!("{}", message);
    }
}
