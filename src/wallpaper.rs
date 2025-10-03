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
        set_windows_wallpaper(path)
    }

    #[cfg(not(windows))]
    set_unix_wallpaper(path, verbose)
}

#[cfg(windows)]
fn set_windows_wallpaper(path: &Path) -> Result<(), String> {
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
    Ok(())
}

#[cfg(not(windows))]
fn set_unix_wallpaper(path: &Path, verbose: bool) -> Result<(), String> {
    let path_str = path.to_string_lossy();
    let uri = format!(
        "file://{}",
        path_str.strip_prefix("file://").unwrap_or(&path_str)
    );

    if let Some(q) = find_command(&["qdbus", "qdbus-qt5", "qdbus6"]) {
        if verbose {
            println!("→ Using KDE Plasma ({})", q);
        }
        if try_kde(&q, &uri).is_ok() {
            return Ok(());
        }
    }

    if command_exists("gsettings") {
        if verbose {
            println!("→ Using GNOME");
        }
        if run_command(
            "gsettings",
            &["set", "org.gnome.desktop.background", "picture-uri", &uri],
        )
        .is_ok()
        {
            return Ok(());
        }
    }

    if command_exists("feh") {
        if verbose {
            println!("→ Using feh");
        }
        if run_command("feh", &["--bg-fill", &path_str]).is_ok() {
            return Ok(());
        }
    }

    Err("No wallpaper setter found".into())
}

#[cfg(not(windows))]
fn try_kde(qdbus: &str, uri: &str) -> Result<(), String> {
    let script = format!(
        "var d=desktops();for(i=0;i<d.length;i++){{d[i].wallpaperPlugin=\"org.kde.image\";d[i].currentConfigGroup=[\"Wallpaper\",\"org.kde.image\",\"General\"];d[i].writeConfig(\"Image\",\"{}\")}}",
        uri
    );
    run_command(
        qdbus,
        &[
            "org.kde.plasmashell",
            "/PlasmaShell",
            "org.kde.PlasmaShell.evaluateScript",
            &script,
        ],
    )
}

#[cfg(not(windows))]
fn run_command(cmd: &str, args: &[&str]) -> Result<(), String> {
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
fn command_exists(cmd: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {}", cmd)])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn find_command(cmds: &[&str]) -> Option<String> {
    cmds.iter()
        .find(|&&c| command_exists(c))
        .map(|&s| s.to_string())
}
