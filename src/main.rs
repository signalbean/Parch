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
        if args.verbose {
            local::get_random(args.nsfw, args.verbose)?
        } else {
            local::get_random(args.nsfw, false)?
        }
    } else {
        let post = if args.verbose {
            api::fetch(args.id, args.nsfw, args.verbose)?
        } else {
            api::fetch(args.id, args.nsfw, false)?
        };

        let url = api::image_url(&post)?;

        if args.verbose {
            download::save(post.id, &url, post.rating == "e", args.verbose)?
        } else {
            download::save(post.id, &url, post.rating == "e", false)?
        }
    };

    wallpaper::set(&path, false)?;
    println!("✓ Applied");

    Ok(())
}
