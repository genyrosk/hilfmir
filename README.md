# Hilfmir: A Telegram language translation bot

Target languages currently supported:
- English
- German
- French
- Spanish
- Russian
- Korean

# Docker builds

Currently 2 docker builds are available: the "vanilla" multistage `Dockerfile` and the more advanced dependecies-caching `Dockerfile-with-chef`, which uses [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) to cache the dependencies and speed up incremental builds. 

In order to use the `Dockerfile-with-chef`, make sure you have the following in `Cargo.toml`:

```toml
[dependencies]
...
# Use vendored openssl. We don't depend on it directly.
openssl = { version = "0.10.41", features = ["vendored"], optional = true }

[features]
vendored-openssl = ["openssl"]

```


