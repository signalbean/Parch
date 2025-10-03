use crate::paths::parch_dir;
use std::fs::{File, create_dir_all};
use std::io::copy;
use std::path::PathBuf;

pub fn save(id: u64, url: &str, nsfw: bool, verbose: bool) -> Result<PathBuf, String> {
    let dir = parch_dir(nsfw)?;
    create_dir_all(&dir).map_err(|e| e.to_string())?;

    let ext = extract_extension(url);
    let dest = dir.join(format!("{}.{}", id, ext));

    if verbose {
        println!(
            "→ Downloading from: {}\n→ Saving to: {}",
            url,
            dest.display()
        );
    }

    let resp = ureq::get(url)
        .header("User-Agent", "parch")
        .call()
        .map_err(|e| format!("Download failed: {}", e))?;

    let mut file = File::create(&dest).map_err(|e| e.to_string())?;
    let bytes = copy(&mut resp.into_body().into_reader(), &mut file).map_err(|e| e.to_string())?;

    if bytes == 0 {
        return Err("Empty file".to_string());
    }
    if verbose {
        println!("→ Downloaded {} bytes", bytes);
    }

    dest.canonicalize().or(Ok(dest))
}

fn extract_extension(url: &str) -> &str {
    url.rsplit('/')
        .next()
        .and_then(|n| n.split('?').next())
        .and_then(|n| n.rsplit('.').next())
        .filter(|e| !e.is_empty())
        .unwrap_or("jpg")
}
