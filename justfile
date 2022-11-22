build:
  cargo build

build-release:
  cargo build --release

lint:
  cargo clippy

format:
  cargo fmt

run:
  RUST_BACKTRACE=1 cargo run

install:
  cargo install --path .

docker-build:
  docker build -t hilfmir .

docker-build-with-chef:
  docker build -t hilfmir-with-chef -f ./Dockerfile-with-chef .

docker-run:
  docker run --rm --env-file .env --name hilfmir hilfmir
