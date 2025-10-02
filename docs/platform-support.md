# Platform Support

## 🖥️ Supported Platforms

### Windows 10/11 ✅

* Uses native `SystemParametersInfo` API
* No extra deps → instant wallpaper changes
* Works with all themes & styles

---

### Linux ✅

Parch auto-detects DE/WM and falls back gracefully.

#### KDE Plasma

* Detects session vars → uses `qdbus6`, fallback to `qdbus-qt5` / `qdbus`
* Requirements: Plasma 5/6 + `qdbus` package (usually preinstalled)

```bash
qdbus6 org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "..."
```

#### GNOME / Ubuntu

* Detects GNOME session → uses `gsettings`
* Requirements: GNOME 3+ + `gsettings`

```bash
gsettings set org.gnome.desktop.background picture-uri "file://$path"
```

#### Generic WMs (i3, bspwm, dwm, awesome, openbox, etc.)

* Uses `feh` for wallpaper setting
* Requirement: `feh`

```bash
feh --bg-scale "$path"
```

**Install feh:**

```bash
# Debian/Ubuntu
sudo apt install feh
# Arch
sudo pacman -S feh
# Fedora
sudo dnf install feh
# openSUSE
sudo zypper install feh
```

---

### Detection Order

1. KDE (Plasma session vars)
2. GNOME (GNOME session vars)
3. Generic fallback → `feh`

---

## ⚡ Performance

* **Binary**: ~2–3 MB (LTO + stripped)
* **Memory**: ~5–10 MB while running, exits after use
* **Startup**: <100ms typical
* **Network**: async HTTP, streamed direct-to-disk

---

## 🔧 Build Profile

```toml
[profile.release]
opt-level = "3"
lto = true
strip = true
codegen-units = 1
panic = "abort"
```

---

## 🐛 Known Issues

* **Windows**: wallpaper cache → may need Explorer restart
* **Linux KDE**: missing `qdbus6` → install `qt6-tools` or fallback to `qdbus-qt5`
* **Linux GNOME**: D-Bus permissions → ensure user session
* **Generic Linux**: `feh` missing → install manually

---

## 🚀 Future Support

* macOS (via `osascript` or native APIs)
* BSD (FreeBSD/OpenBSD)
* More DEs: Cinnamon, XFCE, MATE, LXQt
* Android (Termux experiments)
