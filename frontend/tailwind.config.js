module.exports = {
  content: [
    './public/index.html',
    './src/**/*.svelte',
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f0fff4',
          100: '#dcfce7',
          200: '#bbf7d0',
          300: '#86efac',
          400: '#4ade80',
          500: '#48bb78',
          600: '#38a169',
          700: '#2f855a',
          800: '#276749',
          900: '#22543d',
          950: '#14432a',
        },
        secondary: {
          50: '#f7fafc',
          100: '#edf2f7',
          200: '#e2e8f0',
          300: '#cbd5e0',
          400: '#a0aec0',
          500: '#718096',
          600: '#4a5568',
          700: '#2d3748',
          800: '#1a202c',
          900: '#171923',
          950: '#0f1117',
        },
      },
      boxShadow: {
        'custom-light': '0 1px 3px rgba(0, 0, 0, 0.1)',
        'custom-medium': '0 8px 25px rgba(72, 187, 120, 0.25)',
        'custom-hover': '0 12px 35px rgba(72, 187, 120, 0.4)',
        'card-hover': '0 4px 12px rgba(0, 0, 0, 0.1)',
      },
      backgroundImage: {
        'btn-primary': 'linear-gradient(135deg, #2d3748, #4a5568)',
        'btn-secondary': 'linear-gradient(135deg, #48bb78, #38a169)',
      },
    },
  },
  plugins: [require('daisyui'), require('@tailwindcss/line-clamp')],
  daisyui: {
    themes: [
      {
        light: {
          "primary": "#48bb78",
          "secondary": "#2d3748", 
          "accent": "#f6ad55",
          "neutral": "#2d3748",
          "base-100": "#ffffff",
          "base-200": "#f7fafc",
          "base-300": "#edf2f7",
          "info": "#3182ce",
          "success": "#48bb78",
          "warning": "#ed8936",
          "error": "#e53e3e",
        },
      },
    ],
  },
};
