use serde_json;
use serde_derive::Deserialize;
use twitter_stream::{TwitterStream, FutureTwitterStream, TwitterStreamBuilder};
use futures::{Poll, Future, Stream};
use futures::{future::FlattenStream, stream::FilterMap};
use string;
use bytes::Bytes;

pub use twitter_stream::Token;

pub struct TweetStream {
    stream: FilterMap<FlattenStream<FutureTwitterStream>, fn(string::String<Bytes>) -> Option<Tweet>>
}

impl TweetStream {
    pub fn new(token: Token) -> Self {
        TweetStream {
            stream: TwitterStreamBuilder::sample(token)
                .listen()
                .unwrap()
                .flatten_stream()
                .filter_map(|json| {
                    match serde_json::from_str::<Tweet>(&json) {
                        Ok(val) => Some(val),
                        Err(_) => None, // May have seen a status deletion { delete: { status } }
                    }
                })
        }
    }
}

impl Stream for TweetStream {
    type Item = Tweet;
    type Error = <TwitterStream as Stream>::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.stream.poll()
    }
}

#[derive(Deserialize, Debug)]
pub struct Tweet {
    #[serde(rename = "id_str")]
    pub id: String,
    pub text: String
}
