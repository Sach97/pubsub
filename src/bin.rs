use structopt;
use structopt::StructOpt;
extern crate pubsub;
use crate::pubsub::PubSubTrait;
use pubsub::PubSub;
use std::thread;

#[derive(StructOpt, Debug)]
#[structopt(name = "pubsub", about = "the dead simple pubsub cli")]
enum PubSubCli {
    #[structopt(name = "subscribe")]
    Subscribe { topic: Option<String> },
}

fn main() {
    // match PubSubCli::from_args() {
    //     PubSubCli::Subscribe { topic } => println!("{:?}", topic),
    // }
    let mut pubsub = PubSub::new();
    pubsub.subscribe("firsttopic");
    pubsub.subscribe("secondtopic");
    let topics = pubsub.get_topics();
    pubsub.publish("firsttopic", "hello from firsttopic");
    pubsub.publish("secondtopic", "hello from secondtopic");
    println!("topics : {:?}", topics);
    let mut children = vec![];
    for topic in pubsub.get_topics() {
        let pubsub = PubSub::clone(&pubsub);
        children.push(thread::spawn(move || {
            let mut pubsub = PubSub::from(pubsub);
            for message in pubsub.listen(topic.as_str()) {
                println!("{}", message);
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
