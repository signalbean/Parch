PKG_NAME := parch
RELEASE_DIR := target/release
RELEASE_BIN := $(RELEASE_DIR)/$(PKG_NAME)

ifeq ($(OS),Windows_NT)
    RELEASE_BIN := $(RELEASE_BIN).exe
endif

all: build

build:
	@echo "Building $(PKG_NAME) in debug mode..."
	cargo build

release:
	@echo "Building $(PKG_NAME) in release mode..."
	cargo build --release

release-upx: release
	@echo "Compressing $(RELEASE_BIN) with UPX..."
	upx --best --lzma $(RELEASE_BIN)

install:
	cargo install --path .

clean:
	cargo clean

.PHONY: all build release release-upx clean run-sfw run-nsfw run-local-sfw run-local-nsfw run-id install
