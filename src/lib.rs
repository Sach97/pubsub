mod message;
use message::Message;
mod topics;
use crate::topics::{Topics, Topic ,TopicsMethods};
use crossbeam::{channel::IntoIter, unbounded};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct PubSub {
    topics: Arc<Mutex<HashMap<String, Topic>>>,
}

impl PubSub {
    pub fn from(topics: Arc<Mutex<HashMap<String, Topic>>>) -> PubSub {
        PubSub { topics: topics }
    }

    pub fn clone(pubsub: &PubSub) -> Arc<Mutex<HashMap<String, Topic>>> {
        return Arc::clone(&pubsub.topics);
    }
}

pub trait PubSubTrait {
    fn new() -> PubSub;
    fn subscribe(&mut self, topic: &str);
    fn unsubscribe(&mut self, topic: &str);
    fn publish(&mut self, topic: &str, body: &str);
    fn get_topics(&mut self) -> Vec<String>;
    fn topics(&mut self) -> Topics;
    fn listen(&mut self, topic: &str) -> IntoIter<Message>;
}

impl PubSubTrait for PubSub {
    fn get_topics(&mut self) -> Vec<String> {
        self.topics()
            .keys()
            .map(|topic| topic.clone())
            .collect::<Vec<String>>()
    }

    fn topics(&mut self) -> Topics {
        return self.topics.lock().unwrap();
    }

    fn new() -> PubSub {
        let topics = Arc::new(Mutex::new(HashMap::new()));
        PubSub { topics: topics }
    }

    fn subscribe(&mut self, topic: &str) {
        let (s, r) = unbounded();
        self.topics()
            .insert(String::from(topic), Topic::new(&s, &r));
    }

    fn unsubscribe(&mut self, topic: &str) {
        drop(self.topics().retrieve(topic));
        self.topics().remove(topic);
    }

    fn publish(&mut self, topic: &str, body: &str) {
        self.topics().retrieve(topic).send_message(body);
    }

    fn listen(&mut self, topic: &str) -> IntoIter<Message> {
        self.topics().retrieve(topic).clone().listen()
    }
}
