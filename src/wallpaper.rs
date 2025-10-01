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
        return windows_set(path);
    }

    #[cfg(not(windows))]
    {
        let path_str = path.to_string_lossy();

        // Try KDE
        if let Some(qdbus) = find(&["qdbus", "qdbus-qt5", "qdbus6"]) {
            if verbose {
                println!("→ Using KDE Plasma ({})", qdbus);
            }
            if kde(&qdbus, &path_str).is_ok() {
                return Ok(());
            }
        }

        // Try GNOME
        if exists("gsettings") {
            if verbose {
                println!("→ Using GNOME");
            }
            if gnome(&path_str).is_ok() {
                return Ok(());
            }
        }

        // Try feh
        if exists("feh") {
            if verbose {
                println!("→ Using feh");
            }
            if feh(&path_str).is_ok() {
                return Ok(());
            }
        }

        Err("No wallpaper setter found".into())
    }
}

#[cfg(windows)]
fn windows_set(path: &Path) -> Result<(), String> {
    let path_wide: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        if SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path_wide.as_ptr() as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 0
        {
            Err("Failed to set wallpaper".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(not(windows))]
fn kde(qdbus: &str, path: &str) -> Result<(), String> {
    let uri = if path.starts_with("file://") {
        path.to_string()
    } else {
        format!("file://{}", path)
    };

    let script = format!(
        "var d=desktops();for(i=0;i<d.length;i++){{d[i].wallpaperPlugin=\"org.kde.image\";\
         d[i].currentConfigGroup=[\"Wallpaper\",\"org.kde.image\",\"General\"];\
         d[i].writeConfig(\"Image\",\"{}\")}}",
        uri
    );

    Command::new(qdbus)
        .args([
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &script,
        ])
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
fn gnome(path: &str) -> Result<(), String> {
    let uri = if path.starts_with("file://") {
        path.to_string()
    } else {
        format!("file://{}", path)
    };

    Command::new("gsettings")
        .args(["set", "org.gnome.desktop.background", "picture-uri", &uri])
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
fn feh(path: &str) -> Result<(), String> {
    Command::new("feh")
        .args(["--bg-fill", path])
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
