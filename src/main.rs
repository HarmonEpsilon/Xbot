#[macro_use] extern crate serenity;
extern crate dotenv;

use serenity::{
    client::Client,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::*,
};

use dotenv::dotenv;
use std::env;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "~help" {
            if let Err(why) = msg.channel_id.send_message(|m| m 
                .embed(|e| e
                    .title("Hi there! I'm Xbot.")
                    .description("Here's a helpful list of some common commands.")
                    .fields(vec![
                        ("General Commands", "~help\n"/*~status\n~money [USD] to [AUD]*/, true),
                        ("Description", "Brings up this dialog\n"/*Checks status of Xbox Live\nConverts one currency to another*/, true),
                    ])
                    .footer(|f| f
                        .text("Created by Kaironaut. Ver. 0.0.1")))) {
                            println!("Error sending message: {:?}", why);
                        }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    dotenv().ok();

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
                        .expect("Error creating client");
    
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
