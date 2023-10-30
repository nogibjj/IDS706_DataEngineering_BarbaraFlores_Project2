format:
	#cargo fmt --quiet

release:
	cargo build --release

lint:
	cargo clippy

test:
	cargo test --quiet

run:
	#cargo run 

all: format lint test release run
