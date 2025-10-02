use std::path::Path;

#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use winapi::um::winuser::{
    SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SystemParametersInfoW,
};

#[cfg(not(windows))]
use std::process::Command;

pub fn set(path: &Path, verbose: bool) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    #[cfg(windows)]
    {
        if verbose {
            println!("→ Using Windows API");
        }
        let wide: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        unsafe {
            if SystemParametersInfoW(
                SPI_SETDESKWALLPAPER,
                0,
                wide.as_ptr() as *mut _,
                SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
            ) == 0
            {
                return Err("Failed to set wallpaper".into());
            }
        }
        return Ok(());
    }

    #[cfg(not(windows))]
    {
        let p = path.to_string_lossy();
        let uri = format!(
            "file://{}",
            if let Some(stripped) = p.strip_prefix("file://") {
                stripped
            } else {
                &p
            }
        );

        if let Some(q) = find(&["qdbus", "qdbus-qt5", "qdbus6"]) {
            if verbose {
                println!("→ Using KDE Plasma ({})", q);
            }
            if run(&q, &["org.kde.plasmashell", "/PlasmaShell", "org.kde.PlasmaShell.evaluateScript",
                &format!("var d=desktops();for(i=0;i<d.length;i++){{d[i].wallpaperPlugin=\"org.kde.image\";d[i].currentConfigGroup=[\"Wallpaper\",\"org.kde.image\",\"General\"];d[i].writeConfig(\"Image\",\"{}\")}}", uri)
            ]).is_ok() { return Ok(()); }
        }

        if exists("gsettings") {
            if verbose {
                println!("→ Using GNOME");
            }
            if run(
                "gsettings",
                &["set", "org.gnome.desktop.background", "picture-uri", &uri],
            )
            .is_ok()
            {
                return Ok(());
            }
        }

        if exists("feh") {
            if verbose {
                println!("→ Using feh");
            }
            if run("feh", &["--bg-fill", &p]).is_ok() {
                return Ok(());
            }
        }

        Err("No wallpaper setter found".into())
    }
}

#[cfg(not(windows))]
fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| e.to_string())
        .and_then(|s| {
            if s.success() {
                Ok(())
            } else {
                Err("Failed".into())
            }
        })
}

#[cfg(not(windows))]
fn exists(cmd: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {}", cmd)])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn find(cmds: &[&str]) -> Option<String> {
    cmds.iter().find(|&&c| exists(c)).map(|&s| s.to_string())
}
