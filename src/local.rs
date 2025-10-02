use std::fs::read_dir;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_random(nsfw: bool, verbose: bool) -> Result<PathBuf> {
    let dir = dir_path(nsfw)?;

    if verbose {
        println!("→ Scanning directory: {}", dir.display());
    }

    if !dir.exists() {
        return Err(format!("Directory not found: {}", dir.display()).into());
    }

    let mut images = Vec::new();

    for entry in read_dir(&dir)? {
        let entry = entry?;
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
        )
        .into());
    }

    if verbose {
        println!("→ Found {} wallpaper(s)", images.len());
    }

    // Simple random selection using timestamp
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize)
        % images.len();

    let selected = images.into_iter().nth(idx).unwrap();

    if verbose {
        println!("→ Selected: {}", selected.display());
    }

    Ok(selected)
}

pub fn get_random_any(verbose: bool) -> Result<PathBuf> {
    let sfw_dir = dir_path(false)?;
    let nsfw_dir = dir_path(true)?;

    if verbose {
        println!("→ Scanning all directories for wallpapers");
    }

    let mut all_images = Vec::new();

    // Collect from SFW directory
    if sfw_dir.exists() {
        if verbose {
            println!("→ Scanning SFW: {}", sfw_dir.display());
        }
        if let Ok(entries) = read_dir(&sfw_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && let Some(ext) = path.extension()
                {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "bmp") {
                        all_images.push(path);
                    }
                }
            }
        }
    }

    // Collect from NSFW directory
    if nsfw_dir.exists() {
        if verbose {
            println!("→ Scanning NSFW: {}", nsfw_dir.display());
        }
        if let Ok(entries) = read_dir(&nsfw_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && let Some(ext) = path.extension()
                {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "bmp") {
                        all_images.push(path);
                    }
                }
            }
        }
    }

    if all_images.is_empty() {
        return Err(
            "No wallpapers found in any directory. Download some first with 'parch sfw' or 'parch nsfw'"
                .into(),
        );
    }

    if verbose {
        println!(
            "→ Found {} total wallpaper(s) across all directories",
            all_images.len()
        );
    }

    // Simple random selection using timestamp
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize)
        % all_images.len();

    let selected = all_images.into_iter().nth(idx).unwrap();

    if verbose {
        println!("→ Selected: {}", selected.display());
    }

    Ok(selected)
}

fn dir_path(nsfw: bool) -> Result<PathBuf> {
    let mut path = if cfg!(windows) {
        let userprofile = std::env::var("USERPROFILE")
            .or_else(|_| {
                std::env::var("HOMEDRIVE").and_then(|drive| {
                    std::env::var("HOMEPATH").map(|path| format!("{}{}", drive, path))
                })
            })
            .map_err(|_| "User directory not found")?;
        PathBuf::from(userprofile)
    } else {
        let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
        PathBuf::from(home)
    };

    path.push("Pictures");
    path.push("Parch");
    if nsfw {
        path.push("Nsfw");
    }

    Ok(path)
}
