use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::json;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::collections::HashMap;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use regex::Regex;
use tokio;

/// Структура ответа поиска
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResponse {
    pub total: i32,
    pub results: Vec<MediaResult>,
    pub error: Option<String>,
    pub next_page_id: Option<String>
}

/// Структура ответа с информацией
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InfoResponse {
    pub series_count: i32,
    pub translations: Vec<Translation>
}

/// Структура перевода
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Translation {
    pub id: String,
    pub translation_type: String,
    pub name: String,
}

/// Структура результата медиа
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaResult {
    pub title: String,
    pub title_orig: String,
    pub other_title: Option<String>,
    pub media_type: Option<String>,
    pub year: i32,
    pub screenshots: Vec<String>,
    pub shikimori_id: Option<String>,
    pub kinopoisk_id: Option<String>,
    pub imdb_id: Option<String>,
    pub worldart_link: Option<String>,
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    pub material_data: Option<HashMap<String, serde_json::Value>>,
    pub link: String
}

/// Структура данных медиа
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaData {
    pub series_count: i32,
    pub translations: Vec<Translation>
}


impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.translation_type)
    }
}

/// Перечисление ошибок Kodik
#[derive(Error, Debug)]
pub enum KodikError {
    #[error("Token error: {0}")]
    TokenError(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("No results found: {0}")]
    NoResults(String),

    #[error("Invalid ID type: {0}")]
    InvalidIdType(String),

    #[error("Parser error: {0}")]
    ParserError(String),
}

/// Структура парсера Kodik
#[derive(Debug, Clone)]
pub struct KodikParser {
    token: Option<String>,
    use_lxml: bool,
    client: Client,
}

impl KodikParser {
    /// Создать новый экземпляр KodikParser
    ///
    /// # Аргументы
    /// * `token` - Токен Kodik для поиска по базе. Если не указан, будет предпринята попытка автоматического получения токена
    /// * `use_lxml` - Использовать парсер lxml (в некоторых случаях lxml может не работать)
    pub async fn new(token: Option<String>, use_lxml: bool) -> Result<Self> {
        let token = match token {
            Some(t) => Some(t),
            None => {
                Some(Self::get_token().await?)
            }
        };

        Ok(Self {
            token,
            use_lxml,
            client: Client::new(),
        })
    }

    /// Создать пустой экземпляр KodikParser
    pub fn empty() -> Self {
        Self {
            token: None,
            use_lxml: false,
            client: Client::new(),
        }
    }

    /// Получить токен с Kodik
    pub async fn get_token() -> Result<String> {
        let script_url = "https://kodik-add.com/add-players.min.js?v=2";
        let response = reqwest::get(script_url).await.context("Не удалось выполнить запрос для получения токена")?;
        let data = response.text().await.context("Не удалось прочитать ответ для токена")?;
        
        let token_start = data.find("token=").context("Не удалось найти начало токена")? + 6;
        let token_end = data[token_start..].find('"').context("Не удалось найти конец токена")?;
        
        Ok(data[token_start..token_start + token_end].to_string())
    }

    /// Вспомогательная функция для декодирования символов
    fn convert_char(&self, char: char) -> char {
        let is_lower = char.is_lowercase();
        let alph = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        if alph.contains(char.to_ascii_uppercase()) {
            let index = alph.find(char.to_ascii_uppercase()).unwrap();
            let new_char = alph.chars().nth((index + 13) % alph.len()).unwrap();
            if is_lower {
                new_char.to_lowercase().next().unwrap()
            } else {
                new_char
            }
        } else {
            char
        }
    }

    /// Декодировать строку со ссылкой
    fn convert(&self, string: &str) -> String {
        string.chars().map(|c| self.convert_char(c)).collect()
    }

