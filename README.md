# Parch

A fast, lightweight CLI tool to fetch and apply wallpapers from Konachan. Built with Rust for optimal performance and cross-platform compatibility.

## ✨ Features

- **Cross-platform**: Native support for Windows 10/11 and Linux
- **Fast & Lightweight**: Optimized for minimal resource usage and quick execution
- **Smart Wallpaper Setting**: Automatically detects and uses the best available method
- **Local Mode**: Use random wallpapers from your downloaded collection (offline support)
- **Safe & NSFW Content**: Separate handling and storage for different content ratings
- **Minimal Dependencies**: Streamlined codebase with only essential dependencies

## 🚀 Quick Start

```bash
# Fetch and apply random SFW wallpaper
parch sfw

# Fetch and apply random NSFW wallpaper
parch nsfw

# Use random wallpaper from any local collection (no internet required)
parch local

# Use random local SFW wallpaper
parch local sfw

# Download specific post by ID
parch id 123456
```

## 📚 Documentation

- **[Installation Guide](docs/installation.md)** - Detailed installation instructions for all platforms
- **[Usage Guide](docs/usage.md)** - Complete command reference and tips
- **[Platform Support](docs/platform-support.md)** - Platform-specific details and troubleshooting
- **[Contributing](CONTRIBUTING.md)** - Guidelines for contributors

## 📦 Installation

### Quick Install

**From Releases** (Recommended):
- Download the latest binary for your platform from the [releases page](https://github.com/signalbean/Parch/releases/latest)
- Add to PATH for easy access

**From Crates.io**:
```bash
cargo install parch
```

See the [Installation Guide](docs/installation.md) for detailed instructions.

## 🖥️ Platform Support

- ✅ **Windows 10/11** - Native API support
- ✅ **Linux (KDE Plasma)** - Full support
- ✅ **Linux (GNOME)** - Full support  
- ✅ **Linux (i3, bspwm, etc.)** - Via `feh`

See [Platform Support](docs/platform-support.md) for details.

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting PRs.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## ⚠️ Content Notice

This tool fetches content from Konachan. Please be mindful of content ratings and use appropriate flags for your environment.

---

**Made with ❤️ and Rust**
