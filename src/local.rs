use crate::paths::parch_dir;
use std::fs::read_dir;
use std::path::PathBuf;

pub fn get_random(nsfw: bool, verbose: bool) -> Result<PathBuf, String> {
    let dir = parch_dir(nsfw)?;

    if verbose {
        println!("→ Scanning directory: {}", dir.display());
    }
    if !dir.exists() {
        return Err(format!("Directory not found: {}", dir.display()));
    }

    let mut images: Vec<PathBuf> = read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .map(|ext| {
                        matches!(
                            ext.to_string_lossy().to_lowercase().as_str(),
                            "jpg" | "jpeg" | "png" | "bmp"
                        )
                    })
                    .unwrap_or(false)
        })
        .collect();

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
