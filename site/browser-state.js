/**
 * browser-state.js — Browser → WASM shared memory channel
 *
 * Populates the BrowserState buffer so Rust code can read viewport size,
 * device capabilities, visibility, focus, audio state, FPS, etc.
 *
 * Usage in a game's index.html:
 *
 *   import { initBrowserState } from '../browser-state.js';
 *
 *   const wasm = await initWasm();
 *   const memory = wasm.memory;
 *   // ... after init() and setup_*() ...
 *
 *   const updateBrowserState = initBrowserState(memory, api.browser_state_ptr, api.browser_state_len);
 *
 *   function frame(now) {
 *       const dt = now - last;
 *       last = now;
 *       updateBrowserState(dt);   // before tick()
 *       tick(dt);
 *       // ... render ...
 *       requestAnimationFrame(frame);
 *   }
 *
 * Optionally pass an options object to initBrowserState:
 *   initBrowserState(memory, ptrFn, lenFn, {
 *       audioContext: myAudioCtx,         // to report AUDIO_UNLOCKED
 *       canvasSizeFn: function() {        // to report CANVAS_CSS_W/H
 *           return { w: canvas.clientWidth, h: canvas.clientHeight };
 *       }
 *   });
 *
 * Slot layout must match engine/crates/engine-core/src/browser.rs
 */

// Slot indices — keep in sync with Rust browser.rs
var BS_VIEWPORT_W       = 0;
var BS_VIEWPORT_H       = 1;
var BS_CANVAS_CSS_W     = 2;
var BS_CANVAS_CSS_H     = 3;
var BS_DEVICE_PIXEL_RATIO = 4;
var BS_IS_TOUCH_DEVICE  = 5;
var BS_MAX_TOUCH_POINTS = 6;
var BS_ORIENTATION      = 7;
var BS_DOCUMENT_VISIBLE = 8;
var BS_DOCUMENT_FOCUSED = 9;
var BS_AUDIO_UNLOCKED   = 10;
var BS_JS_FPS           = 11;
var BS_JS_FRAME_TIME_MS = 12;
var BS_ONLINE           = 13;
var BS_WALL_CLOCK_S     = 14;
var BS_SCROLL_Y         = 15;

/**
 * Detect touch support across browsers.
 * Returns 1.0 if touch is supported, 0.0 otherwise.
 */
function detectTouch() {
    if ('ontouchstart' in window) return 1.0;
    // IE10/11, Edge legacy
    if (typeof navigator.msMaxTouchPoints === 'number' && navigator.msMaxTouchPoints > 0) return 1.0;
    // Modern fallback
    if (typeof navigator.maxTouchPoints === 'number' && navigator.maxTouchPoints > 0) return 1.0;
    // Media query fallback (broad browser support)
    if (window.matchMedia && window.matchMedia('(pointer: coarse)').matches) return 1.0;
    return 0.0;
}

/**
 * Get max touch points with IE/Edge legacy fallback.
 */
function getMaxTouchPoints() {
    if (typeof navigator.maxTouchPoints === 'number') return navigator.maxTouchPoints;
    if (typeof navigator.msMaxTouchPoints === 'number') return navigator.msMaxTouchPoints;
    return 0;
}

/**
 * Get screen orientation angle with broad fallback.
 */
function getOrientationAngle() {
    // Modern API
    if (typeof screen !== 'undefined' && screen.orientation && typeof screen.orientation.angle === 'number') {
        return screen.orientation.angle;
    }
    // Older WebKit (iOS Safari < 16.4, Android Browser)
    if (typeof window.orientation === 'number') {
        // window.orientation uses 0/90/-90/180; normalize to 0/90/180/270
        var o = window.orientation;
        if (o < 0) o += 360;
        return o;
    }
    return 0;
}

/**
 * Check page visibility with vendor-prefix fallback.
 */
function isDocumentVisible() {
    if (typeof document.visibilityState === 'string') {
        return document.visibilityState === 'visible' ? 1.0 : 0.0;
    }
    // Vendor prefixes (IE10, old WebKit)
    if (typeof document.msVisibilityState === 'string') {
        return document.msVisibilityState === 'visible' ? 1.0 : 0.0;
    }
    if (typeof document.webkitVisibilityState === 'string') {
        return document.webkitVisibilityState === 'visible' ? 1.0 : 0.0;
    }
    // Assume visible if API not available
    return 1.0;
}

