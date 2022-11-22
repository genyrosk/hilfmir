build:
  cargo build

build-release:
  cargo build --release

run:
  RUST_BACKTRACE=1 cargo run

run-w *FLAGS:
  fd .rs | entr -r cargo run {{FLAGS}}

test-w *FLAGS:
  fd .rs | entr -r cargo test {{FLAGS}}

install:
  cargo install --path .

docker-build:
  docker build -t hilfmir .

docker-build-with-chef:
  docker build -t hilfmir-with-chef -f ./Dockerfile-with-chef .

docker-run:
  docker run --rm --env-file .env --name hilfmir hilfmir
