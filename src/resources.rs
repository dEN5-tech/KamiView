use std::collections::HashMap;
use once_cell::sync::Lazy;
use reqwest::Client;

static EMBEDDED_RESOURCES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("resources/home.svg", include_str!("resources/home.svg"));
    m.insert("resources/settings.svg", include_str!("resources/settings.svg"));
    m.insert("resources/profile.svg", include_str!("resources/profile.svg"));
    m.insert("resources/arrow_back.svg", include_str!("resources/arrow_back.svg"));
    m.insert("resources/shikimori.svg", include_str!("resources/shikimori.svg"));
    m.insert("resources/imdb.svg", include_str!("resources/imdb.svg"));
    m.insert("resources/link.svg", include_str!("resources/link.svg"));
    m.insert("resources/no_image.svg", include_str!("resources/no_image.svg"));
    m.insert("resources/loading.svg", include_str!("resources/loading.svg"));
    m
});

pub fn get_svg(path: &str) -> Option<&'static str> {
    let full_path = if !path.starts_with("resources/") {
        format!("resources/{}", path)
    } else {
        path.to_string()
    };
    EMBEDDED_RESOURCES.get(full_path.as_str()).copied()
}

pub async fn fetch_image(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let client = Client::new();
    client.get(url)
        .send()
        .await?
        .bytes()
        .await
        .map(|b| b.to_vec())
}

