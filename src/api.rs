use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub id: u64,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
    pub rating: String,
}

pub fn fetch(id: Option<u64>, nsfw: bool, verbose: bool) -> Result<Post, String> {
    let url = match id {
        Some(i) => {
            if verbose {
                println!("→ Fetching post ID {}", i)
            }
            format!("https://konachan.net/post.json?tags=id%3A{}", i)
        }
        None => {
            let rating = if nsfw { "explicit" } else { "safe" };
            if verbose {
                println!("→ Fetching random {} post", rating)
            }
            format!(
                "https://konachan.net/post.json?tags=rating%3A{}%20order%3Arandom&limit=1",
                rating
            )
        }
    };

    let buf = ureq::get(&url)
        .header("User-Agent", "parch")
        .call()
        .map_err(|e| format!("Request failed: {}", e))?
        .into_body()
        .read_to_vec()
        .map_err(|e| e.to_string())?;

    serde_json::from_slice::<Vec<Post>>(&buf)
        .map_err(|e| e.to_string())?
        .into_iter()
        .next()
        .ok_or_else(|| "No post found".to_string())
}

pub fn image_url(post: &Post) -> Result<String, String> {
    let url = post
        .file_url
        .as_deref()
        .or(post.large_file_url.as_deref())
        .ok_or("No image URL")?;

    Ok(if url.starts_with("//") {
        format!("https:{}", url)
    } else if !url.starts_with("http") {
        format!("https://{}", url)
    } else {
        url.to_string()
    })
}
