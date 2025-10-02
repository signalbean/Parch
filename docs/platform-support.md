# Platform Support

Detailed information about platform compatibility and implementation details.

## 🖥️ Supported Platforms

### Windows (10, 11)

**Status**: ✅ Fully Supported

**Implementation**:
- Uses native Windows `SystemParametersInfo` API
- Direct integration with Windows Desktop Window Manager
- No external dependencies required
- Instant wallpaper changes

**Features**:
- Respects Windows display settings
- Compatible with all Windows themes
- Automatic wallpaper style handling

**Requirements**:
- Windows 10 or Windows 11
- No additional software needed

### Linux

**Status**: ✅ Fully Supported

Parch supports multiple Linux desktop environments through automatic detection and fallback mechanisms.

#### KDE Plasma

**Detection**: Checks for KDE-specific environment variables and processes

**Implementation**:
- Tries `qdbus6` (Qt6 version)
- Falls back to `qdbus-qt5` (Qt5 version)
- Falls back to `qdbus` (legacy version)
- Uses D-Bus to communicate with KDE Plasma shell

**Requirements**:
- KDE Plasma 5.x or 6.x
- `qdbus`, `qdbus-qt5`, or `qdbus6` (usually pre-installed)

**Command used**:
```bash
qdbus6 org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "..."
```

#### GNOME / Ubuntu

**Detection**: Checks for GNOME-specific environment variables

**Implementation**:
- Uses `gsettings` command-line tool
- Modifies GNOME desktop background settings via dconf

**Requirements**:
- GNOME 3.x or later
- `gsettings` (usually pre-installed)

**Command used**:
```bash
gsettings set org.gnome.desktop.background picture-uri "file://$path"
```

#### Generic X11/Wayland (Window Managers)

**Desktop Environments/Window Managers**:
- i3
- bspwm
- dwm
- awesome
- openbox
- And any other X11/Wayland WM

**Implementation**:
- Uses `feh` as the wallpaper setter
- Most flexible and widely compatible option

**Requirements**:
- `feh` must be installed

**Installation**:
```bash
# Debian/Ubuntu
sudo apt install feh

# Arch Linux
sudo pacman -S feh

# Fedora
sudo dnf install feh

# openSUSE
sudo zypper install feh
```

**Command used**:
```bash
feh --bg-scale "$path"
```

### Detection Order

Parch tries methods in this order for best results:

1. **KDE Plasma**: Checks for `KDE_FULL_SESSION` or `DESKTOP_SESSION=plasma`
2. **GNOME**: Checks for `GNOME_DESKTOP_SESSION_ID` or `XDG_CURRENT_DESKTOP=GNOME`
3. **Generic/Fallback**: Uses `feh` for all other cases

## ⚡ Performance Characteristics

### Binary Size

- **Optimized**: ~2-3 MB (varies by platform)
- **LTO enabled**: Link-Time Optimization for smaller binary
- **Stripped**: Debug symbols removed in release builds

### Memory Usage

- **Idle**: Minimal (process exits after execution)
- **During operation**: ~5-10 MB depending on image size
- **Efficient**: Images streamed directly to disk

### Startup Time

- **Typical**: < 100ms on modern hardware
- **Optimized**: Minimal allocations during initialization
- **Fast CLI parsing**: Optimized argument handling

### Network Performance

- **Concurrent**: Efficient async HTTP requests
- **Minimal overhead**: Streamlined reqwest configuration
- **Direct download**: No intermediate buffering

## 🔧 Build Configuration

### Release Optimizations

```toml
[profile.release]
opt-level = "3"        # Optimize for size
lto = true             # Link-Time Optimization
strip = true           # Strip debug symbols
codegen-units = 1      # Better optimization
panic = "abort"        # Smaller binary size
```

### Dependency Strategy

**Included**:
- `ureq`: HTTP client (minimal features)
- `serde`: JSON parsing
- `winapi`: Windows api support

**Excluded** (compared to alternatives):
- No GUI frameworks
- No heavy image processing
- No unnecessary HTTP features
- No bundled CA certificates (uses system)

## 🐛 Platform-Specific Issues

### Windows

**Issue**: Wallpaper doesn't update immediately
- **Solution**: Windows may cache wallpapers. Restart Explorer or wait a moment.

**Issue**: Permission errors writing to Pictures
- **Solution**: Ensure your user has write access to `%USERPROFILE%\Pictures\`

### Linux (KDE)

**Issue**: `qdbus6` not found
- **Solution**: Install `qt6-tools` or fall back to `qdbus-qt5`

### Linux (GNOME)

**Issue**: `gsettings` permission denied
- **Solution**: Ensure you're running in a user session with D-Bus access

### Linux (Generic)

**Issue**: `feh` not found
- **Solution**: Install feh: `sudo apt install feh`

## 🚀 Future Platform Support

Potential future additions:

- **macOS**: Using `osascript` or native APIs
- **BSD variants**: FreeBSD, OpenBSD support
- **Additional DEs**: Cinnamon, XFCE, MATE, LXQt
- **Mobile**: Termux support for Android

Contributions welcome!
