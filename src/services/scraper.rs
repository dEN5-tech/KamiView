use scraper::{Html, Selector};

pub struct ScraperService;

#[derive(Debug, Clone)]
pub struct ScrapedData {
    pub title: String,
    pub description: Option<String>,
    pub links: Vec<String>,
}

impl ScraperService {
    pub async fn scrape_url(url: &str) -> Result<ScrapedData, String> {
        let client = reqwest::Client::new();
        
        // Fetch the webpage
        let response = client.get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        let html = response.text()
            .await
            .map_err(|e| e.to_string())?;

        // Parse the HTML
        let document = Html::parse_document(&html);
        
        // Create selectors
        let title_selector = Selector::parse("title").unwrap();
        let meta_desc_selector = Selector::parse("meta[name='description']").unwrap();
        let links_selector = Selector::parse("a[href]").unwrap();

        // Extract title
        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "No title found".to_string());

        // Extract description
        let description = document
            .select(&meta_desc_selector)
            .next()
            .and_then(|el| el.value().attr("content"))
            .map(String::from);

        // Extract links
        let links = document
            .select(&links_selector)
            .filter_map(|el| el.value().attr("href"))
            .map(String::from)
            .collect();

        Ok(ScrapedData {
            title,
            description,
            links,
        })
    }
} 