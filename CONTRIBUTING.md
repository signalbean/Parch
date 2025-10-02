# Contributing to Parch

Thank you for your interest in contributing to Parch! This document provides guidelines and information for contributors.

## 🏗️ Project Structure

```
Parch/
├── src/
│   ├── main.rs          # Entry point and orchestration
│   ├── cli.rs           # Command-line argument parsing
│   ├── api.rs           # Konachan API interaction
│   ├── download.rs      # Image downloading and storage
│   ├── local.rs         # Local wallpaper selection
│   └── wallpaper.rs     # Cross-platform wallpaper setting
├── .github/workflows/   # CI/CD and release automation
├── Cargo.toml           # Project configuration
└── README.md            # Project overview
```

## 🔧 Development Setup

### Prerequisites

- Rust toolchain (stable channel)
- Git
- Platform-specific dependencies (see Platform Support section)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/signalbean/Parch.git
cd Parch

# Build debug version for development
cargo build

# Build optimized release binary
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- sfw -V
```

### Testing Your Changes

```bash
# Run the development build
cargo run -- sfw

# Test different commands
cargo run -- nsfw
cargo run -- local sfw
cargo run -- id 123456
cargo run -- help
```

## 📋 Contribution Guidelines

### Code Style

- Follow Rust's official style guidelines
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Write descriptive commit messages

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly on your platform
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### What We're Looking For

- **Bug fixes**: Always welcome!
- **Performance improvements**: Especially for download or wallpaper setting
- **Platform support**: Additional desktop environments or operating systems
- **Documentation**: Improvements to docs, examples, or code comments
- **Tests**: Additional test coverage
- **Feature enhancements**: Like improved local mode capabilities

## ⚡ Performance Guidelines

When contributing code, keep these things in mind:

- **Minimal binary size**: Avoid unnecessary dependencies
- **Reduced dependencies**: Only essential crates for core functionality
- **Efficient networking**: Streamlined HTTP operations
- **Fast startup**: Optimize argument parsing and initialization
- **Memory efficiency**: Minimize allocations where possible
- **Quick file operations**: Efficient directory scanning for local mode

## 🖥️ Platform-Specific Contributions

### Adding Linux Desktop Environment Support

If you want to add support for a new DE:

1. Add detection logic in `wallpaper.rs`
2. Implement the wallpaper setting method
3. Add fallback behavior if the method fails
4. Update documentation

### Windows Enhancements

Contributions for Windows-specific features should:

- Use native Windows APIs where possible
- Maintain compatibility with Windows 10 and 11
- Test on both platforms if possible

### Local Mode Enhancements

Ideas for improving local mode:

- Better randomization algorithms
- Caching for faster repeated access
- Filtering options (by size, aspect ratio, etc.)
- History tracking to avoid recent repeats

## 📄 License

By contributing to Parch, you agree that your contributions will be licensed under the MIT License.

## 🤝 Questions?

Feel free to open an issue for any questions about contributing!
