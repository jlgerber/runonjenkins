build:
	cargo build --release
install:
	cp target/release/pkg-build-remote /dd/dept/software/bin/cent6_64/.

all: build install