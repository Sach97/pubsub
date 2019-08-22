use crossbeam::{
    channel::{IntoIter, Receiver, Sender},
};
use std::{
    collections::HashMap,
    sync::{MutexGuard},
};
use crate::message::Message;

#[derive(Clone)]
pub struct Topic {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Topic {
    pub fn new(sender: &Sender<Message>, receiver: &Receiver<Message>) -> Topic {
        Topic {
            sender: sender.to_owned(),
            receiver: receiver.to_owned(),
        }
    }
   pub fn send_message(&mut self, body: &str) {
        self.sender
            .send(Message::new(body))
            .expect("error sending message to topic");
    }

    pub fn listen(self) -> IntoIter<Message> {
        self.receiver.into_iter()
    }
}

//Hmm bad to comment this as we can't drop topic anymore
// impl Drop for Topic {
//     fn drop(&mut self) {
//         drop(self.sender.to_owned());
//     }
// }

pub type Topics<'a> = MutexGuard<'a, HashMap<String, Topic>>;

pub trait TopicsMethods {
    fn retrieve(&mut self, topic: &str) -> &mut Topic;
}

impl<'a> TopicsMethods for Topics<'a> {
    fn retrieve(&mut self, topic: &str) -> &mut Topic {
        return self.get_mut(topic).unwrap();
    }
}