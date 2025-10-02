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

### Use Any Local Wallpaper

```bash
parch local
```

Randomly selects and applies a wallpaper from **any** of your local collections (both SFW and NSFW). No internet connection required. This is perfect for maximum variety!

### Use Local SFW Wallpaper

```bash
parch local sfw
```

Randomly selects and applies a wallpaper from your local SFW collection only. No internet connection required.

### Use Local NSFW Wallpaper

```bash
parch local nsfw
```

Randomly selects and applies a wallpaper from your local NSFW collection only. No internet connection required.

### Download Specific Post

```bash
parch id 123456
```

Downloads and sets a specific wallpaper by its Konachan post ID.

### Verbose Output

```bash
parch sfw -V
parch local verbose
parch local nsfw verbose
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

### Using Local Mode

The `local` command has three modes:

1. **`parch local`** - Random from **all** collections (maximum variety)
2. **`parch local sfw`** - Random from SFW only (safe for work)
3. **`parch local nsfw`** - Random from NSFW only (private use)

The `local` command is perfect for:
- **Offline use**: No internet connection needed
- **Quick switching**: Instantly change wallpapers from your collection
- **Rotation scripts**: Create randomized wallpaper rotations
- **Favorite collections**: Curate your own collection and cycle through it
- **Surprise mode**: Use `parch local` to get wallpapers from anywhere in your collection

**Example workflow:**
```bash
# Download wallpapers to both collections
parch sfw
parch sfw
parch nsfw
parch nsfw

# Use any wallpaper randomly
parch local

# Or be specific
parch local sfw
parch local nsfw
```

### Automation

**Linux (cron):**
```bash
# Add to crontab (crontab -e)
# Random wallpaper from any collection every hour
0 * * * * ~/.cargo/bin/parch local

# Mix of new downloads and local wallpapers
0 */2 * * * ~/.cargo/bin/parch sfw          # Download new every 2 hours
30 * * * * ~/.cargo/bin/parch local         # Use any local every hour at :30

# SFW only rotation
0 * * * * ~/.cargo/bin/parch local sfw

# New wallpaper every day at 9 AM
0 9 * * * /usr/local/bin/parch sfw
```

**Windows (Task Scheduler):**
1. Open Task Scheduler
2. Create Basic Task
3. Set trigger (e.g., daily, on login)
4. Action: Start a program
5. Program: `C:\Tools\parch.exe`
6. Arguments: `local` (for any collection) or `local sfw` (for SFW only)

### Integration with Window Managers

**i3wm example (~/.config/i3/config):**
```
bindsym $mod+w exec parch local
bindsym $mod+Shift+w exec parch sfw
bindsym $mod+Control+w exec parch local sfw
```

**Keybinding for quick wallpaper change:**
```bash
# Add to your shell config
alias wallpaper='parch local'
alias sfwpaper='parch local sfw'
alias newpaper='parch sfw'
alias wp='parch local'
```

### Finding Post IDs

If you found a wallpaper you like on Konachan:
1. Open the post page on konachan.com
2. Look at the URL: `https://konachan.com/post/show/123456`
3. The number at the end is the post ID
4. Use: `parch id 123456`

## 🐛 Troubleshooting

### No local wallpapers found

```bash
# Check if directory exists and has images
ls ~/Pictures/Parch/         # Linux
dir %USERPROFILE%\Pictures\Parch\  # Windows

# Download some wallpapers first
parch sfw
parch sfw
parch sfw

# Then try local mode
parch local
```

### Wallpaper not setting

**Linux:**
```bash
# Check which desktop environment is detected
parch local -V

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

# If offline, use local mode instead
parch local

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
- `local` mode with no category will randomly select from **both** SFW and NSFW collections
- Use `local sfw` if you want to ensure only safe-for-work wallpapers in shared environments

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

### Building a curated collection

```bash
# Download multiple wallpapers to different collections
for i in {1..10}; do parch sfw; sleep 2; done
for i in {1..5}; do parch nsfw; sleep 2; done

# Review them manually
feh ~/Pictures/Parch/  # Linux
explorer %USERPROFILE%\Pictures\Parch  # Windows

# Delete unwanted ones, keep favorites
# Then use local mode to cycle through your curated collection
parch local  # Random from all
parch local sfw  # Only SFW
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
