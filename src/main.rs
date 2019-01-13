#![feature(slice_index_methods)]

mod annotated;
mod twitter;
use self::twitter::{Token, TweetStream};
use futures::{Future, Stream};
use dotenv::dotenv;
use envy;
use serde_derive::Deserialize;
use wordsworth::syllable_counter;

fn main() {

    dotenv().ok();

    let twitter_prefix = envy::prefixed("TWITTER_");
    let twitter_config: TwitterConfig = twitter_prefix.from_env().unwrap();

    let token = Token::new(
        twitter_config.api_key,
        twitter_config.api_secret,
        twitter_config.access_token,
        twitter_config.access_secret
    );

    let stream_future = TweetStream::new(token)
        .filter(|tweet| {

            let mut tot_syllables = 0;
            let mut had_5 = false;
            let mut had_12 = false;
            let mut had_17 = false;

            if !tweet.text.is_ascii() { // wordsworth breaks when there are multi-byte characters
                return false;
            }

            for word in tweet.text.split_whitespace() {
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
        .map(|tweet| {

            let annotated_parts = annotated::annotate(
                annotated::to_parts(&tweet.text),
                |part| tweet.text[..][part].to_owned() //twitter::analyze_part(&part)
            );

            (tweet, annotated_parts)
        })
        .for_each(|x| {
            println!("{:#?}", x);
            Ok(())
        })
        .map_err(|e| println!("error: {}", e));

    twitter::run(stream_future);
}

#[derive(Deserialize, Debug)]
struct TwitterConfig {
    api_key: String,
    api_secret: String,
    access_token: String,
    access_secret: String,
}
