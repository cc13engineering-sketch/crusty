/**
 * Crusty UI — Font loading helpers for Crusty Engine games
 *
 * Usage:
 *   import { CrustyUI } from '../crusty-ui.js';
 *
 *   // Load Google Fonts dynamically
 *   await CrustyUI.loadGoogleFont('Inter', [400, 500, 600, 700]);
 *   await CrustyUI.loadGoogleFont('JetBrains Mono', [400, 700]);
 *
 *   // Load a custom font file (TTF, OTF, WOFF, WOFF2)
 *   await CrustyUI.loadCustomFont('MyFont', './fonts/MyFont.woff2');
 *   await CrustyUI.loadCustomFont('MyFont', './fonts/MyFont-Bold.ttf', {
 *     weight: '700',
 *   });
 *
 *   // Apply a loaded font as the UI font
 *   CrustyUI.setFont('MyFont');
 */

export const CrustyUI = {
  /**
   * Load a Google Font by injecting a <link> into <head>.
   * Returns a promise that resolves when the font is ready.
   *
   * @param {string} family  — e.g. 'Inter', 'Roboto', 'Press Start 2P'
   * @param {number[]} [weights=[400,700]] — weight values to load
   * @param {boolean} [italic=false] — also load italic variants
   * @returns {Promise<void>}
   */
  async loadGoogleFont(family, weights = [400, 700], italic = false) {
    const axis = italic ? 'ital,wght' : 'wght';
    const tuples = [];
    for (const w of weights.sort((a, b) => a - b)) {
      if (italic) {
        tuples.push(`0,${w}`);
        tuples.push(`1,${w}`);
      } else {
        tuples.push(String(w));
      }
    }
    const params = `family=${encodeURIComponent(family)}:${axis}@${tuples.join(';')}`;
    const url = `https://fonts.googleapis.com/css2?${params}&display=swap`;

    if (document.querySelector(`link[href="${url}"]`)) return;

    _ensurePreconnect('https://fonts.googleapis.com');
    _ensurePreconnect('https://fonts.gstatic.com', true);

    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = url;
    document.head.appendChild(link);

    await document.fonts.ready;
  },

  /**
   * Load a custom font file (TTF, OTF, WOFF, WOFF2) using the FontFace API.
   *
   * @param {string} family — the CSS font-family name to register
   * @param {string} url    — path to the font file (relative or absolute)
   * @param {object} [opts] — optional descriptors
   * @param {string} [opts.weight='400'] — font-weight
   * @param {string} [opts.style='normal'] — font-style
   * @param {string} [opts.display='swap'] — font-display
   * @returns {Promise<FontFace>}
   */
  async loadCustomFont(family, url, opts = {}) {
    const { weight = '400', style = 'normal', display = 'swap' } = opts;
    const face = new FontFace(family, `url(${url})`, { weight, style, display });
    const loaded = await face.load();
    document.fonts.add(loaded);
    return loaded;
  },

  /**
   * Set the primary UI font (updates --font-ui CSS variable).
   * @param {string} family — must already be loaded
   */
  setFont(family) {
    document.documentElement.style.setProperty(
      '--font-ui', `'${family}', var(--font-system)`
    );
  },

  /**
   * Set the monospace font (updates --font-mono CSS variable).
   * @param {string} family — must already be loaded
   */
  setMonoFont(family) {
    document.documentElement.style.setProperty(
      '--font-mono', `'${family}', monospace`
    );
  },

  /**
   * Set the accent color (updates --color-accent CSS variable).
   * @param {string} color — any valid CSS color
   */
  setAccent(color) {
    document.documentElement.style.setProperty('--color-accent', color);
  },
};

function _ensurePreconnect(href, crossorigin = false) {
  if (document.querySelector(`link[rel="preconnect"][href="${href}"]`)) return;
  const link = document.createElement('link');
  link.rel = 'preconnect';
  link.href = href;
  if (crossorigin) link.crossOrigin = '';
  document.head.appendChild(link);
}
