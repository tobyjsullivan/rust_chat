extern crate rumqtt;

use rumqtt::{MqttOptions, MqttClient, MqttCallback, QoS};
use std::io;

fn main() {
    println!("Starting...");
    let topic = "hello/rustclub";

    let client_options = MqttOptions::new()
                                  .set_keep_alive(5)
                                  .set_reconnect(3)
                                  .set_client_id("rumqtt-docs")
                                  .set_broker("test.mosquitto.org:1883");

    let cb = MqttCallback::new();
    let cb = cb.on_message(move |msg| {
        let str_msg = String::from_utf8(msg.payload.to_vec());
        println!("{:?} message --> {:?}", msg.topic, str_msg);
    });

    let mut mq_client = MqttClient::start(client_options, Some(cb)).expect("Could not start client.");
    mq_client.subscribe(vec![(topic, QoS::Level1),("hello/rust", QoS::Level1)]).expect("Failed to sub1");
    
    let mut username_in = String::new();
    println!("Enter a username:");
    io::stdin().read_line(&mut username_in).expect("Failed to read username!");
    let username = username_in.trim();

    loop {
        let mut msg_in = String::new();
        println!("Enter a message:");
        io::stdin().read_line(&mut msg_in).expect("Failed to read message!");

        let msg = msg_in.trim();

        let payload = format!("[{}]: {}", username, msg);
        println!("Okay, I want to send: {}", payload);
        mq_client.publish(topic, QoS::Level1, payload.into_bytes()).expect("Publish failure");
    }
}