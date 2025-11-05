<div align="center">

# Parch

**A cmd line tool for fetching and applying wallpapers from Konachan.**  
*Built with Rust.*

</div>

## Usage

```bash
# Fetch random SFW wallpaper
parch sfw

# Fetch random NSFW wallpaper  
parch nsfw

# Fetch specific wallpaper by ID
parch id 12345

# Use local wallpapers (offline mode)
parch local sfw
parch local nsfw

# Verbose output
parch sfw verbose
```

## Installation

**From releases:** Download the latest binary from [releases](https://github.com/signalbean/Parch/releases/latest) and add to PATH.

**From source:**
```bash
cargo install parch
```

## Features

- Cross-platform (Windows 10/11, Linux)
- Online fetching from Konachan
- Local wallpaper management
- SFW/NSFW content filtering

## License

MIT - see [LICENSE](LICENSE).

**Note:** Content sourced from Konachan. Use rating flags responsibly.