/**
 * Initialize the browser state bridge.
 *
 * @param {WebAssembly.Memory} memory - WASM linear memory
 * @param {function(): number} ptrFn  - browser_state_ptr() from WASM exports
 * @param {function(): number} lenFn  - browser_state_len() from WASM exports
 * @param {object} [opts]             - Optional configuration
 * @param {AudioContext} [opts.audioContext] - AudioContext to monitor
 * @param {function(): {w: number, h: number}} [opts.canvasSizeFn] - Returns CSS canvas size
 * @returns {function(number): void}  - Call each frame with dt (ms) BEFORE tick()
 */
export function initBrowserState(memory, ptrFn, lenFn, opts) {
    opts = opts || {};

    var bsPtr = ptrFn();
    var bsLen = lenFn();
    var slots = new Float64Array(memory.buffer, bsPtr, bsLen);

    // One-time values
    slots[BS_IS_TOUCH_DEVICE]    = detectTouch();
    slots[BS_MAX_TOUCH_POINTS]   = getMaxTouchPoints();
    slots[BS_DEVICE_PIXEL_RATIO] = window.devicePixelRatio || 1;

    /**
     * Per-frame update. Call BEFORE tick().
     * @param {number} dt - Frame delta in milliseconds
     */
    return function updateBrowserState(dt) {
        // Re-create Float64Array view if WASM memory grew (buffer detached).
        // Re-call ptrFn() in case the allocation moved during growth.
        if (slots.buffer !== memory.buffer) {
            bsPtr = ptrFn();
            slots = new Float64Array(memory.buffer, bsPtr, bsLen);
        }

        slots[BS_VIEWPORT_W] = window.innerWidth;
        slots[BS_VIEWPORT_H] = window.innerHeight;

        // Canvas CSS size (if provider given)
        if (opts.canvasSizeFn) {
            var cs = opts.canvasSizeFn();
            slots[BS_CANVAS_CSS_W] = cs.w;
            slots[BS_CANVAS_CSS_H] = cs.h;
        }

        // DPR can change (drag window between monitors)
        slots[BS_DEVICE_PIXEL_RATIO] = window.devicePixelRatio || 1;

        slots[BS_ORIENTATION]      = getOrientationAngle();
        slots[BS_DOCUMENT_VISIBLE] = isDocumentVisible();
        slots[BS_DOCUMENT_FOCUSED] = document.hasFocus() ? 1.0 : 0.0;

        // Audio state
        if (opts.audioContext) {
            slots[BS_AUDIO_UNLOCKED] = (opts.audioContext.state === 'running') ? 1.0 : 0.0;
        }

        // Frame timing
        slots[BS_JS_FRAME_TIME_MS] = dt;
        slots[BS_JS_FPS] = dt > 0 ? 1000 / dt : 0;

        // Network
        slots[BS_ONLINE] = (typeof navigator.onLine === 'boolean') ? (navigator.onLine ? 1.0 : 0.0) : 1.0;

        // Wall clock (seconds since epoch)
        slots[BS_WALL_CLOCK_S] = Date.now() / 1000;

        // Scroll position
        slots[BS_SCROLL_Y] = window.pageYOffset || document.documentElement.scrollTop || 0;
    };
}

// Also export slot constants for games that want to read them directly
export var BS = {
    VIEWPORT_W: BS_VIEWPORT_W,
    VIEWPORT_H: BS_VIEWPORT_H,
    CANVAS_CSS_W: BS_CANVAS_CSS_W,
    CANVAS_CSS_H: BS_CANVAS_CSS_H,
    DEVICE_PIXEL_RATIO: BS_DEVICE_PIXEL_RATIO,
    IS_TOUCH_DEVICE: BS_IS_TOUCH_DEVICE,
    MAX_TOUCH_POINTS: BS_MAX_TOUCH_POINTS,
    ORIENTATION: BS_ORIENTATION,
    DOCUMENT_VISIBLE: BS_DOCUMENT_VISIBLE,
    DOCUMENT_FOCUSED: BS_DOCUMENT_FOCUSED,
    AUDIO_UNLOCKED: BS_AUDIO_UNLOCKED,
    JS_FPS: BS_JS_FPS,
    JS_FRAME_TIME_MS: BS_JS_FRAME_TIME_MS,
    ONLINE: BS_ONLINE,
    WALL_CLOCK_S: BS_WALL_CLOCK_S,
    SCROLL_Y: BS_SCROLL_Y
};
