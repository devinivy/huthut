use serde_json;
use serde_derive::Deserialize;
use futures::{future::FlattenStream, stream::FilterMap, Poll, Future, Stream};
use twitter_stream::{TwitterStream, FutureTwitterStream, TwitterStreamBuilder};
pub use twitter_stream::{rt::run, Token};

type TwitterStreamItem = <TwitterStream as Stream>::Item;
type TwitterStreamError = <TwitterStream as Stream>::Error;

#[derive(Deserialize, Debug)]
pub struct Tweet {
    #[serde(rename = "id_str")]
    pub id: String,
    pub text: String
}

pub struct TweetStream {
    inner: FilterMap<FlattenStream<FutureTwitterStream>, fn(TwitterStreamItem) -> Option<Tweet>>
}

impl TweetStream {
    pub fn new(token: Token) -> Self {
        TweetStream {
            inner: TwitterStreamBuilder::sample(token)
                .listen()
                .unwrap()
                .flatten_stream()
                .filter_map(|json| serde_json::from_str::<Tweet>(&json).ok())
        }
    }
}

impl Stream for TweetStream {
    type Item = Tweet;
    type Error = TwitterStreamError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.inner.poll()
    }
}
