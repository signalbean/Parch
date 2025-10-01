# Parch

A fast, lightweight CLI tool to fetch and apply wallpapers from Konachan. Built with Rust for optimal performance and cross-platform compatibility.

## ✨ Features

- **Cross-platform**: Native support for Windows 10/11 and Linux
- **Fast & Lightweight**: Optimized for minimal resource usage and quick execution
- **Smart Wallpaper Setting**: Automatically detects and uses the best available method
- **Safe & NSFW Content**: Separate handling and storage for different content ratings
- **Minimal Dependencies**: Streamlined codebase with only essential dependencies

## 🚀 Installation

### From Releases (Recommended)

Download the latest binary for your platform from the [releases page](https://github.com/signalbean/Parch/releases/latest):

- **Windows**: `parch-windows-x64.exe` or `parch-windows-x64-msvc.exe`
- **Linux**: `parch-linux-x64`

### From Source

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install parch
cargo install parch
```

### From Crates.io

```bash
cargo install parch
```

## 📖 Usage

### Basic Commands

```bash
# Fetch and apply random SFW wallpaper
parch sfw

# Fetch and apply random NSFW wallpaper
parch nsfw

# Download specific post by ID
parch id 123456

# Enable verbose output for debugging
parch sfw -V
```

### Help & Information

```bash
# Show help
parch help
parch -h

# Show version
parch version
parch -v
```

## 📁 Storage Locations

Wallpapers are automatically organized and saved to:

### Windows
```
%USERPROFILE%\Pictures\Parch\          # SFW wallpapers
%USERPROFILE%\Pictures\Parch\Nsfw\     # NSFW wallpapers
```

### Linux
```
~/Pictures/Parch/          # SFW wallpapers
~/Pictures/Parch/Nsfw/     # NSFW wallpapers
```

Files are named using their Konachan post ID (e.g., `123456.jpg`)

## 🖥️ Platform Support

### Windows (10, 11)
- **Native API**: Uses Windows SystemParametersInfo API for instant wallpaper changes
- **No external dependencies required**

### Linux
- **KDE Plasma**: Automatic detection and use of `qdbus`, `qdbus-qt5`, or `qdbus6`
- **GNOME**: Uses `gsettings` for wallpaper management
- **Generic X11/Wayland**: Falls back to `feh` for window managers
- **Automatic detection**: Tries methods in order of preference

## ⚡ Performance Optimizations

- **Minimal binary size**: Optimized build configuration with LTO and strip
- **Reduced dependencies**: Only essential crates for core functionality
- **Efficient networking**: Streamlined HTTP client without unnecessary features
- **Fast argument parsing**: Optimized CLI parsing for quick startup
- **Memory efficient**: Minimal allocations and smart string handling

## 🏗️ Project Structure

```
Parch/
├── src/
│   ├── main.rs          # Entry point and orchestration
│   ├── cli.rs           # Command-line argument parsing
│   ├── api.rs           # Konachan API interaction
│   ├── download.rs      # Image downloading and storage
│   └── wallpaper.rs     # Cross-platform wallpaper setting
├── .github/workflows/   # CI/CD and release automation
├── Cargo.toml          # Project configuration
└── README.md           # This file
```

## 🔧 Building from Source

```bash
# Clone the repository
git clone https://github.com/signalbean/Parch.git
cd Parch

# Build optimized release binary
cargo build --release

# The binary will be available at target/release/parch
```

### Cross-compilation for Windows (from Linux)

```bash
# Install cross-compilation tools
sudo apt-get install gcc-mingw-w64-x86-64

# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Note**: This tool fetches content from Konachan. Please be mindful of the content ratings and use appropriate flags for your environment.
