# Installation Guide

This guide covers all methods to install Parch on your system.

## 📦 Installation Methods

### From Releases (Recommended)

Download the latest binary for your platform from the [releases page](https://github.com/signalbean/Parch/releases/latest):

- **Windows**: `parch.exe`
- **Linux**: `parch`

#### Adding to PATH

**Windows:**
```powershell
# Move to a permanent location
move parch.exe C:\Tools\parch.exe

# Add to PATH (PowerShell as Administrator)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Tools", "User")
```

**Linux:**
```bash
# Make executable
chmod +x parch

# Move to a directory in PATH
sudo mv parch /usr/local/bin/

# Or for user-only installation
mkdir -p ~/.local/bin
mv parch ~/.local/bin/
# Add ~/.local/bin to PATH in ~/.bashrc or ~/.zshrc if needed
```

### From Source

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/signalbean/Parch.git
cd Parch
cargo build --release

# The binary will be at target/release/parch
# Copy it to your PATH as shown above
```

### From Crates.io

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install parch
cargo install parch
```

This automatically installs to `~/.cargo/bin/` which should be in your PATH after Rust installation.

## 🖥️ Platform-Specific Requirements

### Windows (10, 11)

No additional dependencies required. Parch uses native Windows APIs.

### Linux

Parch will work with most desktop environments, but some may require additional tools:

**KDE Plasma:**
- Usually works out of the box
- Uses `qdbus`, `qdbus-qt5`, or `qdbus6` (typically pre-installed)

**GNOME:**
- Usually works out of the box
- Uses `gsettings` (typically pre-installed)

**Other Window Managers (i3, bspwm, etc.):**
```bash
# Install feh as fallback
# Debian/Ubuntu
sudo apt install feh

# Arch Linux
sudo pacman -S feh

# Fedora
sudo dnf install feh
```

## ✅ Verifying Installation

After installation, verify Parch is working:

```bash
# Check version
parch version

# Try fetching a wallpaper
parch sfw
```

## 🔄 Updating

**From Releases:**
- Download the latest release and replace the old binary

**From Crates.io:**
```bash
cargo install parch --force
```

**From Source:**
```bash
cd Parch
git pull
cargo build --release
```

## 🗑️ Uninstallation

**Binary Installation:**
```bash
# Simply remove the binary from your PATH
# Linux
sudo rm /usr/local/bin/parch

# Windows
del C:\Tools\parch.exe
```

**Crates.io Installation:**
```bash
cargo uninstall parch
```

**Clean up downloaded wallpapers (optional):**
- Windows: Delete `%USERPROFILE%\Pictures\Parch\`
- Linux: Delete `~/Pictures/Parch/`
