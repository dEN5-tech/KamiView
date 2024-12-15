# KamiView

![](https://i.imgur.com/j3w2mXA.png)

KamiView - это десктопное приложение для просмотра аниме, построенное на Rust и Preact.

## 🌟 Особенности

- Современный пользовательский интерфейс с поддержкой светлой и тёмной темы
- Интеграция с Kodik API для просмотра аниме
- Интеграция с Shikimori для получения информации об аниме
- Встроенный MPV плеер
- Кроссплатформенность (Windows, Linux, macOS)

## 🚀 Технологии

### Backend
- Rust
- Tokio (асинхронный рантайм)
- wry (WebView)
- MPV (воспроизведение видео)

### Frontend
- Preact
- Vite
- TailwindCSS
- Framer Motion (анимации)

## 📦 Установка

1. Убедитесь, что у вас установлены:
   - Rust (последняя стабильная версия)
   - Node.js
   - pnpm (рекомендуется) или npm
   - MPV плеер

2. Клонируйте репозиторий:
```bash
git clone https://github.com/dEN5-tech/KamiView.git
cd KamiView
```

3. Создайте файл `.env` на основе `.env.example` и заполните необходимые API ключи:
```env
KODIK_API_KEY=your_kodik_api_key_here
SHIKIMORI_CLIENT_ID=your_client_id_here 
SHIKIMORI_CLIENT_SECRET=your_client_secret_here
```

4. Установите зависимости и соберите проект:
```bash
# Установка зависимостей frontend
cd kami-view-front
pnpm install

# Сборка проекта
cd ..
cargo build --release
```

## 🔧 Разработка

- Запуск в режиме разработки:
```bash
# Terminal 1 - Frontend
cd kami-view-front
pnpm dev

# Terminal 2 - Backend
cargo run
```

## 📝 Конфигурация

- Настройки окна приложения находятся в `src/main.rs` (строки 85-92)
- Конфигурация тем в `kami-view-front/tailwind.config.cjs`
- Настройки сборки в `vite.config.js` и `build.rs`

## 🤝 Вклад в проект

Приветствуются pull request'ы! Для крупных изменений, пожалуйста, сначала создайте issue для обсуждения предлагаемых изменений.

## 📄 Лицензия

[MIT](LICENSE)
