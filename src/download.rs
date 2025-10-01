use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn save(id: u64, url: &str, nsfw: bool, verbose: bool) -> Result<PathBuf> {
    let dir = dir_path(nsfw)?;
    let ext = url
        .rsplit('/')
        .next()
        .and_then(|n| n.split('?').next())
        .and_then(|n| n.rsplit('.').next())
        .filter(|e| !e.is_empty())
        .unwrap_or("jpg");

    let dest = dir.join(format!("{}.{}", id, ext));

    if verbose {
        println!("→ Downloading from: {}", url);
        println!("→ Saving to: {}", dest.display());
    }

    let resp = ureq::get(url)
        .set("User-Agent", "parch")
        .call()
        .map_err(|e| format!("Download failed: {}", e))?;

    let mut file = File::create(&dest)?;
    let bytes = copy(&mut resp.into_reader(), &mut file)?;

    if bytes == 0 {
        return Err("Empty file".into());
    }

    if verbose {
        println!("→ Downloaded {} bytes", bytes);
    }

    dest.canonicalize().or(Ok(dest))
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

    create_dir_all(&path)?;
    Ok(path)
}
