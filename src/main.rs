use std::env;

use serenity::Client;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, EventHandler};
use serenity::utils::MessageBuilder;

use crate::gif::GifKeys;

mod gif;

const VOWELS: [char; 8] = ['a', 'e', 'i', 'o', 'u', 'æ', 'ø', 'å'];

const SALT_TAGS: [&str; 2] = [
    "salt",
    "salty"
];
const MIKRO_TAGS: [&str; 7] = [
    "whisky",
    "whiskey",
    "vodka",
    "moonshine",
    "beer",
    "brewery",
    "distillery"
];

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

struct Handler(GifKeys);

impl EventHandler for Handler {
    fn message(&self, context: Context, message: Message) {
        let mut response = None;
        if message.content == "!salt" {
            let result = gif::gif(&self.0, &SALT_TAGS);
            let message =
                match result {
                    Ok(gif_url) => gif_url,
                    Err(error) => {
                        eprintln!("{}", error);
                        "!?".to_string()
                    }
                };
            response = Some(MessageBuilder::new().push(message).build());
        } else {
            let mut split = message.content.splitn(2, ' ');
            if let Some("!molde") = split.next() {
                if let Some(text) = split.next() {
                    let molde_text = molde(text);
                    response = Some(MessageBuilder::new()
                        .push(molde_text)
                        .build());
                }
            }
        }

        if let Some(response) = response {
            if let Err(why) = message.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn main() {
    let token = env::var("SALT_TOKEN").expect("Missing Discord token");
    let tenor_key = env::var("TENOR_KEY").expect("Missing Tenor key");
    let giphy_key = env::var("GIPHY_KEY").expect("Missing Giphy key");
    let gif_keys = GifKeys {
        tenor: tenor_key,
        giphy: giphy_key,
    };
    let mut client = Client::new(&token, Handler(gif_keys)).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
