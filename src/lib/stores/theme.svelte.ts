// Store de Svelte 5 para el control reactivo del tema de la aplicación (Claro/Oscuro)
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark';

class ThemeStore {
  // Estado reactivo (rune $state)
  current = $state<Theme>('light');

  constructor() {
    if (browser) {
      // Cargar tema guardado, predeterminado a 'light'
      const savedTheme = localStorage.getItem('theme') as Theme | null;
      this.current = savedTheme || 'light';
      this.applyTheme();
    }
  }

  setTheme(theme: Theme) {
    this.current = theme;
    if (browser) {
      localStorage.setItem('theme', theme);
      this.applyTheme();
    }
  }

  toggle() {
    this.setTheme(this.current === 'light' ? 'dark' : 'light');
  }

  private applyTheme() {
    if (browser) {
      if (this.current === 'dark') {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    }
  }
}

export const theme = new ThemeStore();
