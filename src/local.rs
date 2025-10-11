use crate::paths::parch_dir;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png"];

pub fn get_random(nsfw: bool, verbose: bool) -> Result<PathBuf, String> {
    let dir = parch_dir(nsfw)?;

    if verbose {
        println!("→ Scanning directory: {}", dir.display());
    }
    if !dir.exists() {
        return Err(format!("Directory not found: {}", dir.display()));
    }

    let mut images = collect_images(&dir)?;

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

    let selected = select_random(&mut images);
    if verbose {
        println!("→ Selected: {}", selected.display());
    }

    Ok(selected)
}

fn collect_images(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    Ok(read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| is_image_file(p))
        .collect())
}

fn is_image_file(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| IMAGE_EXTS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
}

fn select_random(images: &mut Vec<PathBuf>) -> PathBuf {
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize)
        % images.len();
    images.swap_remove(idx)
}
