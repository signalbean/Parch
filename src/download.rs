use std::fs::{File, create_dir_all};
use std::io::copy;
use std::path::PathBuf;

pub fn save(id: u64, url: &str, nsfw: bool, verbose: bool) -> Result<PathBuf, String> {
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

    create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}
