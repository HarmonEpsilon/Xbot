#[macro_use] extern crate serenity;
extern crate dotenv;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate reqwest;

mod commands;

use serenity::{
    client::Client,
    framework::StandardFramework,
    model::{
        gateway::Ready,
    },
    prelude::*,
    http,
};

use dotenv::dotenv;
use std::env;
use std::collections::HashSet;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    dotenv().ok();

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
                        .expect("Error creating client");

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
            .configure(|c| c
                .owners(owners)
                .prefix("~"))
            .command("help", |c| c.cmd(commands::meta::help)));
    
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
