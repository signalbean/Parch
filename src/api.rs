use crate::cli::Args;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub id: u64,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
    pub rating: String,
}

pub fn fetch(args: &Args) -> Result<Post, String> {
    let url = if let Some(id) = args.id {
        if args.verbose {
            println!("→ Fetching post ID {}", id);
        }
        format!("https://konachan.net/post.json?tags=id%3A{}", id)
    } else {
        if args.verbose {
            let rating = if args.nsfw { "explicit" } else { "safe" };
            println!("→ Fetching random {} post", rating);
        }
        if args.nsfw {
            "https://konachan.net/post.json?tags=rating%3Aexplicit%20order%3Arandom&limit=1"
        } else {
            "https://konachan.net/post.json?tags=rating%3Asafe%20order%3Arandom&limit=1"
        }
        .to_string()
    };

    let mut resp = ureq::get(&url)
        .header("User-Agent", "parch")
        .call()
        .map_err(|e| format!("Request failed: {}", e))?
        .into_body();

    let buf = resp.read_to_vec().map_err(|e| e.to_string())?;

    let posts: Vec<Post> = serde_json::from_slice(&buf).map_err(|e| e.to_string())?;

    posts
        .into_iter()
        .next()
        .ok_or_else(|| "No post found".to_string())
}

pub fn image_url(post: &Post) -> Result<&str, String> {
    post.file_url
        .as_deref()
        .or(post.large_file_url.as_deref())
        .map(|url| url.strip_prefix("//").unwrap_or(url))
        .ok_or_else(|| "No image URL".to_string())
}
