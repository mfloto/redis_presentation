use redis;
use redis::PubSubCommands;
use std::{io, thread};

use redis_demo::create_redis_connection;

fn main() {
    // Read topic name from stdin
    let topic = {
        println!("Please enter the topic you want to subscribe to:");
        let mut topic = String::new();
        io::stdin()
            .read_line(&mut topic)
            .expect("Failed to read line");
        topic.trim().to_owned()
    };

    // Clone the topic, so it can be moved to the thread
    let topic_for_subscribe = topic.clone();
    let topic_for_publish = topic;

    // Spawn a new thread to handle the subscription
    let subscribe_handle = thread::spawn(move || {
        // Create a redis connection
        let mut connection = create_redis_connection();

        // Subscribe to the topic
        let mut pubsub = connection.as_pubsub();
        pubsub
            .subscribe(&topic_for_subscribe)
            .expect("Failed to subscribe");

        // Print the messages received
        loop {
            let msg = pubsub.get_message().expect("Failed to get message"); // Blocks until a message is received
            let payload: String = msg.get_payload().expect("Failed to get payload");
            println!("Received on {} channel: {}", &topic_for_subscribe, payload);
        }
    });

    // Spawn thread to handle publishing
    let publish_handle = thread::spawn(move || {
        // Create a redis connection
        let mut connection = create_redis_connection();

        loop {
            // Read user message from stdin
            //println!("Please enter the number of messages you want to publish:");
            let mut msg = String::new();
            io::stdin()
                .read_line(&mut msg)
                .expect("Failed to read line");

            // Has the complexity of O(N+M) where N is the number of clients subscribed to the receiving channel
            // and M is the total number of subscribed patterns (by any client)
            let _ = redis::cmd("PUBLISH")
                .arg(&topic_for_publish)
                .arg(&msg)
                .execute(&mut connection);
        }
    });

    // Wait for the threads to finish
    subscribe_handle
        .join()
        .expect("Failed to join subscribe_handle");
    publish_handle
        .join()
        .expect("Failed to join publish_handle");
}
