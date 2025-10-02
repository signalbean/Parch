pub struct Args {
    pub nsfw: bool,
    pub id: Option<u64>,
    pub verbose: bool,
    pub local: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn parse() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);

    let first = match args.next() {
        Some(arg) => arg,
        None => return Err(format!("No arguments.\nTry '{} help'", NAME)),
    };

    let mut nsfw = false;
    let mut sfw = false;
    let mut id = None;
    let mut verbose = false;
    let mut local = false;

    let mut current = Some(first);
    while let Some(arg) = current {
        match arg.as_str() {
            "-h" | "help" => {
                print_help();
                std::process::exit(0);
            }
            "-v" | "version" => {
                println!("{} {}", NAME, VERSION);
                std::process::exit(0);
            }
            "nsfw" => nsfw = true,
            "sfw" => sfw = true,
            "local" => {
                local = true;
            }
            "-V" | "verbose" => verbose = true,
            "id" => {
                let id_str = args.next().ok_or("id requires a value")?;
                id = Some(id_str.parse().map_err(|_| "Invalid post ID")?);
            }
            _ => return Err(format!("Unknown: {}\nTry '{} help'", arg, NAME)),
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
    println!("{} v{}", NAME, VERSION);
    println!("Fetch and apply wallpapers from Konachan\n");
    println!("USAGE:\n    {} [OPTIONS]\n", NAME);
    println!("OPTIONS:");
    println!("    sfw                  Fetch SFW images");
    println!("    nsfw                 Fetch NSFW images");
    println!("    local <TYPE>         Use image from a local folder");
    println!("    id <ID>              Fetch specific post");
    println!("    -V, verbose          Verbose output");
    println!("    -h, help             Show help");
    println!("    -v, version          Show version");
}
