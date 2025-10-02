# Usage Guide

How to use **Parch** to fetch and set wallpapers.

## 📖 Basic Commands

```bash
parch sfw          # Random safe wallpaper
parch nsfw         # Random NSFW wallpaper
parch local        # Random local (any, content warning)
parch local sfw    # Random local SFW
parch local nsfw   # Random local NSFW
parch id 123456    # Fetch by post ID
parch -V           # Verbose mode
parch help         # Show help
parch version      # Show version
```

---

## 📁 Storage

Wallpapers are auto-organized:

**Windows**

```
%USERPROFILE%\Pictures\Parch\
%USERPROFILE%\Pictures\Parch\Nsfw\
```

**Linux**

```
~/Pictures/Parch/
~/Pictures/Parch/Nsfw/
```

Files named by Konachan ID → avoids duplicates & easy lookups.

---

## 💡 Tips

### Local Mode

* `parch local` → from all collections (⚠️ mixed content)
* `parch local sfw` → safe only
* `parch local nsfw` → private only

Great for offline use, quick switching, or scripts.

### Automation

**Linux (cron):**

```bash
0 * * * * ~/.cargo/bin/parch local
30 * * * * ~/.cargo/bin/parch local
```

**Windows (Task Scheduler):**

* Program: `parch.exe`
* Args: `local` or `local sfw`

### Integrations

**i3wm config**

```
bindsym $mod+w exec parch local
```

**Aliases**

```bash
alias wp='parch local'
alias sfwp='parch local sfw'
alias newwp='parch sfw'
```

---

## 🐛 Troubleshooting

* **No local wallpapers** → download some first with `parch sfw`
* **Wallpaper not changing**

  * Linux: check DE detection (`-V`), install `feh`
  * Windows: check `Pictures\Parch\`
* **Network issues** → use `parch -V` or offline with `parch local`
* **Permission errors** → ensure `~/Pictures/Parch/` (Linux) or `%USERPROFILE%\Pictures\Parch\` (Win) is writable

---

## ⚠️ Content Ratings

* `sfw` = safe
* `nsfw` = private use only
* `local` = mixes both → use with caution
* NSFW always stored separately

---

## 🔄 Manage Collection

**View**

```bash
explorer %USERPROFILE%\Pictures\Parch      # Windows
feh ~/Pictures/Parch/                      # Linux
```

**Curate**

* Download a batch (`for i in {1..10}; do parch sfw; done`)
* Review → delete unwanted → cycle with `local`

**Clean Up**

```bash
rm -rf ~/Pictures/Parch/*              # Linux
del %USERPROFILE%\Pictures\Parch\*     # Windows
```
