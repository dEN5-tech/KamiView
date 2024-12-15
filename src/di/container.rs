//! Модуль контейнера для внедрения зависимостей.
//! Предоставляет централизованный способ управления зависимостями во всем приложении.

use shaku::module;
use shaku::HasComponent;
use crate::kodik::KodikClient;
use crate::shikimori::{
    ShikimoriClient, 
    ShikimoriOAuthComponent,
    ShikimoriClientParameters,
    ShikimoriOAuth,
    ShikimoriOAuthParameters
};
use crate::mpv::{MpvClient, MpvClientParameters};
use crate::storage::{Storage, StorageParameters};
use crate::di::interfaces::{IKodik, IShikimoriClient, IMpvClient, IStorage, IReqwestClient};
use std::sync::{Arc, Mutex};
use crate::client::ReqwestClient;
use crate::utils::constants::CONFIG;

// Include the generated environment variables module
include!(concat!(env!("OUT_DIR"), "/env.rs"));

/// Основной контейнер внедрения зависимостей, управляющий всеми компонентами приложения.
/// Использует фреймворк shaku для внедрения зависимостей.
module! {
    pub DIContainer {
        components = [
            KodikClient, 
            ShikimoriClient, 
            ShikimoriOAuthComponent,
            MpvClient, 
            Storage, 
            ReqwestClient
        ],
        providers = []
    }
}

// Make container Send + Sync
unsafe impl Send for DIContainer {}
unsafe impl Sync for DIContainer {}

/// Псевдоним типа для контейнера DI для удобства использования
pub type Container = DIContainer;

impl Container {
    /// Создает новый экземпляр контейнера DI со всеми настроенными компонентами.
    /// 
    /// # Возвращает
    /// * `Self` - Полностью настроенный контейнер DI
    ///
    /// # Пример
    /// ```
    /// let container = Container::new();
    /// ```
    pub fn new() -> Self {
        // Load environment variables from generated module
        let env = EnvVars::new();
        
        Self::builder()
            .with_component_parameters::<KodikClient>(env.kodikapikey)
            .with_component_parameters::<ShikimoriClient>(ShikimoriClientParameters {
                base_url: "https://shikimori.one/api".to_string(),
                client_id: env.shikimoriclientid.clone(),
                client_secret: env.shikimoriclientsecret.clone(),
            })
            .with_component_parameters::<MpvClient>(MpvClientParameters {
                socket_path: CONFIG.mpv_socket_path.to_string(),
                sender: Arc::new(Mutex::new(None)),
            })
            .with_component_parameters::<Storage>(StorageParameters {
                path: Storage::initialize_path(),
            })
            .build()
    }

    /// Получает интерфейс клиента Kodik API.
    ///
    /// # Возвращает
    /// * `&dyn IKodik` - Ссылка на реализацию клиента Kodik
    pub fn kodik(&self) -> &dyn IKodik {
        self.resolve_ref()
    }

    /// Получает интерфейс клиента Shikimori API.
    ///
    /// # Возвращает
    /// * `&dyn IShikimoriClient` - Ссылка на реализацию клиента Shikimori
    pub fn shikimori(&self) -> &dyn IShikimoriClient {
        self.resolve_ref()
    }

    /// Получает интерфейс клиента MPV плеера.
    ///
    /// # Возвращает
    /// * `&dyn IMpvClient` - Ссылка на реализацию клиента MPV
    pub fn mpv(&self) -> &dyn IMpvClient {
        self.resolve_ref()
    }

    /// Получает интерфейс хранилиа данных.
    ///
    /// # Возвращает
    /// * `&dyn IStorage` - Ссылка на реализацию хранилища
    pub fn storage(&self) -> &dyn IStorage {
        self.resolve_ref()
    }

    /// Получает интерфейс HTTP клиента.
    ///
    /// # Возвращает
    /// * `&dyn IReqwestClient` - Ссылка на реализацию HTTP клиента
    pub fn reqwest(&self) -> &dyn IReqwestClient {
        self.resolve_ref()
    }
}
