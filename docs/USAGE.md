# Usage Guide

Complete guide to using Parch for fetching and setting wallpapers.

## 📖 Basic Commands

### Fetch SFW Wallpaper

```bash
parch sfw
```

Fetches a random safe-for-work wallpaper from Konachan and sets it as your desktop background.

### Fetch NSFW Wallpaper

```bash
parch nsfw
```

Fetches a random NSFW wallpaper from Konachan. These are stored in a separate folder.

### Download Specific Post

```bash
parch id 123456
```

Downloads and sets a specific wallpaper by its Konachan post ID.

### Verbose Output

```bash
parch sfw -V
parch nsfw verbose
parch id 123456 -V
```

Enable detailed logging to see what Parch is doing. Useful for debugging.

## 🆘 Help & Information

### Show Help

```bash
parch help
parch -h
parch help

# Get help for specific command
parch help sfw
parch sfw help
```

### Show Version

```bash
parch version
parch -v
parch version
```

## 📁 Storage Locations

Wallpapers are automatically organized and saved to platform-specific directories:

### Windows
```
%USERPROFILE%\Pictures\Parch\          # SFW wallpapers
%USERPROFILE%\Pictures\Parch\Nsfw\     # NSFW wallpapers
```

Example: `C:\Users\YourName\Pictures\Parch\123456.jpg`

### Linux
```
~/Pictures/Parch/          # SFW wallpapers
~/Pictures/Parch/Nsfw/     # NSFW wallpapers
```

Example: `/home/username/Pictures/Parch/123456.jpg`

### File Naming

Files are named using their Konachan post ID:
- `123456.jpg` - JPEG images
- `789012.png` - PNG images

This makes it easy to:
- Find the original post on Konachan
- Avoid duplicate downloads
- Manage your wallpaper collection

## 💡 Tips & Tricks

### Automation

**Linux (cron):**
```bash
# Add to crontab (crontab -e)
# New wallpaper every hour
0 * * * * ~/~.cargo/bin/parch sfw

# New wallpaper every day at 9 AM
0 9 * * * ~/~usr/local/bin/parch sfw
```

**Windows (Task Scheduler):**
1. Open Task Scheduler
2. Create Basic Task
3. Set trigger (e.g., daily, on login)
4. Action: Start a program
5. Program: `C:\Tools\parch.exe`
6. Arguments: `sfw`

### Integration with Window Managers

**i3wm example (~/.config/i3/config):**
```
bindsym $mod+w exec parch sfw
```

**Keybinding for quick wallpaper change:**
```bash
# Add to your shell config
alias wallpaper='parch sfw'
alias wp='parch sfw'
```

### Finding Post IDs

If you found a wallpaper you like on Konachan:
1. Open the post page on konachan.com
2. Look at the URL: `https://konachan.com/post/show/123456`
3. The number at the end is the post ID
4. Use: `parch id 123456`

## 🐛 Troubleshooting

### Wallpaper not setting

**Linux:**
```bash
# Check which desktop environment is detected
parch sfw -V

# Try installing feh as fallback
sudo apt install feh  # Debian/Ubuntu
```

**Windows:**
- Make sure you're running Windows 10 or 11
- Check if the file was downloaded to Pictures\Parch\

### Network issues

```bash
# Use verbose mode to see detailed error
parch sfw -V

# Check your internet connection
# Verify Konachan is accessible
```

### Permission errors

**Linux:**
```bash
# Ensure Pictures directory is writable
ls -ld ~/Pictures/Parch/

# Create directory manually if needed
mkdir -p ~/Pictures/Parch/
```

## ⚠️ Content Ratings

**Important**: This tool fetches content from Konachan. Be mindful of content ratings:

- Use `sfw` for safe-for-work environments
- Use `nsfw` only in appropriate private settings
- NSFW content is automatically stored in a separate folder
- Review the images before using in shared/work environments

## 🔄 Managing Your Collection

### Viewing your wallpapers

**Windows:**
```powershell
# Open folder in Explorer
explorer %USERPROFILE%\Pictures\Parch
```

**Linux:**
```bash
# List all downloaded wallpapers
ls -lh ~/Pictures/Parch/

# View with image viewer
feh ~/Pictures/Parch/
```

### Cleaning up

```bash
# Delete all SFW wallpapers
rm -rf ~/Pictures/Parch/*.jpg    # Linux
del %USERPROFILE%\Pictures\Parch\*.jpg  # Windows

# Delete all NSFW wallpapers
rm -rf ~/Pictures/Parch/Nsfw/    # Linux
rmdir /s %USERPROFILE%\Pictures\Parch\Nsfw  # Windows
```
