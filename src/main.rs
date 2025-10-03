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
            let result = local::get_random(args.nsfw, false)?;
            drop(_spin);
            result
        }
    } else {
        let post = if args.verbose {
            api::fetch(args.id, args.nsfw, args.verbose)?
        } else {
            let _spin = SpinnerGuard::new("Fetching post");
            let result = api::fetch(args.id, args.nsfw, false)?;
            drop(_spin);
            result
        };

        let url = api::image_url(&post)?;

        if args.verbose {
            download::save(post.id, &url, post.rating == "e", args.verbose)?
        } else {
            let _spin = SpinnerGuard::new("Downloading image");
            let result = download::save(post.id, &url, post.rating == "e", false)?;
            drop(_spin);
            result
        }
    };

    if args.verbose {
        wallpaper::set(&path, args.verbose)?;
        println!("✓ Wallpaper set successfully.");
    } else {
        let _spin = SpinnerGuard::new("Applying wallpaper");
        wallpaper::set(&path, false)?;
        drop(_spin);
        print!("✓ Applied!");
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }

    Ok(())
}
