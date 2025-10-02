mod api;
mod cli;
mod download;
mod local;
mod wallpaper;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::parse()?;

    let path = if args.local {
        // Use local wallpaper
        local::get_random(args.nsfw, args.verbose)?
    } else {
        // Fetch from Konachan
        let post = api::fetch(&args)?;
        let url = api::image_url(&post)?;
        let full_url = if url.starts_with("//") {
            format!("https:{}", url)
        } else if !url.starts_with("http") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        download::save(post.id, &full_url, post.rating == "e", args.verbose)?
    };

    wallpaper::set(&path, args.verbose)?;

    if args.verbose {
        println!("✓ Wallpaper set successfully.");
    } else {
        println!("✓ Applied!");
    }
    Ok(())
}
