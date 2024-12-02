use reqwest::{Client, Proxy};
use serde::de::DeserializeOwned;

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new() -> Self {
        let proxy = match Proxy::http("http://127.0.0.1:7899") {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to create proxy: {}", e);
                panic!("Failed to initialize HTTP client");
            }
        };

        let client = Client::builder()
            .proxy(proxy)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: "https://jsonplaceholder.typicode.com".to_string(),
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        self.client.get(&url).send().await?.json::<T>().await
    }

    pub async fn get_image(&self, url: &str) -> Result<Vec<u8>, reqwest::Error> {
        self.client.get(url)
            .send()
            .await?
            .bytes()
            .await
            .map(|b| b.to_vec())
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}