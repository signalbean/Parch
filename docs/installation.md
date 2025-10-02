# Installation Guide

How to get **Parch** running on your system.

## 📦 Install Options

### 1. From Releases (Recommended)

Grab the latest binary from [Releases](https://github.com/signalbean/Parch/releases/latest):

* **Windows** → `parch.exe`
* **Linux** → `parch`

#### Add to PATH

**Windows (PowerShell as Admin):**

```powershell
move parch.exe C:\Tools\parch.exe
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Tools", "User")
```

**Linux:**

```bash
chmod +x parch
sudo mv parch /usr/local/bin/

# Or user-only
mkdir -p ~/.local/bin
mv parch ~/.local/bin/
# Add ~/.local/bin to PATH in ~/.bashrc or ~/.zshrc
```

### 2. From Source

```bash
# Install Rust if needed
curl -sSf https://sh.rustup.rs | sh

git clone https://github.com/signalbean/Parch.git
cd Parch
cargo build --release

# Binary: target/release/parch → copy to PATH
```

### 3. From Crates.io

```bash
cargo install parch
```

Installs to `~/.cargo/bin/` (usually auto-added to PATH).

---

## 🖥️ Platform Requirements

* **Windows 10/11** → no extra deps, uses native APIs
* **Linux KDE** → works out of the box (`qdbus` preinstalled)
* **Linux GNOME** → works out of the box (`gsettings` preinstalled)
* **Tiling WMs (i3, bspwm, etc.)** → install `feh`

```bash
# Debian/Ubuntu
sudo apt install feh
# Arch
sudo pacman -S feh
# Fedora
sudo dnf install feh
```

---

## ✅ Verify

```bash
parch version
```

---

## 🔄 Update

* **Releases** → replace old binary
* **Crates.io** → `cargo install parch --force`
* **Source** → `git pull && cargo build --release`

---

## 🗑️ Uninstall

**Binary:** delete from PATH

```bash
sudo rm /usr/local/bin/parch     # Linux
del C:\Tools\parch.exe           # Windows
```

**Cargo:**

```bash
cargo uninstall parch
```

**Wallpapers (optional):**

* Windows → `%USERPROFILE%\Pictures\Parch\`
* Linux → `~/Pictures/Parch/`
