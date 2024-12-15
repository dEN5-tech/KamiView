use shaku::Component;
use crate::di::interfaces::IReqwestClient;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

/// HTTP клиент, использующий библиотеку reqwest для выполнения запросов.
/// Реализует интерфейс IReqwestClient для использования в контейнере DI.
#[derive(Component)]
#[shaku(interface = IReqwestClient)]
pub struct ReqwestClient {
    /// Внутренний клиент reqwest для выполнения HTTP запросов
    #[shaku(default)]
    client: reqwest::Client,
}

impl IReqwestClient for ReqwestClient {
    /// Выполняет GET запрос по указанному URL.
    ///
    /// # Параметры
    /// * `url` - URL для запроса
    ///
    /// # Возвращает
    /// * `Result<String>` - Результат запроса в виде строки
    fn get<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let response = self.client.get(url).send().await?;
            let text = response.text().await?;
            Ok(text)
        })
    }

    /// Выполняет POST запрос по указанному URL с телом запроса.
    ///
    /// # Параметры
    /// * `url` - URL для запроса
    /// * `body` - Тело запроса
    ///
    /// # Возвращает
    /// * `Result<String>` - Результат запроса в виде строки
    fn post<'a>(&'a self, url: &'a str, body: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let response = self.client.post(url)
                .body(body.to_string())
                .send()
                .await?;
            let text = response.text().await?;
            Ok(text)
        })
    }

    fn get_bytes<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + 'a>> {
        let client = self.client.clone();
        let url = url.to_string();
        Box::pin(async move {
            let response = client.get(&url).send().await?;
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        })
    }
}
