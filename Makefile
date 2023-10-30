build:
    cargo build

test:
    cargo test

run:
    cargo run

clean:
    cargo clean

format:
    cargo fmt

lint:
    cargo clippy

docker-build:
    docker build -t my-rust-app .

docker-run:
    docker run my-rust-app
