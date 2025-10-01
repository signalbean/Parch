pub struct Args {
    pub nsfw: bool,
    pub id: Option<u64>,
    pub verbose: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn parse() -> Result<Args, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        return Err(format!("No arguments.\nTry '{} help'", NAME));
    }

    let mut nsfw = false;
    let mut sfw = false;
    let mut id = None;
    let mut verbose = false;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
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
            "-V" | "verbose" => verbose = true,
            "id" => {
                i += 1;
                if i >= args.len() {
                    return Err("id requires a value".into());
                }
                id = Some(args[i].parse().map_err(|_| "Invalid post ID")?);
            }
            _ => return Err(format!("Unknown: {}\nTry '{} help'", args[i], NAME)),
        }
        i += 1;
    }

    if nsfw && sfw {
        return Err("Cannot use nsfw and sfw together".into());
    }

    if id.is_none() && !nsfw && !sfw {
        return Err("Please specify either a post ID with id or a category with sfw / nsfw".into());
    }

    Ok(Args { nsfw, id, verbose })
}

fn print_help() {
    println!("{} v{}", NAME, VERSION);
    println!("Fetch and apply wallpapers from Konachan\n");
    println!("USAGE:\n    {} [OPTIONS]\n", NAME);
    println!("OPTIONS:");
    println!("    sfw            Fetch SFW images");
    println!("    nsfw           Fetch NSFW images");
    println!("    id <ID>        Download specific post");
    println!("    -V, verbose    Verbose output");
    println!("    -h, help       Show help");
    println!("    -v, version    Show version");
}
