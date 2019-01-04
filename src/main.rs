
use dotenv::dotenv;
use envy;
use serde_derive::Deserialize;
use twitter_stream::{Token as TwitterToken, TwitterStreamBuilder};
use twitter_stream::rt::{Future, Stream};
use serde_json;
use std::fmt;
use wordsworth::syllable_counter;

fn main() {

    dotenv().ok();

    let twitter_prefix = envy::prefixed("TWITTER_");
    let twitter_config: TwitterConfig = twitter_prefix.from_env().unwrap();

    let token = TwitterToken::new(
        twitter_config.api_key,
        twitter_config.api_secret,
        twitter_config.access_token,
        twitter_config.access_secret
    );

    let stream_future = TwitterStreamBuilder::sample(token)
        .listen()
        .unwrap()
        .flatten_stream()
        .filter_map(|json| {
            match serde_json::from_str::<Tweet>(&json) {
                Ok(val) => Some(val),
                Err(_) => None, // May have seen a status deletion { delete: { status } }
            }
        })
        .filter(|tweet| {

            let mut tot_syllables = 0;
            let mut had_5 = false;
            let mut had_12 = false;
            let mut had_17 = false;

            if !tweet.text.is_ascii() { // wordsworth breaks when there are multi-byte characters
                return false;
            }

            for word in tweet.text.as_str().split_whitespace() {
                tot_syllables += syllable_counter(&word);
                if tot_syllables == 5 {
                    had_5 = true;
                }
                if tot_syllables == 12 {
                    had_12 = true;
                }
                if tot_syllables == 17 {
                    had_17 = true;
                }
            }

            had_5 && had_12 && had_17 && tot_syllables == 17
        })
        .for_each(|tweet| {
            debug(&tweet);
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    twitter_stream::rt::run(stream_future);
}

fn debug(value: impl fmt::Debug) -> () {
    println!("{:#?}", value);
}

#[derive(Deserialize, Debug)]
struct TwitterConfig {
    api_key: String,
    api_secret: String,
    access_token: String,
    access_secret: String,
}

#[derive(Deserialize, Debug)]
struct Tweet {
    #[serde(rename = "id_str")]
    id: String,
    text: String
}