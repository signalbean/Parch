mod api;
mod cli;
mod download;
mod local;
mod paths;
mod spinner;
mod wallpaper;

use spinner::SpinnerGuard;

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
            let _spin = SpinnerGuard::new("Picking local wallpaper");
            local::get_random(args.nsfw, false)?
        }
    } else {
        let post = if args.verbose {
            api::fetch(args.id, args.nsfw, args.verbose)?
        } else {
            let _spin = SpinnerGuard::new("Fetching post");
            api::fetch(args.id, args.nsfw, false)?
        };

        let url = api::image_url(&post)?;

        if args.verbose {
            download::save(post.id, &url, post.rating == "e", args.verbose)?
        } else {
            let _spin = SpinnerGuard::new("Downloading image");
            download::save(post.id, &url, post.rating == "e", false)?
        }
    };

    if args.verbose {
        wallpaper::set(&path, args.verbose)?;
    } else {
        let _spin = SpinnerGuard::new("Applying wallpaper");
        wallpaper::set(&path, false)?;
    }

    println!(
        "{}",
        if args.verbose {
            "✓ Wallpaper set successfully."
        } else {
            "✓ Applied!"
        }
    );
    Ok(())
}
