use std::env;

use serenity::Client;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};
use serenity::utils::MessageBuilder;

const VOWELS: [char; 8] = ['a', 'e', 'i', 'o', 'u', 'æ', 'ø', 'å'];

fn to_case_of(target_case: char, subject: char) -> char {
    if target_case.is_uppercase() {
        subject.to_uppercase().next().unwrap()
    } else if target_case.is_lowercase() {
        subject.to_lowercase().next().unwrap()
    } else {
        subject
    }
}


fn molde(s: &str) -> String {
    s.chars()
        .map(|c|
            if VOWELS.contains(&c.to_lowercase().next().unwrap()) {
                to_case_of(c, 'i')
            } else {
                c
            }
        )
        .collect()
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, message: Message) {
        let mut split = message.content.splitn(2, ' ');
        if let Some("!molde") = split.next() {
            if let Some(text) = split.next() {
                let molde_text = molde(text);
                let response = MessageBuilder::new()
                    .push(molde_text)
                    .build();

                if let Err(why) = message.channel_id.say(&context.http, &response) {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
}

fn main() {
    let token = env::var("SALT_TOKEN").expect("Missing Discord token");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
