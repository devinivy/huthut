
use dotenv::dotenv;
use envy;
use serde_derive::Deserialize;

fn main() {

    dotenv().ok();

    let twitter_prefix = envy::prefixed("TWITTER_");
    let twitter_config = twitter_prefix.from_env::<TwitterConfig>().unwrap();

    println!("{:#?}", twitter_config);
}

#[derive(Deserialize, Debug)]
struct TwitterConfig {
    api_key: String,
    api_secret: String,
    access_token: String,
    access_secret: String
}
