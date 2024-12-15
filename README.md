# KamiView

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
```

Этот README.md основан на следующих частях кодовой базы:

- Конфигурация окна:

```85:92:KamiView/src/main.rs
    // Create window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("KamiView")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 720.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .expect("Failed to build window");
```


- Конфигурация сборки:

```6:43:KamiView/build.rs
fn main() {
    println!("cargo:rerun-if-changed=src/gui/assets");
    println!("cargo:rerun-if-changed=kami-view-front/src");
    println!("cargo:rerun-if-changed=.env");
    
    // Build frontend first
    build_frontend().expect("Failed to build frontend");
    
    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dir = Path::new(&out_dir);

    // Create required directories in the output
    fs::create_dir_all(dest_dir).unwrap();

    // Copy assets with timestamp check
    copy_assets_with_timestamp(dest_dir);

    // Generate env module
    generate_env_module(dest_dir);
}

fn build_frontend() -> Result<(), Box<dyn std::error::Error>> {
    let frontend_dir = Path::new("kami-view-front");
    
    // Check if pnpm is installed
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();

    let package_manager = if pnpm_check.is_ok() {
        "pnpm"
    } else {
        // Fallback to npm if pnpm is not available
        "npm"
    };

    println!("Building frontend using {}", package_manager);
```


- Конфигурация темы:

```1:74:KamiView/kami-view-front/tailwind.config.cjs
module.exports = {
  content: ["./src/**/*.{js,jsx}"],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Dark theme
        dark: {
          background: '#0f172a',
          'background-alt': '#1e293b',
          card: 'rgba(30, 41, 59, 0.7)',
          'card-hover': 'rgba(30, 41, 59, 0.9)',
          border: 'rgba(255, 255, 255, 0.1)',
          text: {
            DEFAULT: '#f8fafc',
            secondary: '#94a3b8',
            muted: '#64748b'
          }
        },
        // Light theme
        light: {
          background: '#f8fafc',
          'background-alt': '#f1f5f9',
          card: 'rgba(255, 255, 255, 0.8)',
          'card-hover': 'rgba(255, 255, 255, 0.95)',
          border: 'rgba(0, 0, 0, 0.1)',
          text: {
            DEFAULT: '#0f172a',
            secondary: '#475569',
            muted: '#64748b'
          }
        },
        // Brand colors
        primary: {
          DEFAULT: '#6366f1',
          hover: '#4f46e5',
          light: '#818cf8',
          dark: '#4338ca'
        },
        error: {
          DEFAULT: '#ef4444',
          hover: '#dc2626'
        },
        success: {
          DEFAULT: '#22c55e',
          hover: '#16a34a'
        }
      },
      gridTemplateColumns: {
        'auto-fill-220': 'repeat(auto-fill, minmax(220px, 1fr))',
      },
      backdropBlur: {
        'glass': '10px',
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        }
      }
    },
  },
  plugins: [
    require('tailwind-scrollbar')({ nocompatible: true }),
  ],
};
```

