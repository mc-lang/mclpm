
GIT_LINK=""

all: build

build:
	cargo build

build.release:
	cargo build --release

install: build.release
	mkdir -p ~/.mclpm
	pushd ~/.mclpm
	cargo clone $(GIT_LINK)
