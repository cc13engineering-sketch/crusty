import initWasm, {
    init, tick, framebuffer_ptr, framebuffer_len,
    key_down, key_up, mouse_move, mouse_down, mouse_up,
    load_world, get_schema
} from '../pkg/engine_core.js';

const WIDTH = 960;
const HEIGHT = 540;

async function main() {
    const wasm = await initWasm();
    init(WIDTH, HEIGHT);

    const resp = await fetch('space_survival.world');
    const source = await resp.text();
    load_world(source);

    const canvas = document.getElementById('game');
    const ctx = canvas.getContext('2d');

    document.addEventListener('keydown', e => {
        key_down(e.code);
        e.preventDefault();
    });
    document.addEventListener('keyup', e => {
        key_up(e.code);
    });

    const rect = () => canvas.getBoundingClientRect();
    canvas.addEventListener('mousemove', e => {
        const r = rect();
        mouse_move(e.clientX - r.left, e.clientY - r.top);
    });
    canvas.addEventListener('mousedown', e => {
        const r = rect();
        mouse_down(e.clientX - r.left, e.clientY - r.top, e.button);
        e.preventDefault();
    });
    canvas.addEventListener('mouseup', e => {
        const r = rect();
        mouse_up(e.clientX - r.left, e.clientY - r.top, e.button);
    });
    canvas.addEventListener('contextmenu', e => e.preventDefault());

    window.engine = { getSchema: () => JSON.parse(get_schema()) };

    let lastTime = performance.now();
    function frame(now) {
        const dt = Math.min(now - lastTime, 50);
        lastTime = now;

        tick(dt);

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