    /// Получить ссылку из информации
    async fn _link_to_info(&self, id: &str, id_type: &str, https: bool) -> Result<String> {
        if let Some(token) = &self.token {
            let serv = match id_type {
                "shikimori" => format!(
                    "https://kodikapi.com/get-player?title=Player&hasPlayer=false&url=https%3A%2F%2Fkodikdb.com%2Ffind-player%3FshikimoriID%3D{}&token={}&shikimoriID={}", 
                    id, token, id
                ),
                "kinopoisk" => format!(
                    "https://kodikapi.com/get-player?title=Player&hasPlayer=false&url=https%3A%2F%2Fkodikdb.com%2Ffind-player%3FkinopoiskID%3D{}&token={}&kinopoiskID={}", 
                    id, token, id
                ),
                "imdb" => format!(
                    "https://kodikapi.com/get-player?title=Player&hasPlayer=false&url=https%3A%2F%2Fkodikdb.com%2Ffind-player%3FkinopoiskID%3D{}&token={}&imdbID={}", 
                    id, token, id
                ),
                _ => return Err(KodikError::InvalidIdType(id_type.to_string()).into()),
            };

            let data = self.client.get(&serv).send().await?.json::<serde_json::Value>().await?;

            if let Some(error) = data.get("error").and_then(|e| e.as_str()) {
                match error {
                    "Отсутствует или неверный токен" => {
                        return Err(KodikError::TokenError("Отсутствует или неверный токен".into()).into());
                    },
                    _ => return Err(KodikError::ServiceError(error.to_string()).into()),
                }
            }

            if !data.get("found").and_then(|f| f.as_bool()).unwrap_or(false) {
                return Err(KodikError::NoResults(format!("Нет данных по {} id \"{}\"", id_type, id)).into());
            }

            let link = data.get("link").and_then(|l| l.as_str()).unwrap_or("");
            Ok(if https { 
                format!("https:{}", link) 
            } else { 
                format!("http:{}", link) 
            })
        } else {
            Err(KodikError::TokenError("Токен kodik не указан".into()).into())
        }
    }

    /// Проверить, является ли ссылка на сериал
    fn is_serial(&self, iframe_url: &str) -> bool {
        iframe_url.contains(".info/s")
    }

    /// Проверить, является ли ссылка на видео
    fn is_video(&self, iframe_url: &str) -> bool {
        iframe_url.contains(".info/v")
    }

    /// Генерировать словарь переводов
    fn generate_translations_dict(&self, translations_div: Option<Vec<scraper::element_ref::ElementRef>>) -> Vec<Translation> {
        if let Some(options) = translations_div {
            let mut translations = Vec::new();
            for translation in options {
                if let Some(id) = translation.value().attr("data-id") {
                    let t_type = match translation.value().attr("data-translation-type") {
                        Some("voice") => "Озвучка",
                        Some("subtitles") => "Субтитры",
                        _ => "Неизвестно",
                    };
                    let name = translation.text().collect::<Vec<_>>().join("");
                    translations.push(Translation {
                        id: id.to_string(),
                        translation_type: t_type.to_string(),
                        name,
                    });
                }
            }
            translations
        } else {
            vec![Translation {
                id: "0".to_string(),
                translation_type: "Неизвестно".to_string(),
                name: "Неизвестно".to_string(),
            }]
        }
    }

    /// Получить информацию: series_count, translations
    pub async fn get_info(&self, id: &str, id_type: &str) -> Result<InfoResponse> {
        let link = self._link_to_info(id, id_type, true).await?;
        let response = self.client.get(&link).send().await?.text().await?;
        let document = Html::parse_document(&response);

        if self.is_serial(&link) {
            // Для сериала
            let series_count = document.select(&Selector::parse("div.serial-series-box select option").unwrap())
                .count() as i32;

            let translations_div = match document.select(&Selector::parse("div.serial-translations-box select").unwrap())
                .next() {
                Some(div) => {
                    let options = div.select(&Selector::parse("option").unwrap()).collect::<Vec<_>>();
                    Some(options)
                },
                None => None
            };

            Ok(InfoResponse {
                series_count,
                translations: self.generate_translations_dict(translations_div)
            })
        } else if self.is_video(&link) {
            // Для фильма
            let translations_div = match document.select(&Selector::parse("div.movie-translations-box select").unwrap())
                .next() {
                Some(div) => {
                    let options = div.select(&Selector::parse("option").unwrap()).collect::<Vec<_>>();
                    Some(options)
                },
                None => None
            };

            Ok(InfoResponse {
                series_count: 0,
                translations: self.generate_translations_dict(translations_div)
            })
        } else {
            Err(KodikError::ParserError("Ссылка на данные не была распознана как ссылка на сериал или фильм".into()).into())
        }
    }

