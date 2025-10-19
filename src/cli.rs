pub struct Args {
    pub nsfw: bool,
    pub id: Option<u64>,
    pub verbose: bool,
    pub local: bool,
}

pub fn parse() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let first = args
        .next()
        .ok_or_else(|| format!("No arguments.\nTry '{} help'", env!("CARGO_PKG_NAME")))?;

    let (mut nsfw, mut sfw, mut id, mut verbose, mut local) = (false, false, None, false, false);

    let mut current = Some(first);
    while let Some(arg) = current {
        match arg.as_str() {
            "-h" | "help" => {
                print_help();
                std::process::exit(0)
            }
            "-v" | "version" => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                std::process::exit(0)
            }
            "nsfw" => nsfw = true,
            "sfw" => sfw = true,
            "local" => local = true,
            "-V" | "verbose" => verbose = true,
            "id" => {
                id = Some(
                    args.next()
                        .ok_or("id requires a value")?
                        .parse()
                        .map_err(|_| "Invalid post ID")?,
                )
            }
            _ => {
                return Err(format!(
                    "Unknown: {}\nTry '{} help'",
                    arg,
                    env!("CARGO_PKG_NAME")
                ));
            }
        }
        current = args.next();
    }

    if nsfw && sfw {
        return Err("Cannot use nsfw and sfw together".into());
    }
    if local && id.is_some() {
        return Err("Cannot use local with id".into());
    }
    if id.is_none() && !nsfw && !sfw {
        return Err("Please provide a post ID with id or a type sfw/nsfw".into());
    }

    Ok(Args {
        nsfw,
        id,
        verbose,
        local,
    })
}

fn print_help() {
    println!(
        "{} v{}\nFetch and apply wallpapers from Konachan\n\nUSAGE:\n    {} [OPTIONS]\n\nOPTIONS:\
    \n    sfw                  Fetch SFW images\
    \n    nsfw                 Fetch NSFW images\
    \n    local <TYPE>         Use downloaded images\
    \n    id <ID>              Fetch specific post\
    \n    -V, verbose          Verbose output\
    \n    -h, help             Show help\
    \n    -v, version          Show version",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME")
    );
}
