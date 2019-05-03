build:
	cargo build --release
install:
	cp target/release/pkg-build-remote ~/bin/.

all: build install