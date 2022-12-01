set positional-arguments

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

changelog:
  git cliff --unreleased

prepare tag:
  git cliff --unreleased --tag $1 --prepend CHANGELOG.md
  @echo
  @echo "Make sure the version in Cargo.toml is up to date:"
  @echo "--------------------------------------------------"
  @head -9 Cargo.toml | grep version
  @echo
  @echo "Release commit:"
  @echo "---------------"
  @echo "git commit -S -m \"chore(release): prepare for $1\""
