use crate::cli::Args;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub id: u64,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
    pub rating: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fetch(args: &Args) -> Result<Post> {
    let url = if let Some(id) = args.id {
        if args.verbose {
            println!("→ Fetching post ID {}", id);
        }
        format!("https://konachan.net/post.json?tags=id%3A{}", id)
    } else {
        let rating = if args.nsfw { "explicit" } else { "safe" };
        if args.verbose {
            println!("→ Fetching random {} post", rating);
        }
        format!(
            "https://konachan.net/post.json?tags=rating%3A{}%20order%3Arandom&limit=1",
            rating
        )
    };

    let resp = ureq::get(&url)
        .set("User-Agent", "parch")
        .call()
        .map_err(|e| format!("Request failed: {}", e))?;

    let posts: Vec<Post> = resp.into_json()?;

    posts
        .into_iter()
        .next()
        .ok_or_else(|| "No post found".into())
}

pub fn image_url(post: &Post) -> Result<&str> {
    post.file_url
        .as_deref()
        .or(post.large_file_url.as_deref())
        .map(|url| url.strip_prefix("//").unwrap_or(url))
        .ok_or_else(|| "No image URL".into())
}
