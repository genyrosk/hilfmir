<p align="left">
    <a href="https://github.com/genyrosk/hilfmir/actions">
        <img src="https://github.com/genyrosk/hilfmir/actions/workflows/rust.yml/badge.svg">
    </a>
    <a href="https://github.com/genyrosk/hilfmir/releases/">
        <img src="https://img.shields.io/github/release/genyrosk/hilfmir.svg">
    </a>
    <a href="https://www.rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-1.65.0-orange">
    </a>
    <img src="https://img.shields.io/badge/Telegram-2CA5E0?style=flat&logo=telegram&logoColor=white">
</p>

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

# Webhook

To configure a webhook that Telegram can send push notifications, set the following environment variables:

```sh
WEBHOOK_MODE=true
DOMAIN_HOST=your.domain
```

# TODO:

Add Github Action to push to Dockerhub when releasing.
