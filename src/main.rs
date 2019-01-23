mod annotated;
mod syllables;
mod twitter;
use self::twitter::{Token, TweetStream, TweetToken};
use self::annotated::{Part, PartIterator};
use futures::{Future, Stream};
use dotenv::dotenv;
use envy;
use serde_derive::Deserialize;
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
        .filter_map(|tweet| {
            if tweet.lang != "en" {
                return None;
            }

            #[derive(Debug)]
            struct SyllableState {
                had_5: bool,
                had_12: bool,
                had_17: bool,
                tot_syllables: usize,
                text_w_syllables: String,
            }

            impl SyllableState {
                fn new() -> Self {
                    SyllableState {
                        had_5: false,
                        had_12: false,
                        had_17: false,
                        tot_syllables: 0,
                        text_w_syllables: String::from(""),
                    }
                }
            }

            PartIterator::new(&tweet.text)
                .map(|part| (twitter::analyze_part(&part), part))
                .fold(Some(SyllableState::new()), |maybe_state, (token, part)| {
                    maybe_state.and_then(|mut state| {
                        let SyllableState {
                            ref mut had_5,
                            ref mut had_12,
                            ref mut had_17,
                            ref mut tot_syllables,
                            ref mut text_w_syllables
                        } = state;

                        let word = match part {
                            Part::Word(val) | Part::Whitespace(val) => val,
                        };

                        let maybe_normalized_word: Option<String> = match token {
                            TweetToken::RT => None, // Some("retweet".to_string()),
                            TweetToken::Mention => None, // Some("at ".to_owned() + &deunicode(&word[1..])),
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

                        let maybe_syllables: Option<usize> = maybe_normalized_word
                            .map(|normalized_word| normalized_word.replace(|c: char| c.is_ascii_punctuation(), ""))
                            .map(|normalized_word| normalized_word.split_whitespace().map(syllables::count).sum());

                        let text_part_w_syllables = match maybe_syllables {
                            Some(syllables) => format!("{} ({})", word, syllables),
                            None => word.to_string()
                        };

                        text_w_syllables.push_str(&text_part_w_syllables);

                        *tot_syllables += maybe_syllables.unwrap_or(0);

                        if *tot_syllables == 5 {
                            *had_5 = true;
                        }
                        if *tot_syllables == 12 {
                            *had_12 = true;
                        }
                        if *tot_syllables == 17 {
                            *had_17 = true;
                        }

                        Some(state)
                    })
                })
                .and_then(|state| {
                    let SyllableState { had_5, had_12, had_17, tot_syllables, text_w_syllables } = state;
                    if !had_5 || !had_12 || !had_17 || tot_syllables != 17 {
                        None
                    } else {
                        Some((tweet, text_w_syllables))
                    }
                })
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
