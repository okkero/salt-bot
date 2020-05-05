use rand;
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
struct TenorGifObject {
    url: String
}

#[derive(Deserialize)]
struct TenorResponse {
    results: Vec<TenorGifObject>
}

pub type Result<T> = reqwest::Result<T>;

pub struct GifKeys {
    pub tenor: String,
    pub giphy: String,
}

fn tenor(key: &str, tag: &str) -> Result<String> {
    let request_url = format!("https://api.tenor.com/v1/random?key={}&q={}&limit=1", key, tag);
    let gif_url = reqwest::blocking::get(&request_url)?
        .json::<TenorResponse>()?
        .results.into_iter().next().unwrap()
        .url;

    Ok(gif_url)
}

fn giphy(key: &str, tag: &str) -> Result<String> {
    let url = format!("https://api.giphy.com/v1/gifs/random?api_key={}&tag={}", key, tag);

    Ok("Giphy WIP".to_string())
}

pub fn gif(keys: &GifKeys, tags: &[&str]) -> Result<String> {
    let mut rng = rand::thread_rng();
    let tag = tags[rng.gen_range(0, tags.len())];

    if rng.gen_bool(0.5) {
        tenor(&keys.tenor, tag)
    } else {
        giphy(&keys.giphy, tag)
    }
}