# Engine Boundaries

Explicit separation rules to prevent platform bleed and hidden coupling.

## engine-core must NOT depend on:
- Windowing or display server APIs
- Graphics API calls (WebGL, WebGPU, OpenGL) — rendering is to a software framebuffer
- Filesystem specifics — all asset loading goes through byte slices or strings passed in
- Audio backends — sound commands are queued and drained by the host
- Network sockets or HTTP

## All host interaction goes through thin interfaces:
- **Input**: `Input` struct receives key/mouse/touch events from WASM bindings
- **Output**: `Framebuffer` pixel buffer read by JS via shared memory
- **Sound**: `SoundCommandQueue` drained as JSON by JS
- **Diagnostics**: `DiagnosticBus` drained as JSON by JS
- **State**: `GameState` read/written via WASM string APIs

## Why this matters
When engines become painful to port, it is almost always because this boundary eroded quietly. These rules ensure that adding a new platform (native, mobile, console) requires only a new host layer — not surgery on engine-core.

## Future platform trait layer (not yet implemented)
When needed, introduce traits like:
```rust
trait PlatformTime { fn now_seconds(&self) -> f64; }
trait PlatformInput { fn snapshot(&self) -> InputState; }
trait PlatformFs { fn load_bytes(&self, path: &str) -> Result<Vec<u8>>; }
```
Then route current implementations through the trait. This turns future ports from invasive refactors into new backend implementations.
