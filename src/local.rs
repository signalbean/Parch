use std::fs::read_dir;
use std::path::PathBuf;

pub fn get_random(nsfw: bool, verbose: bool) -> Result<PathBuf, String> {
    let dir = dir_path(nsfw)?;

    if verbose {
        println!("→ Scanning directory: {}", dir.display());
    }

    if !dir.exists() {
        return Err(format!("Directory not found: {}", dir.display()));
    }

    let mut images = Vec::new();

    for entry in read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file()
            && let Some(ext) = path.extension()
        {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "bmp") {
                images.push(path);
            }
        }
    }

    if images.is_empty() {
        return Err(format!(
            "No wallpapers found in {}. Download some first with 'parch {}'",
            dir.display(),
            if nsfw { "nsfw" } else { "sfw" }
        ));
    }

    if verbose {
        println!("→ Found {} wallpaper(s)", images.len());
    }

    // Fast random selection using timestamp
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize)
        % images.len();

    let selected = images.swap_remove(idx);

    if verbose {
        println!("→ Selected: {}", selected.display());
    }

    Ok(selected)
}

fn dir_path(nsfw: bool) -> Result<PathBuf, String> {
    let mut path = if cfg!(windows) {
        let userprofile = std::env::var("USERPROFILE")
            .or_else(|_| {
                std::env::var("HOMEDRIVE").and_then(|drive| {
                    std::env::var("HOMEPATH").map(|path| format!("{}{}", drive, path))
                })
            })
            .map_err(|_| "User directory not found".to_string())?;
        PathBuf::from(userprofile)
    } else {
        let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
        PathBuf::from(home)
    };

    path.push("Pictures");
    path.push("Parch");
    if nsfw {
        path.push("Nsfw");
    }

    Ok(path)
}
