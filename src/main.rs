use std::collections::{VecDeque,HashMap};

#[derive(Default)]
pub struct PubSub {
    topics : HashMap<Topic,VecDeque<Message>>,
}


#[derive(Hash,Eq)]
struct Topic {
    name:  String,
}

impl PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


impl Topic{
    fn new(name: &str) -> Topic {
        Topic {
            name : name.to_owned(),
        }
    }
}

pub struct Message {
    body: String, 
}

impl Message{
    fn new(body: &str) -> Message {
        Message { body : body.to_owned() }
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
    fn new()-> PubSub;
    fn subscribe(&mut self,topic: &str)-> Option<VecDeque<Message>>; //-> SubscribeStatus;
    fn unsubscribe(&mut self,topic: &str)-> Option<VecDeque<Message>>;//-> UnsubscribeStatus;
    fn publish(&mut self, topic: &str, message: &str); //-> PublishStatus;
    fn topics(self) -> Vec<String>;

 }

 impl PubSubTrait for PubSub {
     fn topics(self)->Vec<String>{
         self.topics.keys().map(|topic| topic.name.clone()).collect::<Vec<String>>()
     }

     fn new()-> PubSub{
        let topics: HashMap<Topic, VecDeque<Message>> = HashMap::new();
        PubSub {
            topics : topics
        }
     }

     fn subscribe(&mut self,topic: &str) -> Option<VecDeque<Message>>{
         let messages: VecDeque<Message> = VecDeque::new();
         self.topics.insert(Topic::new(topic),messages)
     }
     fn unsubscribe(&mut self,topic: &str)-> Option<VecDeque<Message>>{
         self.topics.remove(&Topic::new(topic))
     }

    fn publish(&mut self,topic: &str, message: &str){
        self.topics.get_mut(&Topic::new(topic)).unwrap().push_back(Message::new(message)); //iter_mut().map(|messages| messages.push_back(Message::new(message)));
    }
     
 }


fn main() {
    let mut pubsub = PubSub::new();
    pubsub.subscribe("mytopic");
    let topics = pubsub.topics();
    println!("topics : {:?}",topics);
}
