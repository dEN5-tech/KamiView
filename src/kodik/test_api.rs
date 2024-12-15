use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use regex::Regex;

#[derive(Debug, Error)]
pub enum KodikError {
    #[error("Service error: {0}")]
    ServiceError(String),
    #[error("Token error: {0}")]
    TokenError(String),
    #[error("No results found for query: {0}")]
    NoResults(String),
    #[error("Unexpected behavior: {0}")]
    UnexpectedBehavior(String),
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SearchResult {
    time: String,
    total: i32,
    results: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct MediaInfo {
    pub series_count: i32,
    pub translations: Vec<Translation>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Translation {
    id: String,
    #[serde(rename = "type")]
    translation_type: String,
    name: String,
}

#[derive(Clone)]
pub struct KodikParser {
    token: String,
    client: Client,
    use_lxml: bool,
}

// Async methods implementation
impl KodikParser {
    pub async fn new(token: Option<String>, use_lxml: bool) -> Result<Self> {
        let token = match token {
            Some(t) => t,
            None => "447d179e875efe44217f20d1ee2146be".to_string()
        };

        Ok(Self {
            token,
            client: Client::new(),
            use_lxml,
        })
    }

    pub async fn base_search(&self, title: &str, limit: Option<i32>, include_material_data: bool) -> Result<Value> {
        if self.token.is_empty() {
            return Err(KodikError::TokenError("Token not specified".to_string()).into());
        }

        let payload = json!({
            "token": self.token,
            "title": title,
            "limit": limit.unwrap_or(50),
            "with_material_data": include_material_data
        });

        let response = self.client.post("https://kodikapi.com/search")
            .json(&payload)
            .send()
            .await?;

        let data: Value = response.json().await?;

        if let Some(error) = data.get("error") {
            if error.as_str() == Some("Отсутствует или неверный токен") {
                return Err(KodikError::TokenError("Invalid or missing token".to_string()).into());
            }
            return Err(KodikError::ServiceError(error.to_string()).into());
        }

        if data["total"].as_i64() == Some(0) {
            return Err(KodikError::NoResults(format!("No results found for query: {}", title)).into());
        }

        Ok(data)
    }

    pub async fn search(&self, title: &str, limit: Option<i32>) -> Result<Vec<Value>> {
        let search_data = self.base_search(title, limit, true).await?;
        let mut data = Vec::new();
        let mut added_titles = Vec::new();

        let results = search_data["results"].as_array()
            .ok_or_else(|| KodikError::UnexpectedBehavior("Invalid results format".to_string()))?;

        for result in results {
            let title = result["title"].as_str()
                .ok_or_else(|| KodikError::UnexpectedBehavior("Missing title".to_string()))?;

            if !added_titles.contains(&title.to_string()) {
                let mut additional_data = HashMap::new();
                
                if let Some(obj) = result.as_object() {
                    for (key, value) in obj {
                        if !["title", "type", "year", "screenshots", "translation",
                            "shikimori_id", "kinopoisk_id", "imdb_id", "worldart_link",
                            "id", "link", "title_orig", "other_title", "created_at",
                            "updated_at", "quality", "material_data"].contains(&key.as_str()) {
                            additional_data.insert(key.clone(), value.clone());
                        }
                    }
                }

                data.push(json!({
                    "title": title,
                    "title_orig": result["title_orig"],
                    "other_title": result.get("other_title"),
                    "type": result["type"],
                    "year": result["year"],
                    "screenshots": result["screenshots"],
                    "shikimori_id": result.get("shikimori_id"),
                    "kinopoisk_id": result.get("kinopoisk_id"),
                    "imdb_id": result.get("imdb_id"),
                    "worldart_link": result.get("worldart_link"),
                    "additional_data": additional_data,
                    "material_data": result.get("material_data")
                }));

                added_titles.push(title.to_string());
            }
        }

        Ok(data)
    }

    pub async fn translations(&self, id: &str, id_type: &str) -> Result<Vec<Translation>> {
        let info = self.get_info(id, id_type).await?;
        Ok(info.translations)
    }

    pub async fn series_count(&self, id: &str, id_type: &str) -> Result<i32> {
        let info = self.get_info(id, id_type).await?;
        Ok(info.series_count)
    }

    async fn _link_to_info(&self, id: &str, id_type: &str, https: bool) -> Result<String> {
        if self.token.is_empty() {
            return Err(KodikError::TokenError("Token not specified".to_string()).into());
        }

        let serv = match id_type {
            "shikimori" => format!("https://kodikapi.com/get-player?title=Player&hasPlayer=false&url=https%3A%2F%2Fkodikdb.com%2Ffind-player%3FshikimoriID%3D{}&token={}&shikimoriID={}", id, self.token, id),
            "kinopoisk" => format!("https://kodikapi.com/get-player?title=Player&hasPlayer=false&url=https%3A%2F%2Fkodikdb.com%2Ffind-player%3FkinopoiskID%3D{}&token={}&kinopoiskID={}", id, self.token, id),
            "imdb" => format!("https://kodikapi.com/get-player?title=Player&hasPlayer=false&url=https%3A%2F%2Fkodikdb.com%2Ffind-player%3FkinopoiskID%3D{}&token={}&imdbID={}", id, self.token, id),
            _ => return Err(KodikError::UnexpectedBehavior("Unknown id type".to_string()).into()),
        };

        let data: Value = self.client.get(&serv).send().await?.json().await?;

        if let Some(error) = data.get("error") {
            if error.as_str() == Some("Отсутствует или неверный токен") {
                return Err(KodikError::TokenError("Invalid or missing token".to_string()).into());
            }
            return Err(KodikError::ServiceError(error.to_string()).into());
        }

        if !data["found"].as_bool().unwrap_or(false) {
            return Err(KodikError::NoResults(format!("No data found for {} id \"{}\"", id_type, id)).into());
        }

        let link = data["link"].as_str()
            .ok_or_else(|| KodikError::UnexpectedBehavior("Missing link in response".to_string()))?;

        Ok(format!("{}:{}", if https { "https" } else { "http" }, link))
    }

    pub async fn get_info(&self, id: &str, id_type: &str) -> Result<MediaInfo> {
        let link = self._link_to_info(id, id_type, true).await?;
        let html = self.client.get(&link).send().await?.text().await?;
        
        self.process_info_html(&html, &link)
    }

    fn process_info_html(&self, html: &str, link: &str) -> Result<MediaInfo> {
        let document = Html::parse_document(html);

        let is_serial = self._is_serial(link);
        let is_video = self._is_video(link);

        if is_serial {
            let series_selector = Selector::parse("div.serial-series-box select option").unwrap();
            let series_count = document.select(&series_selector).count() as i32;
            let translations = self._parse_translations(&document, true)?;

            Ok(MediaInfo {
                series_count,
                translations,
            })
        } else if is_video {
            let translations = self._parse_translations(&document, false)?;

            Ok(MediaInfo {
                series_count: 0,
                translations,
            })
        } else {
            Err(KodikError::UnexpectedBehavior("Link not recognized as serial or video".to_string()).into())
        }
    }

    pub async fn get_link(&self, id: &str, id_type: &str, seria_num: i32, translation_id: &str) -> Result<(String, i32)> {
        let link = self._link_to_info(id, id_type, true).await?;
        let html = self.client.get(&link).send().await?.text().await?;
        
        let (media_hash, media_id, url_params) = self.process_first_html(&html, translation_id, seria_num)?;
        
        let url = if seria_num != 0 {
            format!("https://kodik.info/serial/{}/{}/720p?min_age=16&first_url=false&season=1&episode={}", 
                media_id, media_hash, seria_num)
        } else {
            format!("https://kodik.info/video/{}/{}/720p?min_age=16&first_url=false&season=1&episode=0",
                media_id, media_hash)
        };

        let html = self.client.get(&url).send().await?.text().await?;
        let (video_type, video_hash, video_id) = self.process_second_html(&html)?;
        
        let (link_data, max_quality) = self._get_link_with_data(&video_type, &video_hash, &video_id, &url_params).await?;

        let download_url = link_data.replace("https://", "");
        let download_url = &download_url[2..download_url.len()-26];

        Ok((download_url.to_string(), max_quality))
    }

    fn process_first_html(&self, html: &str, translation_id: &str, seria_num: i32) -> Result<(String, String, Value)> {
        let document = Html::parse_document(html);
        
        // Extract urlParams
        let re = Regex::new(r"urlParams\s*=\s*(\{[^}]+\})").unwrap();
        let url_params: Value = if let Some(caps) = re.captures(html) {
            serde_json::from_str(caps.get(1).unwrap().as_str())?
        } else {
            return Err(KodikError::UnexpectedBehavior("Failed to extract urlParams".to_string()).into());
        };

        let (media_hash, media_id) = self._extract_media_info(&document, translation_id, seria_num)?;
        
        Ok((media_hash, media_id, url_params))
    }

    fn process_second_html(&self, html: &str) -> Result<(String, String, String)> {
        let document = Html::parse_document(html);
        self._extract_video_info(&document)
    }

    async fn _get_link_with_data(&self, video_type: &str, video_hash: &str, video_id: &str, url_params: &Value) 
        -> Result<(String, i32)> {
        let params = json!({
            "hash": video_hash,
            "id": video_id,
            "type": video_type,
            "d": url_params["d"],
            "d_sign": url_params["d_sign"],
            "pd": url_params["pd"],
            "pd_sign": url_params["pd_sign"],
            "ref": "",
            "ref_sign": url_params["ref_sign"],
            "bad_user": "true",
            "cdn_is_working": "true",
        });

        let post_link = self._get_post_link().await?;
        let response = self.client.post(format!("https://kodik.info{}", post_link))
            .json(&params)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let data: Value = response.json().await?;
        let url = self._convert(&data["links"]["360"][0]["src"].as_str().unwrap());
        let max_quality = data["links"].as_object().unwrap().keys()
            .filter_map(|k| k.parse::<i32>().ok())
            .max()
            .unwrap_or(360);

        let decoded = match BASE64.decode(url.as_bytes()) {
            Ok(decoded) => String::from_utf8(decoded)?,
            Err(_) => {
                let padded = format!("{}==", url);
                String::from_utf8(BASE64.decode(padded.as_bytes())?)?
            }
        };

        Ok((decoded.replace("https:", ""), max_quality))
    }

    async fn _get_post_link(&self) -> Result<String> {
        let script_url = "https://kodik.info/assets/js/app.js";
        let response = self.client.get(script_url).send().await?;
        let data = response.text().await?;

        let start = data.find("$.ajax").ok_or_else(|| KodikError::UnexpectedBehavior("Ajax call not found".to_string()))? + 30;
        let end = data.find("cache:!1").ok_or_else(|| KodikError::UnexpectedBehavior("End of URL not found".to_string()))? - 3;
        
        let encoded_url = &data[start..end];
        let decoded = String::from_utf8(BASE64.decode(encoded_url)?)?;
        
        Ok(decoded)
    }

    pub async fn get_token() -> Result<String> {
        let client = Client::new();
        let script_url = "https://kodik-add.com/add-players.min.js?v=2";
        let response = client.get(script_url).send().await?;
        let data = response.text().await?;
        
        let token_start = data.find("token=")
            .ok_or_else(|| KodikError::UnexpectedBehavior("Token not found".to_string()))? + 7;
        let token_end = data[token_start..].find('"')
            .ok_or_else(|| KodikError::UnexpectedBehavior("Token end not found".to_string()))?;
        
        Ok(data[token_start..token_start + token_end].to_string())
    }

    // Synchronous helper methods
    fn _is_serial(&self, iframe_url: &str) -> bool {
        iframe_url[iframe_url.find(".info/").unwrap() + 6..].starts_with('s')
    }

    fn _is_video(&self, iframe_url: &str) -> bool {
        iframe_url[iframe_url.find(".info/").unwrap() + 6..].starts_with('v')
    }

    fn _parse_translations(&self, document: &Html, is_serial: bool) -> Result<Vec<Translation>> {
        let selector = if is_serial {
            Selector::parse("div.serial-translations-box select option").unwrap()
        } else {
            Selector::parse("div.movie-translations-box select option").unwrap()
        };

        let translations = document.select(&selector)
            .map(|option| {
                let id = option.value().attr("value").unwrap_or("0").to_string();
                let translation_type = match option.value().attr("data-translation-type").unwrap_or("unknown") {
                    "voice" => "Озвучка",
                    "subtitles" => "Субтитры",
                    _ => "Неизвестно",
                }.to_string();
                let name = option.text().collect::<String>();

                Translation {
                    id,
                    translation_type,
                    name,
                }
            })
            .collect::<Vec<_>>();

        if translations.is_empty() {
            Ok(vec![Translation {
                id: "0".to_string(),
                translation_type: "Неизвестно".to_string(),
                name: "Неизвестно".to_string(),
            }])
        } else {
            Ok(translations)
        }
    }

    fn _convert(&self, input: &str) -> String {
        input.chars()
            .map(|c| self._convert_char(c))
            .collect()
    }

    fn _convert_char(&self, c: char) -> char {
        let is_lower = c.is_lowercase();
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        
        if let Some(idx) = alphabet.chars().position(|x| x == c.to_ascii_uppercase()) {
            let new_char = alphabet.chars().nth((idx + 13) % 26).unwrap();
            if is_lower {
                new_char.to_ascii_lowercase()
            } else {
                new_char
            }
        } else {
            c
        }
    }

    fn _extract_media_info(&self, document: &Html, translation_id: &str, seria_num: i32) 
        -> Result<(String, String)> {
        if translation_id == "0" {
            return Ok(("".to_string(), "".to_string()));
        }

        let selector = if seria_num != 0 {
            Selector::parse("div.serial-translations-box select option").unwrap()
        } else {
            Selector::parse("div.movie-translations-box select option").unwrap()
        };

        for option in document.select(&selector) {
            if option.value().attr("data-id").unwrap_or("") == translation_id {
                return Ok((
                    option.value().attr("data-media-hash").unwrap_or("").to_string(),
                    option.value().attr("data-media-id").unwrap_or("").to_string(),
                ));
            }
        }

        Err(KodikError::UnexpectedBehavior("Translation not found".to_string()).into())
    }

    fn _extract_video_info(&self, document: &Html) -> Result<(String, String, String)> {
        let scripts: Vec<_> = document.select(&Selector::parse("script").unwrap()).collect();
        let script_text = scripts.get(4)
            .ok_or_else(|| KodikError::UnexpectedBehavior("Script not found".to_string()))?
            .text()
            .collect::<String>();

        let video_type = Self::extract_value(&script_text, ".type = '", "'")
            .ok_or_else(|| KodikError::UnexpectedBehavior("Video type not found".to_string()))?;
        let video_hash = Self::extract_value(&script_text, ".hash = '", "'")
            .ok_or_else(|| KodikError::UnexpectedBehavior("Video hash not found".to_string()))?;
        let video_id = Self::extract_value(&script_text, ".id = '", "'")
            .ok_or_else(|| KodikError::UnexpectedBehavior("Video ID not found".to_string()))?;

        Ok((video_type, video_hash, video_id))
    }

    fn extract_value(text: &str, start_marker: &str, end_marker: &str) -> Option<String> {
        text.find(start_marker)
            .map(|i| {
                let start = i + start_marker.len();
                text[start..].find(end_marker)
                    .map(|end| text[start..start + end].to_string())
            })
            .flatten()
    }
}