    /// Извлечение media_hash и media_id из контейнера переводов
    fn extract_media_info<'a>(&self, container: &scraper::ElementRef<'a>, translation_id: &str) -> Result<(String, String)> {
        let selector = Selector::parse("option").unwrap();
        
        for translation in container.select(&selector) {
            if translation.value().attr("data-id") == Some(translation_id) {
                let media_hash = translation.value().attr("data-media-hash")
                    .ok_or_else(|| KodikError::ParserError("Не найден media-hash".into()))?;
                    
                let media_id = translation.value().attr("data-media-id")
                    .ok_or_else(|| KodikError::ParserError("Не найден media-id".into()))?;
                    
                return Ok((media_hash.to_string(), media_id.to_string()));
            }
        }
        
        Err(KodikError::NoResults(format!("Перевод с id \"{}\" не найден.", translation_id)).into())
    }

    /// Получить ссылку на видео файл
    pub async fn get_link(&self, id: &str, id_type: &str, seria_num: i32, translation_id: &str) -> Result<(String, i32)> {
        let link = self._link_to_info(id, id_type, true).await?;
        let response = self.client.get(&link).send().await?.text().await?;
        
        let (media_hash, media_id) = {
            let document = Html::parse_document(&response);

            if translation_id != "0" {
                if seria_num != 0 {
                    // Сериал с известной озвучкой на более чем 1 серию
                    let container = document.select(&Selector::parse("div.serial-translations-box select").unwrap())
                        .next()
                        .ok_or_else(|| KodikError::ParserError("Не найден контейнер переводов сериала".into()))?;
                    self.extract_media_info(&container, translation_id)?
                } else {
                    // Фильм/одна серия с несколькими переводами
                    let container = document.select(&Selector::parse("div.movie-translations-box select").unwrap())
                        .next()
                        .ok_or_else(|| KodikError::ParserError("Не найден контейнер переводов фильма".into()))?;
                    self.extract_media_info(&container, translation_id)?
                }
            } else {
                // Если translation_id == "0", используем данные из скрипта
                let script_text = document.select(&Selector::parse("script").unwrap())
                    .nth(4)
                    .ok_or_else(|| KodikError::ParserError("Не найден скрипт с данными".into()))?
                    .text()
                    .collect::<String>();

                let hash_regex = Regex::new(r"\.hash = '([^']+)'").unwrap();
                let id_regex = Regex::new(r"\.id = '([^']+)'").unwrap();

                let media_hash = hash_regex.captures(&script_text)
                    .and_then(|c| c.get(1))
                    .ok_or_else(|| KodikError::ParserError("Не найден hash в скрипте".into()))?
                    .as_str()
                    .to_string();

                let media_id = id_regex.captures(&script_text)
                    .and_then(|c| c.get(1))
                    .ok_or_else(|| KodikError::ParserError("Не найден id в скрипте".into()))?
                    .as_str()
                    .to_string();

                (media_hash, media_id)
            }
        };

        let url = if seria_num != 0 {
            format!("https://kodik.info/serial/{}/{}/720p?min_age=16&first_url=false&season=1&episode={}", 
                   media_id, media_hash, seria_num)
        } else {
            format!("https://kodik.info/video/{}/{}/720p?min_age=16&first_url=false", 
                   media_id, media_hash)
        };

        let response = self.client.get(&url).send().await?.text().await?;
        
        let (script_url, script_text) = {
            let document = Html::parse_document(&response);

            let script_url = document.select(&Selector::parse("script").unwrap())
                .nth(1)
                .and_then(|el| el.value().attr("src"))
                .ok_or_else(|| KodikError::ParserError("Не найден URL скрипта".into()))?
                .to_string();

            let script_text = document.select(&Selector::parse("script").unwrap())
                .nth(4)
                .ok_or_else(|| KodikError::ParserError("Не найден скрипт с данными".into()))?
                .text()
                .collect::<String>();

            (script_url, script_text)
        };

        let type_regex = Regex::new(r"\.type = '([^']+)'").unwrap();
        let hash_regex = Regex::new(r"\.hash = '([^']+)'").unwrap();
        let id_regex = Regex::new(r"\.id = '([^']+)'").unwrap();

        let video_type = type_regex.captures(&script_text)
            .and_then(|c| c.get(1))
            .ok_or_else(|| KodikError::ParserError("Не найден type в скрипте".into()))?
            .as_str();

        let video_hash = hash_regex.captures(&script_text)
            .and_then(|c| c.get(1))
            .ok_or_else(|| KodikError::ParserError("Не найден hash в скрипте".into()))?
            .as_str();

        let video_id = id_regex.captures(&script_text)
            .and_then(|c| c.get(1))
            .ok_or_else(|| KodikError::ParserError("Не найден id в скрипте".into()))?
            .as_str();

        let url_params_regex = Regex::new(r#"urlParams\s*=\s*'([^']+)'"#).unwrap();
        let url_params_json = url_params_regex.captures(&response)
            .and_then(|c| c.get(1))
            .ok_or_else(|| KodikError::ParserError("Не найдены urlParams".into()))?
            .as_str();

        let url_params_value: serde_json::Value = serde_json::from_str(url_params_json)?;
        let mut url_params = HashMap::new();
        if let serde_json::Value::Object(map) = url_params_value {
            for (key, value) in map {
                let string_value = match value {
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Number(n) => n.to_string(), 
                    serde_json::Value::String(s) => s,
                    _ => value.to_string(),
                };
                url_params.insert(key, string_value);
            }
        }

        let (link_data, max_quality) = self.get_link_with_data(
            video_type,
            video_hash,
            video_id,
            &url_params,
            &script_url
        ).await?;

        let download_url = str::replace(&link_data, "https://", "");
        let download_url = &download_url[2..download_url.len()-26]; // Remove :hls:manifest.m3u8

        Ok((download_url.to_string(), max_quality))
    }
    
    /// Вспомогательная функция для получения ссылки с данными
    async fn get_link_with_data(
        &self,
        video_type: &str,
        video_hash: &str,
        video_id: &str,
        url_params: &HashMap<String, String>,
        script_url: &str
    ) -> Result<(String, i32)> {
        let params = [
            ("hash", video_hash.to_string()),
            ("id", video_id.to_string()),
            ("type", video_type.to_string()),
            ("d", url_params.get("d").cloned().unwrap_or_default()),
            ("d_sign", url_params.get("d_sign").cloned().unwrap_or_default()),
            ("pd", url_params.get("pd").cloned().unwrap_or_default()),
            ("pd_sign", url_params.get("pd_sign").cloned().unwrap_or_default()),
            ("ref", "".to_string()),
            ("ref_sign", url_params.get("ref_sign").cloned().unwrap_or_default()),
            ("bad_user", "true".to_string()),
            ("cdn_is_working", "true".to_string()),
        ];

        let post_link = match self.get_post_link(script_url).await {
            Ok(link) => link,
            Err(e) => {
                eprintln!("Ошибка при получении post_link: {}", e);
                return Err(e);
            }
        };

        let post_url = format!("https://kodik.info{}", post_link);

        let mut header_map = reqwest::header::HeaderMap::new();
        header_map.insert(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap()
        );

        let response = match self.client.post(&post_url)
            .headers(header_map)
            .form(&params)
            .send()
            .await {
                Ok(r) => match r.json::<serde_json::Value>().await {
                    Ok(j) => j,
                    Err(e) => {
                        eprintln!("Ошибка при парсинге JSON ответа: {}", e);
                        return Err(e.into());
                    }
                },
                Err(e) => {
                    eprintln!("Ошибка при отправке POST запроса: {}", e);
                    return Err(e.into());
                }
            };

        let url = self.convert(&response["links"]["360"][0]["src"].as_str().ok_or_else(|| {
            KodikError::ParserError("Не удалось найти ссылку на 360p".into())
        })?);

        let max_quality = response["links"].as_object()
            .ok_or_else(|| KodikError::ParserError("Не найдены данные о качестве".into()))?
            .keys()
            .filter_map(|k| k.parse::<i32>().ok())
            .max()
            .ok_or_else(|| KodikError::ParserError("Не удалось определить максимальное качество".into()))?;

        let url_decoded = match base64::decode(url.as_bytes()) {
            Ok(decoded) => String::from_utf8_lossy(&decoded).to_string(),
            Err(_) => {
                let padded = format!("{}==", url);
                match base64::decode(padded.as_bytes()) {
                    Ok(decoded) => String::from_utf8_lossy(&decoded).to_string().replace("https:", ""),
                    Err(e) => {
                        eprintln!("Ошибка при декодировании base64: {}", e);
                        return Err(e.into());
                    }
                }
            }
        };

        Ok((url_decoded, max_quality))
    }

    /// Получить ссылку на скачивание
    pub async fn get_download_link(
        &self,
        id: &str,
        id_type: &str,
        seria_num: i32,
        translation_id: &str,
    ) -> Result<(String, i32)> {
        // Проверка токена
        let token = self.token.as_ref()
            .context("Токен не установлен")?;

        // Получение базовой ссылки на информацию
        let base_url = self._link_to_info(id, id_type, true).await?;
        
        // Получение HTML страницы
        let data = self.client.get(&base_url).send().await?.text().await?;

        // Извлечение urlParams
        let url_params_regex = Regex::new(r#"urlParams\s*=\s*'([^']+)'"#).unwrap();
        let url_params = url_params_regex.captures(&data)
            .and_then(|caps| caps.get(1))
            .ok_or_else(|| KodikError::ParserError("Не удалось найти urlParams".into()))?;

        let url_params_value: serde_json::Value = serde_json::from_str(url_params.as_str())?;
        let mut url_params = HashMap::new();
        if let serde_json::Value::Object(map) = url_params_value {
            for (key, value) in map {
                let string_value = match value {
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::String(s) => s,
                    _ => value.to_string(),
                };
                url_params.insert(key, string_value);
            }
        }

        // Получение media_hash и media_id в зависимости от типа контента
        let (media_hash, media_id) = {
            let document = Html::parse_document(&data);
            let selector = if seria_num != 0 {
                Selector::parse(".serial-translations-box select option").unwrap()
            } else {
                Selector::parse(".movie-translations-box select option").unwrap()
            };

            let mut found_hash = None;
            let mut found_id = None;

            for option in document.select(&selector) {
                if option.value().attr("data-id") == Some(translation_id) {
                    found_hash = option.value().attr("data-media-hash").map(|s| s.to_string());
                    found_id = option.value().attr("data-media-id").map(|s| s.to_string());
                    break;
                }
            }

            match (found_hash, found_id) {
                (Some(h), Some(i)) => (h, i),
                _ => return Err(KodikError::ParserError("Не удалось найти media_hash/id".into()).into())
            }
        };

        if translation_id == "0" {
            return Err(KodikError::ParserError("Требуется translation_id".into()).into());
        }

        // Формирование URL для получения видео
        let video_url = if seria_num != 0 {
            format!("https://kodik.info/serial/{}/{}/720p?min_age=16&first_url=false&season=1&episode={}", 
                media_id, media_hash, seria_num)
        } else {
            format!("https://kodik.info/video/{}/{}/720p?min_age=16&first_url=false", 
                media_id, media_hash)
        };

        // Получение данных видео
        let video_data = self.client.get(&video_url).send().await?.text().await?;

        // Получение script_url и параметров видео
        let script_url = {
            let video_document = Html::parse_document(&video_data);
            video_document.select(&Selector::parse("script[src*='app.serial']").unwrap())
                .next()
                .and_then(|script| script.value().attr("src"))
                .ok_or_else(|| KodikError::ParserError("Не удалось найти script_url".into()))?
                .to_string()
        };

        // Извлечение параметров видео из скрипта
        let script_text = {
            let video_document = Html::parse_document(&video_data);
            let scripts: Vec<_> = video_document.select(&Selector::parse("script").unwrap())
                .map(|s| s.text().collect::<String>())
                .collect();
                
            scripts.into_iter()
                .find(|text| text.contains("videoInfo.type"))
                .ok_or_else(|| KodikError::ParserError("Не найден скрипт с данными плеера".into()))?
        };

        // Извлекаем значения напрямую из скрипта
        let regex_type = Regex::new(r"videoInfo\.type\s*=\s*'([^']+)'").unwrap();
        let regex_hash = Regex::new(r"videoInfo\.hash\s*=\s*'([^']+)'").unwrap();
        let regex_id = Regex::new(r"videoInfo\.id\s*=\s*'([^']+)'").unwrap();

        let video_type = regex_type.captures(&script_text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| KodikError::ParserError("Не удалось найти videoInfo.type".into()))?;

        let video_hash = regex_hash.captures(&script_text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| KodikError::ParserError("Не удалось найти videoInfo.hash".into()))?;

        let video_id = regex_id.captures(&script_text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| KodikError::ParserError("Не удалось найти videoInfo.id".into()))?;

        // Получение финальной ссылки
        let (download_url, quality) = self.get_link_with_data(
            &video_type,
            &video_hash,
            &video_id,
            &url_params,
            &script_url
        ).await?;

        Ok((download_url[2..download_url.len()-26].to_string(), quality))
    }

    /// Вспомогательная функция для извлечения значения из скрипта
    fn extract_value(script: &str, key: &str) -> Result<String> {
        let regex = Regex::new(&format!(r#"{}\[^']*'([^']+)'"#, key)).unwrap();
        regex.captures(script)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| KodikError::ParserError(format!("Не удалось найти {}", key)).into())
    }

    /// Получить post_link из скрипта
    async fn get_post_link(&self, script_url: &str) -> Result<String> {
        let full_url = format!("https://kodik.info{}", script_url);
        let response = self.client.get(&full_url).send().await?.text().await?;
        
        let ajax_start = response.find("$.ajax")
            .ok_or_else(|| KodikError::ParserError("Не удалось найти $.ajax".into()))? + 30;

        let ajax_end = response[ajax_start..].find("cache:!1")
            .ok_or_else(|| KodikError::ParserError("Не удалось найти конец $.ajax".into()))? - 3;

        let ajax_str = &response[ajax_start..ajax_start + ajax_end];
        
        let decoded = String::from_utf8(BASE64.decode(ajax_str.as_bytes())?)?;

        Ok(decoded)
    }

    /// Поиск медиа по названию с базовым форматированием
    pub async fn search(
        &self,
        title: &str,
        limit: Option<i32>,
        include_material_data: bool,
        anime_status: Option<String>,
        strict: bool,
        only_anime: bool,
    ) -> Result<Vec<MediaResult>> {
        let search_data = self.base_search(
            title,
            limit,
            include_material_data,
            anime_status,
            strict
        ).await?;
        
        Ok(self.prettify_data(&search_data.results, only_anime))
    }

    /// Прямой поиск в базе Kodik
    pub async fn base_search(
        &self,
        title: &str,
        limit: Option<i32>,
        include_material_data: bool,
        anime_status: Option<String>,
        strict: bool,
    ) -> Result<SearchResponse> {
        let token = self.token.as_ref()
            .context("Токен не установлен")?;

        let mut payload = vec![
            ("token", token.clone()),
            ("title", if strict { format!("{} ", title) } else { title.to_string() }),
            ("with_material_data", if include_material_data { "true" } else { "false" }.to_string()),
            ("strict", if strict { "true" } else { "false" }.to_string()),
        ];

        if let Some(limit_val) = limit {
            payload.push(("limit", limit_val.to_string()));
        }

        if let Some(status) = anime_status {
            payload.push(("anime_status", status));
        }

        let response = self.client
            .post("https://kodikapi.com/search")
            .form(&payload)
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        if let Some(error) = &response.error {
            if error == "Отсутствует или неверный токен" {
                return Err(KodikError::TokenError("Отсутствует или неверный токен".into()).into());
            }
            return Err(KodikError::ServiceError(error.clone()).into());
        }

        if response.total == 0 {
            return Err(KodikError::NoResults(format!("По запросу \"{}\" ничего не найдено", title)).into());
        }

        Ok(response)
    }

    /// Поиск списка медиа с пагинацией
    pub async fn get_list(
        &self,
        limit_per_page: i32,
        pages_to_parse: i32,
        include_material_data: bool,
        anime_status: Option<String>,
        only_anime: bool,
        start_from: Option<String>,
    ) -> Result<(Vec<MediaResult>, Option<String>)> {
        let token = self.token.as_ref()
            .context("Токен не установлен")?;

        let mut results = Vec::new();
        let mut next_page = start_from;

        for _ in 0..pages_to_parse {
            let mut payload = vec![
                ("token", token.clone()),
                ("limit", limit_per_page.to_string()),
                ("with_material_data", if include_material_data { "true" } else { "false" }.to_string()),
            ];

            if let Some(status) = &anime_status {
                payload.push(("anime_status", status.clone()));
            }

            if let Some(next) = &next_page {
                payload.push(("next", next.clone()));
            }

            let response = self.client
                .post("https://kodikapi.com/list")
                .form(&payload)
                .send()
                .await?
                .json::<SearchResponse>()
                .await?;

            if let Some(error) = &response.error {
                if error == "Отсутствует или неверный токен" {
                    return Err(KodikError::TokenError("Отсутствует или неверный токен".into()).into());
                }
                return Err(KodikError::ServiceError(error.clone()).into());
            }

            if response.total == 0 {
                return Err(KodikError::NoResults(
                    "Ничего не найдено. Скорее всего произошла ошибка, попробуйте позже или сообщите об ошибке на гитхабе.".into()
                ).into());
            }

            // Обновить next_page
            next_page = response.next_page_id.as_ref()
                .and_then(|np| np.rfind('='))
                .map(|idx| response.next_page_id.as_ref().unwrap()[idx + 1..].to_string());

            results.extend(response.results);
        }

        Ok((self.prettify_data(&results, only_anime), next_page))
    }

    /// Поиск по ID с базовым форматированием
    pub async fn search_by_id(
        &self,
        id: &str,
        id_type: &str,
        limit: Option<i32>,
    ) -> Result<Vec<MediaResult>> {
        let search_data = self.base_search_by_id(id, id_type, limit, true).await?;
        Ok(self.prettify_data(&search_data.results, false))
    }

    /// Прямой поиск по ID в базе Kodik
    pub async fn base_search_by_id(
        &self,
        id: &str,
        id_type: &str,
        limit: Option<i32>,
        include_material_data: bool,
    ) -> Result<SearchResponse> {
        if !["shikimori", "kinopoisk", "imdb"].contains(&id_type) {
            return Err(KodikError::InvalidIdType(format!(
                "Поддерживаются только id shikimori, kinopoisk, imdb. Получено: {}", 
                id_type
            )).into());
        }

        let token = self.token.as_ref()
            .context("Токен не установлен")?;

        let id_key = format!("{}_id", id_type);
        let id_key = id_key.as_str();
        let mut payload = vec![
            ("token", token.clone()),
            (id_key, id.to_string()),
            ("with_material_data", if include_material_data { "true" } else { "false" }.to_string()),
        ];

        if let Some(limit_val) = limit {
            payload.push(("limit", limit_val.to_string()));
        }

        let response = self.client
            .post("https://kodikapi.com/search")
            .form(&payload)
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        if let Some(error) = &response.error {
            match error.as_str() {
                "Отсутствует или неверный токен" => {
                    return Err(KodikError::TokenError("Отсутствует или неверный токен".into()).into());
                },
                _ => return Err(KodikError::ServiceError(error.clone()).into()),
            }
        }

        if response.total == 0 {
            return Err(KodikError::NoResults(format!("По id {} \"{}\" ничего не найдено", id_type, id)).into());
        }

        Ok(response)
    }

    /// Вспомогательная функция для форматирования данных
    fn prettify_data(&self, results: &[MediaResult], only_anime: bool) -> Vec<MediaResult> {
        let mut data = Vec::new();
        let mut added_titles = Vec::new();

        for result in results {
            if only_anime {
                if let Some(media_type) = &result.media_type {
                    if !["anime-serial", "anime"].contains(&media_type.as_str()) {
                        continue;
                    }
                }
            }

            if !added_titles.contains(&result.title) {
                data.push(result.clone());
                added_titles.push(result.title.clone());
            }
        }

        data
    }

    /// Получить переводы для медиа по ID
    pub async fn translations(&self, id: &str, id_type: &str) -> Result<Vec<Translation>> {
        let info = self.get_info(id, id_type).await?;
        Ok(info.translations)
    }

    /// Получить количество серий для медиа по ID
    pub async fn series_count(&self, id: &str, id_type: &str) -> Result<i32> {
        let info = self.get_info(id, id_type).await?;
        Ok(info.series_count)
    }

    /// Получить ссылку до страницы с данными
    pub async fn get_info_full(&self, id: &str, id_type: &str) -> Result<InfoResponse> {
        self.get_info(id, id_type).await
    }
}

/*     // Initialize KodikParser
    let parser = KodikParser::new(Some("447d179e875efe44217f20d1ee2146be".to_string()), false).await?;

    // Search for title and get first result
    let search_results = parser.search(title, Some(10), true, None, false, true).await?;
    println!("search_results: {:?}", search_results);

    // Find best matching title using fuzzy search
    let best_match = search_results.iter()
        .max_by_key(|result| {
            let similarity = strsim::jaro_winkler(
                &result.title.to_lowercase(),
                &title.to_lowercase()
            );
            (similarity * 100.0) as i32
        })
        .ok_or_else(|| anyhow::anyhow!("No results found"))?;

    println!("Best match: {}", best_match.title);
    
    // Get shikimori ID and series count
    let shikimori_id = best_match.shikimori_id.as_ref()
        .ok_or_else(|| anyhow::anyhow!("No shikimori ID found"))?;
    let info = parser.get_info(shikimori_id, "shikimori").await?;
    println!("info: {:?}", info);
    
    // Create m3u8 playlist
    let mut playlist = String::from("#EXTM3U\n");
    
    // Add each episode to playlist
    let episode_count = if info.series_count > 0 { info.series_count } else { 1 };
    for episode in 1..=episode_count {
        let (download_link, quality) = parser.get_link(shikimori_id, "shikimori", episode, translation_id).await?;
        println!("download_link: {}, quality: {}", download_link, quality);
        let download_link = format!("https://{}/720.mp4/", download_link);
        playlist.push_str(&format!("#EXTINF:-1,Episode {}\n{}\n", episode, download_link));
    }

    // Write playlist to file
    let filename = format!("{}.m3u8", title.replace(" ", "_"));
    std::fs::write(&filename, playlist)?;
    println!("Created playlist: {}", filename); */