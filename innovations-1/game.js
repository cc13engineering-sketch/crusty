import initWasm, {
    init, tick, framebuffer_ptr, framebuffer_len,
    key_down, key_up, mouse_move, mouse_down, mouse_up,
    mycelia_init, mycelia_update, mycelia_render, mycelia_tap, mycelia_get_state
} from '../_pkg/engine_core.js';

// Game resolution (portrait, mobile-optimized)
const WIDTH = 480;
const HEIGHT = 720;

async function main() {
    const wasm = await initWasm();
    init(WIDTH, HEIGHT);

    // Initialize Mycelia with a random seed
    const seed = BigInt(Math.floor(Math.random() * 0xFFFFFFFF));
    mycelia_init(seed);

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

    // Input: convert screen coordinates to game coordinates
    function gameCoords(clientX, clientY) {
        const rect = canvas.getBoundingClientRect();
        const scaleX = WIDTH / rect.width;
        const scaleY = HEIGHT / rect.height;
        return {
            x: (clientX - rect.left) * scaleX,
            y: (clientY - rect.top) * scaleY
        };
    }

    // Tap handling (unified touch + mouse)
    let tapStart = null;
    const TAP_THRESHOLD = 15; // max movement to count as tap (in screen px)

    function onPointerDown(x, y) {
        tapStart = { x, y };
        const g = gameCoords(x, y);
        mouse_down(g.x, g.y, 0);
    }

    function onPointerMove(x, y) {
        const g = gameCoords(x, y);
        mouse_move(g.x, g.y);
    }

    function onPointerUp(x, y) {
        const g = gameCoords(x, y);
        mouse_up(g.x, g.y, 0);

        // Check if this was a tap (not a drag)
        if (tapStart) {
            const dx = x - tapStart.x;
            const dy = y - tapStart.y;
            if (Math.sqrt(dx * dx + dy * dy) < TAP_THRESHOLD) {
                const restart = mycelia_tap(g.x, g.y);
                if (restart) {
                    // Restart game with new seed
                    const newSeed = BigInt(Math.floor(Math.random() * 0xFFFFFFFF));
                    init(WIDTH, HEIGHT);
                    mycelia_init(newSeed);
                }
            }
        }
        tapStart = null;
    }

    // Touch events
    canvas.addEventListener('touchstart', e => {
        e.preventDefault();
        const t = e.touches[0];
        onPointerDown(t.clientX, t.clientY);
    }, { passive: false });

    canvas.addEventListener('touchmove', e => {
        e.preventDefault();
        const t = e.touches[0];
        onPointerMove(t.clientX, t.clientY);
    }, { passive: false });

    canvas.addEventListener('touchend', e => {
        e.preventDefault();
        const t = e.changedTouches[0];
        onPointerUp(t.clientX, t.clientY);
    }, { passive: false });

    // Mouse events (for desktop)
    canvas.addEventListener('mousedown', e => {
        e.preventDefault();
        onPointerDown(e.clientX, e.clientY);
    });

    canvas.addEventListener('mousemove', e => {
        onPointerMove(e.clientX, e.clientY);
    });

    canvas.addEventListener('mouseup', e => {
        onPointerUp(e.clientX, e.clientY);
    });

    canvas.addEventListener('contextmenu', e => e.preventDefault());

    // Keyboard (for debug)
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

        // Update game logic
        mycelia_update(dt);

        // Run engine systems (physics, environment clock, particles, etc.)
        tick(dt);

        // Custom render (tilemap, nodes, connections, blight, HUD)
        mycelia_render();

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
