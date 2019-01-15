# huthut
Find accidental haikus

## Running it

* Get a [Twitter developer account](https://developer.twitter.com/en/docs/basics/developer-portal/overview)
* Create a [Twitter app](https://developer.twitter.com/en/apps)
* Go to it's **Keys and tokens** tab and get all 4 things
* Add them to environment variables or create a `.env` file like so

```
cat > .env <<'EOF'
TWITTER_API_KEY=...
TWITTER_API_SECRET=...
TWITTER_ACCESS_TOKEN=...
TWITTER_ACCESS_SECRET=...
EOF
```

* Build and run:

```
cargo build
./target/debug/huthut
```

* Wait a little, and it'll print tweets.
