use super::*;
use crate::engine::Engine;

// Minimal test functions for headless runner infrastructure.
// The S-League game has been removed; these tests verify the
// headless runner works with trivial inline functions.

fn noop_setup(engine: &mut Engine) {
    engine.global_state.set_f64("counter", 0.0);
}

fn noop_update(engine: &mut Engine, dt: f64) {
    let c = engine.global_state.get_f64("counter").unwrap_or(0.0);
    engine.global_state.set_f64("counter", c + dt);
}

fn noop_render(_engine: &mut Engine) {
}

// ─── HeadlessRunner basics ──────────────────────────────────────────────

#[test]
fn headless_runner_runs_frames() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(noop_setup, noop_update, noop_render, 60);
    assert_eq!(result.frames_run, 60);
    assert!(result.elapsed_sim_time > 0.9);
}

#[test]
fn headless_runner_game_state_populated() {
    let mut runner = HeadlessRunner::new(480, 720);
    let result = runner.run(noop_setup, noop_update, noop_render, 1);
    let counter = result.game_state.get("counter").and_then(|v| v.as_f64());
    assert!(counter.is_some(), "counter should be in game state");
}

#[test]
fn headless_runner_deterministic() {
    let run = || {
        let mut runner = HeadlessRunner::new(480, 720);
        runner.run(noop_setup, noop_update, noop_render, 30)
    };
    let r1 = run();
    let r2 = run();
    assert_eq!(r1.framebuffer_hash, r2.framebuffer_hash, "deterministic runs should produce identical framebuffers");
    assert_eq!(
        r1.game_state.get("counter").and_then(|v| v.as_f64()),
        r2.game_state.get("counter").and_then(|v| v.as_f64()),
    );
}
