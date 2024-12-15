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
