# Contributing to Parch

Thanks for helping make **Parch** better! Here’s everything you need to get started.

## 🏗️ Project Structure

```
Parch/
├── src/
│   ├── main.rs       # Entry point
│   ├── cli.rs        # CLI parsing
│   ├── api.rs        # Konachan API
│   ├── download.rs   # Image handling
│   ├── local.rs      # Local wallpapers
│   └── wallpaper.rs  # Wallpaper setting
├── .github/          # CI/CD
├── Cargo.toml        # Config
└── README.md
```

## 🔧 Setup

### Requirements

* Rust (stable)
* Git
* Platform deps (see [Platform Support](docs/platform-support.md))

### Build & Test

```bash
git clone https://github.com/signalbean/Parch.git
cd Parch

# Dev build
cargo build
cargo run -- sfw

# Release build
cargo build --release

# Tests
cargo test

# Debug run
RUST_LOG=debug cargo run -- sfw -V
```

## 📋 Guidelines

* Format: `cargo fmt`
* Lint: `cargo clippy` (no warnings)
* Commit: clear & descriptive
* PRs: fork → branch → code → test → push → open PR

### Good PRs include

* Bug fixes 🐞
* Perf boosts ⚡
* New DE/OS support 🖥️
* Better docs/tests 📝
* Feature upgrades 🚀

## ⚡ Performance Focus

* Keep binary small
* Avoid extra deps
* Optimize startup & IO
* Be memory-smart

## 🖥️ Platform Notes

### Linux DE support

1. Add detection in `wallpaper.rs`
2. Implement set method
3. Add fallback
4. Update docs

### Windows

* Use native APIs
* Support Win 10 + 11

### Local Mode

Ideas welcome: better RNG, caching, filtering, history, etc.

## 📄 License

All contributions are MIT-licensed.

## 🤝 Questions?

Open an issue anytime!
