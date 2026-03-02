import initWasm, {
    init, tick, framebuffer_ptr, framebuffer_len,
    key_down, key_up, drain_sound_commands,
    sleague_init, sleague_update, sleague_render,
    sleague_pointer_down, sleague_pointer_move, sleague_pointer_up
} from '../pkg/engine_core.js';

const WIDTH = 480;
const HEIGHT = 720;

// ─── Web Audio Sound Engine ──────────────────────────────────────────
let audioCtx = null;
let masterGain = null;
const activeLoops = {};

function ensureAudio() {
    if (audioCtx) return;
    audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    masterGain = audioCtx.createGain();
    masterGain.gain.value = 0.3;
    masterGain.connect(audioCtx.destination);
}

function playTone(freq, dur, vol, waveform, attack, decay) {
    ensureAudio();
    const osc = audioCtx.createOscillator();
    const gain = audioCtx.createGain();
    osc.type = waveform || 'square';
    osc.frequency.value = freq;
    gain.gain.setValueAtTime(0, audioCtx.currentTime);
    gain.gain.linearRampToValueAtTime(vol, audioCtx.currentTime + attack);
    gain.gain.linearRampToValueAtTime(0, audioCtx.currentTime + dur - decay);
    osc.connect(gain);
    gain.connect(masterGain);
    osc.start(audioCtx.currentTime);
    osc.stop(audioCtx.currentTime + dur);
}

function playNoise(dur, vol, filterFreq) {
    ensureAudio();
    const bufferSize = audioCtx.sampleRate * dur;
    const buffer = audioCtx.createBuffer(1, bufferSize, audioCtx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < bufferSize; i++) data[i] = Math.random() * 2 - 1;
    const source = audioCtx.createBufferSource();
    source.buffer = buffer;
    const filter = audioCtx.createBiquadFilter();
    filter.type = 'lowpass';
    filter.frequency.value = filterFreq;
    const gain = audioCtx.createGain();
    gain.gain.setValueAtTime(vol, audioCtx.currentTime);
    gain.gain.linearRampToValueAtTime(0, audioCtx.currentTime + dur);
    source.connect(filter);
    filter.connect(gain);
    gain.connect(masterGain);
    source.start();
}

function processSoundCommands() {
    const json = drain_sound_commands();
    if (!json || json === '[]') return;
    try {
        const cmds = JSON.parse(json);
        for (const cmd of cmds) {
            switch (cmd.type) {
                case 'PlayTone':
                    playTone(cmd.frequency, cmd.duration, cmd.volume, cmd.waveform, cmd.attack, cmd.decay);
                    break;
                case 'PlayNoise':
                    playNoise(cmd.duration, cmd.volume, cmd.filter_freq);
                    break;
                case 'StartLoop': {
                    ensureAudio();
                    if (activeLoops[cmd.id]) { activeLoops[cmd.id].stop(); delete activeLoops[cmd.id]; }
                    const osc = audioCtx.createOscillator();
                    const gain = audioCtx.createGain();
                    osc.type = cmd.waveform || 'sine';
                    osc.frequency.value = cmd.frequency;
                    gain.gain.value = cmd.volume;
                    osc.connect(gain);
                    gain.connect(masterGain);
                    osc.start();
                    activeLoops[cmd.id] = osc;
                    break;
                }
                case 'StopLoop': {
                    if (activeLoops[cmd.id]) {
                        activeLoops[cmd.id].stop();
                        delete activeLoops[cmd.id];
                    }
                    break;
                }
                case 'SetVolume':
                    if (masterGain) masterGain.gain.value = cmd.master_volume;
                    break;
            }
        }
    } catch (e) { /* ignore parse errors */ }
}

// ─── Main ────────────────────────────────────────────────────────────

async function main() {
    const wasm = await initWasm();
    init(WIDTH, HEIGHT);
    sleague_init();

    const canvas = document.getElementById('game');
    canvas.width = WIDTH;
    canvas.height = HEIGHT;
    const ctx = canvas.getContext('2d');

    // Scale canvas to fill viewport while maintaining aspect ratio
    function resize() {
        const vw = window.innerWidth;
        const vh = window.innerHeight;
        const aspect = WIDTH / HEIGHT;
        let cw, ch;

        if (vw / vh < aspect) {
            cw = vw;
            ch = vw / aspect;
        } else {
            ch = vh;
            cw = vh * aspect;
        }

        canvas.style.width = cw + 'px';
        canvas.style.height = ch + 'px';
    }
    resize();
    window.addEventListener('resize', resize);

    // Convert screen coordinates to game coordinates
    function gameCoords(clientX, clientY) {
        const rect = canvas.getBoundingClientRect();
        const scaleX = WIDTH / rect.width;
        const scaleY = HEIGHT / rect.height;
        return {
            x: (clientX - rect.left) * scaleX,
            y: (clientY - rect.top) * scaleY
        };
    }

    // Touch events
    canvas.addEventListener('touchstart', e => {
        e.preventDefault();
        ensureAudio(); // Unlock audio on first touch
        const t = e.touches[0];
        const g = gameCoords(t.clientX, t.clientY);
        sleague_pointer_down(g.x, g.y);
    }, { passive: false });

    canvas.addEventListener('touchmove', e => {
        e.preventDefault();
        const t = e.touches[0];
        const g = gameCoords(t.clientX, t.clientY);
        sleague_pointer_move(g.x, g.y);
    }, { passive: false });

    canvas.addEventListener('touchend', e => {
        e.preventDefault();
        const t = e.changedTouches[0];
        const g = gameCoords(t.clientX, t.clientY);
        sleague_pointer_up(g.x, g.y);
    }, { passive: false });

    // Mouse events (desktop)
    canvas.addEventListener('mousedown', e => {
        e.preventDefault();
        ensureAudio(); // Unlock audio on first click
        const g = gameCoords(e.clientX, e.clientY);
        sleague_pointer_down(g.x, g.y);
    });

    canvas.addEventListener('mousemove', e => {
        const g = gameCoords(e.clientX, e.clientY);
        sleague_pointer_move(g.x, g.y);
    });

    canvas.addEventListener('mouseup', e => {
        const g = gameCoords(e.clientX, e.clientY);
        sleague_pointer_up(g.x, g.y);
    });

    canvas.addEventListener('contextmenu', e => e.preventDefault());

    // Keyboard
    document.addEventListener('keydown', e => {
        key_down(e.code);
        e.preventDefault();
    });
    document.addEventListener('keyup', e => {
        key_up(e.code);
    });

    // Game loop
    let lastTime = performance.now();
    function frame(now) {
        const dt = Math.min(now - lastTime, 50);
        lastTime = now;

        sleague_update(dt);
        tick(dt);
        sleague_render();

        // Process sound commands from Rust
        processSoundCommands();

        // Copy framebuffer to canvas
        const ptr = framebuffer_ptr();
        const len = framebuffer_len();
        const pixels = new Uint8ClampedArray(wasm.memory.buffer, ptr, len);
        const imageData = new ImageData(pixels, WIDTH, HEIGHT);
        ctx.putImageData(imageData, 0, 0);

        requestAnimationFrame(frame);
    }
    requestAnimationFrame(frame);
}

main().catch(console.error);
