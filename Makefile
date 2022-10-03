
GIT_LINK=https://github.com/mc-lang/mclpm.git

all: build

build:
	cargo build

build.release:
	cargo build --release

install: build.release
	sudo cp ./target/release/mclpm /usr/bin/mclpm

