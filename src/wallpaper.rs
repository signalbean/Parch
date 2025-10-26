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

pub fn set(path: &Path, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()).into());
    }

    #[cfg(windows)]
    {
        set_windows_wallpaper(path, verbose)
    }

    #[cfg(not(windows))]
    {
        set_unix_wallpaper(path, verbose)
    }
}

#[cfg(windows)]
fn set_windows_wallpaper(path: &Path, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
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
            return Err("Failed to set wallpaper using Windows API".into());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn set_unix_wallpaper(path: &Path, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let uri = create_file_uri(path);

    // Try KDE Plasma first
    if let Some(qdbus) = find_kde_command() {
        if verbose {
            println!("→ Using KDE Plasma ({})", qdbus);
        }
        if try_kde(qdbus, &uri).is_ok() {
            return Ok(());
        }
    }

    // Try GNOME
    if cmd_exists("gsettings") {
        if verbose {
            println!("→ Using GNOME");
        }
        if run_cmd(
            "gsettings",
            &["set", "org.gnome.desktop.background", "picture-uri", &uri],
        )
        .is_ok()
        {
            return Ok(());
        }
    }

    // Try feh as fallback
    if cmd_exists("feh") {
        if verbose {
            println!("→ Using feh");
        }
        if run_cmd("feh", &["--bg-fill", &path.to_string_lossy()]).is_ok() {
            return Ok(());
        }
    }

    Err("No supported wallpaper setter found".into())
}

#[cfg(not(windows))]
fn create_file_uri(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    let clean_path = path_str.strip_prefix("file://").unwrap_or(&path_str);
    format!("file://{}", clean_path)
}

#[cfg(not(windows))]
fn find_kde_command() -> Option<&'static str> {
    ["qdbus", "qdbus-qt5", "qdbus6"]
        .iter()
        .find(|&&cmd| cmd_exists(cmd))
        .copied()
}

#[cfg(not(windows))]
fn try_kde(qdbus: &str, uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(
        "var d=desktops();for(i=0;i<d.length;i++){{d[i].wallpaperPlugin=\"org.kde.image\";d[i].currentConfigGroup=[\"Wallpaper\",\"org.kde.image\",\"General\"];d[i].writeConfig(\"Image\",\"{}\")}}",
        uri
    );

    run_cmd(
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
fn run_cmd(cmd: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new(cmd).args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Command '{}' failed", cmd).into())
    }
}

#[cfg(not(windows))]
fn cmd_exists(cmd: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {}", cmd)])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
