# Parch

A fast CLI for fetching and applying wallpapers from Konachan.
Built with Rust → lightweight, cross-platform, zero-nonsense.

## ✨ Features

* **Cross-platform**: Windows 10/11 & Linux
* **Snappy & lightweight**: Minimal overhead, instant execution
* **Smart wallpaper setting**: Auto-detects the best method
* **Local mode**: Offline wallpapers from your `Parch` folder
* **Rating aware**: SFW & NSFW handled separately
* **Minimal deps**: Just what’s needed-no bloat

## 🚀 Quick Start

```bash
# Random SFW wallpaper
parch sfw

# Random NSFW wallpaper
parch nsfw

# Local random (mixed)
parch local

# Local random (SFW only)
parch local sfw

# Fetch by ID
parch id 123456
```

## 📚 Docs

* [Install](docs/installation.md)
* [Usage](docs/usage.md)
* [Platform Support](docs/platform-support.md)
* [Contributing](CONTRIBUTING.md)

## 📦 Install

**Releases (recommended):**
Download the latest binary [here](https://github.com/signalbean/Parch/releases/latest) and add it to your PATH.

**Cargo:**

```bash
cargo install parch
```

## 🖥️ Supported Platforms

* ✅ Windows 10/11
* ✅ Linux (KDE, GNOME)
* ✅ Linux tiling WMs (i3, bspwm, etc. via `feh`)

See [Platform Support](docs/platform-support.md) for more.

## 🤝 Contributing

PRs welcome! Check [CONTRIBUTING.md](CONTRIBUTING.md).

## 📄 License

MIT - see [LICENSE](LICENSE).

## ⚠️ Content Notice

Parch pulls from Konachan. Ratings matter-use flags responsibly.

---

**Made with ❤️ in Rust**
