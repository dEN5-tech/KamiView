mod api;

use anyhow::Result;
use shaku::Component;
use crate::di::interfaces::{IKodikSearch, IKodikInfo, IKodikPlayback, IKodik};
pub use api::{KodikParser, MediaResult, InfoResponse, Translation};
use std::sync::Arc;
use tokio::runtime::Runtime;
use std::future::Future;
use std::pin::Pin;

pub struct KodikClient {
    api_key: String,
    parser: KodikParser,
    runtime: Arc<Runtime>,
}

impl<M: shaku::Module> Component<M> for KodikClient {
    type Interface = dyn IKodik;
    type Parameters = String;

    fn build(_context: &mut shaku::ModuleBuildContext<M>, api_key: Self::Parameters) -> Box<Self::Interface> {
        let runtime = Arc::new(Runtime::new().unwrap());
        let rt = runtime.clone();

        let parser = rt.block_on(async {
            KodikParser::new(Some(api_key.clone()), false)
                .await
                .expect("Failed to initialize KodikParser")
        });

        Box::new(KodikClient {
            api_key,
            parser,
            runtime,
        })
    }
}

impl IKodik for KodikClient {}

impl IKodikSearch for KodikClient {
    fn search_anime<'a>(&'a self, query: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<MediaResult>>> + Send + 'a>> {
        Box::pin(async move {
            self.parser.search(query, Some(10), true, None, false, true).await
        })
    }
}

impl IKodikInfo for KodikClient {
    fn get_anime_info<'a>(&'a self, shikimori_id: &'a str) -> Pin<Box<dyn Future<Output = Result<InfoResponse>> + Send + 'a>> {
        Box::pin(async move {
            self.parser.get_info(shikimori_id, "shikimori").await
        })
    }

    fn get_translations<'a>(&'a self, shikimori_id: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<Translation>>> + Send + 'a>> {
        Box::pin(async move {
            let info = self.get_anime_info(shikimori_id).await?;
            Ok(info.translations)
        })
    }

    fn get_series_count<'a>(&'a self, shikimori_id: &'a str) -> Pin<Box<dyn Future<Output = Result<i32>> + Send + 'a>> {
        Box::pin(async move {
            let info = self.get_anime_info(shikimori_id).await?;
            Ok(info.series_count)
        })
    }
}

impl IKodikPlayback for KodikClient {
    fn get_episode_link<'a>(&'a self, shikimori_id: &'a str, episode: i32, translation_id: &'a str) -> Pin<Box<dyn Future<Output = Result<(String, i32)>> + Send + 'a>> {
        Box::pin(async move {
            self.parser.get_link(shikimori_id, "shikimori", episode, translation_id).await
        })
    }

    fn create_playlist<'a>(&'a self, _title: &'a str, shikimori_id: &'a str, translation_id: &'a str) -> Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let info = self.get_anime_info(shikimori_id).await?;
            let mut playlist = String::from("#EXTM3U\n");
            let episode_count = if info.series_count > 0 { info.series_count } else { 1 };

            for episode in 1..=episode_count {
                let (download_link, _quality) = self.get_episode_link(
                    shikimori_id,
                    episode,
                    translation_id
                ).await?;

                let download_link = format!("https://{}/720.mp4/", download_link);
                playlist.push_str(&format!(
                    "#EXTINF:-1,Episode {}\n{}\n",
                    episode,
                    download_link
                ));
            }

            Ok(playlist)
        })
    }
}
