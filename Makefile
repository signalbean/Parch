PKG_NAME := parch
RELEASE_DIR := target/release
RELEASE_BIN := $(RELEASE_DIR)/$(PKG_NAME)

ifeq ($(OS),Windows_NT)
    RELEASE_BIN := $(RELEASE_BIN).exe
endif

all: build

build:
    cargo build

release:
    cargo build --release

release-upx: release
    upx --best --lzma $(RELEASE_BIN)

release-target:
    cargo build --release --target x86_64-unknown-linux-musl

run: build
    cargo run

run-release: release
    $(RELEASE_BIN)

test:
    cargo test

fmt-check:
    cargo fmt -- --check

fmt-apply:
    cargo fmt

clippy:
    cargo clippy -- -D warnings

install:
    cargo install --path .

clean:
    cargo clean

.PHONY: all build release release-upx release-target run run-release test fmt-check fmt-apply clippy clean install run-sfw run-nsfw run-local-sfw run-local-nsfw run-id
