#![feature(slice_index_methods)]

mod annotated;
mod twitter;
use self::twitter::{Token, TweetStream, TweetToken};
use futures::{Future, Stream};
use dotenv::dotenv;
use envy;
use serde_derive::Deserialize;
use wordsworth::syllable_counter;
use deunicode::deunicode;

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
        .map(|tweet| {

            let annotated_parts = annotated::annotate(
                annotated::to_parts(&tweet.text),
                |part| twitter::analyze_part(&part, &tweet.text)
            );

            (tweet, annotated_parts)
        })
        .filter_map(|(tweet, part_tokens)| {

            let mut tot_syllables = 0;
            let mut had_5 = false;
            let mut had_12 = false;
            let mut had_17 = false;
            let mut tweet_text_w_syllables = String::from("");

            for (part, token) in part_tokens {
                let word = &tweet.text[..][&part];

                let maybe_normalized_word: Option<String> = match token {
                    TweetToken::RT => Some("retweet".to_string()),
                    TweetToken::Mention => Some("at ".to_owned() + &deunicode(&word[1..])),
                    TweetToken::Hashtag => Some("hashtag ".to_owned() + &deunicode(&word[1..])),
                    TweetToken::Word => Some(deunicode(word)),
                    TweetToken::Link => None,
                    TweetToken::Whitespace => {
                        match word.matches("\n").count() {
                            0 => None,
                            1 => match tot_syllables {
                                0 | 5 | 12 | 17 => None,
                                _ => return None,
                            },
                            _ => match tot_syllables {
                                0 | 17 => None,
                                _ => return None,
                            },
                        }
                    },
                };

                let maybe_syllables: Option<u32> = maybe_normalized_word
                    .map(|normalized_word| normalized_word.replace(|c: char| c.is_ascii_punctuation(), ""))
                    .map(|normalized_word| normalized_word.split_whitespace().map(syllable_counter).sum());

                let text_part_w_syllables = match maybe_syllables {
                    Some(syllables) => format!("{} ({})", word, syllables),
                    None => word.to_string()
                };

                tweet_text_w_syllables.push_str(&text_part_w_syllables);

                tot_syllables += maybe_syllables.unwrap_or(0);

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

            if !had_5 || !had_12 || !had_17 || tot_syllables != 17 {
                return None;
            }

            Some((tweet, tweet_text_w_syllables))
        })
        .for_each(|(tweet, tweet_text_w_syllables)| {
            println!("{:#?}", (tweet.id, tweet_text_w_syllables));
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
