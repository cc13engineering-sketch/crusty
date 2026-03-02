import initWasm, {
    init, tick, framebuffer_ptr, framebuffer_len,
    key_down, key_up,
    sleague_init, sleague_update, sleague_render,
    sleague_pointer_down, sleague_pointer_move, sleague_pointer_up
} from '../pkg/engine_core.js';

const WIDTH = 480;
const HEIGHT = 720;

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

    // Keyboard (debug)
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
