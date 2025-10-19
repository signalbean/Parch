mod api;
mod cli;
mod download;
mod local;
mod paths;
mod wallpaper;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = cli::parse()?;

    let path = if args.local {
        local::get_random(args.nsfw, args.verbose)?
    } else {
        let post = api::fetch(args.id, args.nsfw, args.verbose)?;
        let url = api::image_url(&post)?;
        download::save(post.id, &url, post.rating == "e", args.verbose)?
    };

    wallpaper::set(&path, args.verbose)?;
    println!("✓ Applied");

    Ok(())
}
