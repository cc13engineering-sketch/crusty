//! demo_ball — minimal game validating the Simulation trait pipeline.
//!
//! A ball on a bounded surface. Tap/click to launch it in a direction.
//! Score = total distance traveled. Ball bounces off walls.
//! ~150 lines. Exercises: entities, physics, input, RNG, game state, rendering.

use crate::engine::Engine;
use crate::simulation::Simulation;
use crate::variant::ParamSet;
use crate::components::{Transform, Renderable, RigidBody, Collider, ColliderShape};
use crate::components::renderable::Visual;
use crate::rendering::color::Color;
use crate::ecs::Entity;

const BALL_RADIUS: f64 = 12.0;
const LAUNCH_SPEED: f64 = 200.0;
const FRICTION: f64 = 0.98;
const BOUNCE_DAMPING: f64 = 0.8;

/// Minimal demo game: a bouncing ball on a bounded surface.
#[derive(Clone)]
pub struct DemoBall {
    ball: Option<Entity>,
    prev_x: f64,
    prev_y: f64,
}

impl DemoBall {
    pub fn new() -> Self {
        Self {
            ball: None,
            prev_x: 0.0,
            prev_y: 0.0,
        }
    }
}

impl Simulation for DemoBall {
    fn setup(&mut self, engine: &mut Engine) {
        engine.config.bounds = (480.0, 270.0);
        engine.config.background = Color::from_rgba(20, 20, 40, 255);

        let ball = engine.world.spawn();
        let cx = engine.config.bounds.0 / 2.0;
        let cy = engine.config.bounds.1 / 2.0;

        engine.world.transforms.insert(ball, Transform {
            x: cx, y: cy, rotation: 0.0, scale: 1.0,
        });
        engine.world.rigidbodies.insert(ball, RigidBody {
            vx: 0.0, vy: 0.0, mass: 1.0,
            restitution: BOUNCE_DAMPING,
            ..RigidBody::default()
        });
        engine.world.colliders.insert(ball, Collider {
            shape: ColliderShape::Circle { radius: BALL_RADIUS },
            ..Collider::default()
        });
        engine.world.renderables.insert(ball, Renderable {
            visual: Visual::Circle {
                radius: BALL_RADIUS,
                color: Color::from_rgba(50, 200, 255, 255),
                filled: true,
            },
            layer: 0,
            visible: true,
        });

        self.ball = Some(ball);
        self.prev_x = cx;
        self.prev_y = cy;

        engine.global_state.set_f64("score", 0.0);
        engine.global_state.set_f64("launches", 0.0);
    }

    fn step(&mut self, engine: &mut Engine) {
        let ball = match self.ball {
            Some(b) => b,
            None => return,
        };

        // Read tunable parameters from global_state, falling back to constants
        let launch_speed = engine.global_state.get_f64("ball_speed").unwrap_or(LAUNCH_SPEED);
        let friction = engine.global_state.get_f64("ball_friction").unwrap_or(FRICTION);
        let bounce_damp = engine.global_state.get_f64("ball_bounce").unwrap_or(BOUNCE_DAMPING);

        // Handle tap/click: launch ball toward pointer
        if engine.input.mouse_buttons_pressed.contains(&0) {
            if let Some(t) = engine.world.transforms.get(ball) {
                let dx = engine.input.mouse_x - t.x;
                let dy = engine.input.mouse_y - t.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 1.0 {
                    let nx = dx / dist;
                    let ny = dy / dist;
                    // Add a small random perturbation for variety
                    let jitter = (engine.rng.next_f64() - 0.5) * 20.0;
                    if let Some(rb) = engine.world.rigidbodies.get_mut(ball) {
                        rb.vx = nx * launch_speed + jitter;
                        rb.vy = ny * launch_speed + jitter;
                    }
                    let launches = engine.global_state.get_f64("launches").unwrap_or(0.0);
                    engine.global_state.set_f64("launches", launches + 1.0);
                }
            }
        }

        // Apply friction
        if let Some(rb) = engine.world.rigidbodies.get_mut(ball) {
            rb.vx *= friction;
            rb.vy *= friction;
            // Stop tiny velocities
            if rb.vx.abs() < 0.1 && rb.vy.abs() < 0.1 {
                rb.vx = 0.0;
                rb.vy = 0.0;
            }
        }

        // Manual wall bounce (engine physics handles collider-collider,
        // but we need boundary bouncing)
        let (bw, bh) = engine.config.bounds;
        if let Some(t) = engine.world.transforms.get(ball) {
            let mut nx = t.x;
            let mut ny = t.y;
            let mut bounce_x = false;
            let mut bounce_y = false;

            if nx - BALL_RADIUS < 0.0 { nx = BALL_RADIUS; bounce_x = true; }
            if nx + BALL_RADIUS > bw { nx = bw - BALL_RADIUS; bounce_x = true; }
            if ny - BALL_RADIUS < 0.0 { ny = BALL_RADIUS; bounce_y = true; }
            if ny + BALL_RADIUS > bh { ny = bh - BALL_RADIUS; bounce_y = true; }

            if bounce_x || bounce_y {
                if let Some(rb) = engine.world.rigidbodies.get_mut(ball) {
                    if bounce_x { rb.vx = -rb.vx * bounce_damp; }
                    if bounce_y { rb.vy = -rb.vy * bounce_damp; }
                }
                if let Some(t) = engine.world.transforms.get_mut(ball) {
                    t.x = nx;
                    t.y = ny;
                }
            }
        }

        // Track distance for scoring
        if let Some(t) = engine.world.transforms.get(ball) {
            let dx = t.x - self.prev_x;
            let dy = t.y - self.prev_y;
            let dist = (dx * dx + dy * dy).sqrt();
            let score = engine.global_state.get_f64("score").unwrap_or(0.0);
            engine.global_state.set_f64("score", score + dist);
            self.prev_x = t.x;
            self.prev_y = t.y;
        }
    }

    fn render(&self, _engine: &mut Engine) {
        // Entity rendering is handled by the engine's renderer system in tick().
        // Nothing extra needed for this simple demo.
    }

    fn variants(&self) -> Vec<ParamSet> {
        vec![
            ParamSet::new()
                .named("default"),
            ParamSet::new()
                .named("fast")
                .with("ball_speed", 400.0)
                .with("ball_friction", 0.99)
                .with("ball_bounce", 0.95),
            ParamSet::new()
                .named("slow")
                .with("ball_speed", 80.0)
                .with("ball_friction", 0.90)
                .with("ball_bounce", 0.5),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headless::{HeadlessRunner, RunConfig};
    use crate::input_frame::InputFrame;

    #[test]
    fn demo_ball_deterministic() {
        let inputs = vec![
            InputFrame::default(), // frame 0: idle
            InputFrame::default(), // frame 1: idle
            InputFrame { pointer_down: Some((300.0, 100.0)), ..Default::default() },
            InputFrame::default(),
        ];

        let mut r1 = HeadlessRunner::new(480, 270);
        let mut g1 = DemoBall::new();
        let res1 = r1.run_sim(&mut g1, 42, &inputs, RunConfig::default());

        let mut r2 = HeadlessRunner::new(480, 270);
        let mut g2 = DemoBall::new();
        let res2 = r2.run_sim(&mut g2, 42, &inputs, RunConfig::default());

        assert_eq!(res1.state_hash, res2.state_hash,
            "same seed + same inputs must produce identical state hash");
    }

    #[test]
    fn demo_ball_different_seeds_diverge() {
        let inputs: Vec<InputFrame> = (0..60).map(|_| InputFrame::default()).collect();

        let mut r1 = HeadlessRunner::new(480, 270);
        let mut g1 = DemoBall::new();
        let res1 = r1.run_sim(&mut g1, 1, &inputs, RunConfig::default());

        let mut r2 = HeadlessRunner::new(480, 270);
        let mut g2 = DemoBall::new();
        let res2 = r2.run_sim(&mut g2, 2, &inputs, RunConfig::default());

        // Different seeds → different RNG state → different state hashes
        assert_ne!(res1.state_hash, res2.state_hash);
    }

    #[test]
    fn demo_ball_tap_launches_ball() {
        let mut inputs: Vec<InputFrame> = vec![InputFrame::default(); 5];
        // Tap at frame 5
        inputs.push(InputFrame {
            pointer_down: Some((400.0, 50.0)),
            ..Default::default()
        });
        // Let it run
        for _ in 0..60 {
            inputs.push(InputFrame::default());
        }

        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();
        let result = runner.run_sim(&mut game, 42, &inputs, RunConfig::default());

        let score = result.get_f64("score").unwrap_or(0.0);
        assert!(score > 0.0, "ball should move after tap, score={}", score);

        let launches = result.get_f64("launches").unwrap_or(0.0);
        assert_eq!(launches, 1.0, "should record one launch");
    }

    #[test]
    fn demo_ball_turbo_mode_faster() {
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); 600];

        let mut r1 = HeadlessRunner::new(480, 270);
        let mut g1 = DemoBall::new();
        let res1 = r1.run_sim(&mut g1, 42, &inputs, RunConfig { turbo: true, capture_state_hashes: false });

        let mut r2 = HeadlessRunner::new(480, 270);
        let mut g2 = DemoBall::new();
        let res2 = r2.run_sim(&mut g2, 42, &inputs, RunConfig::default());

        // State hashes should be identical (turbo only skips render)
        assert_eq!(res1.state_hash, res2.state_hash,
            "turbo and normal mode must produce identical state hashes");
    }

    #[test]
    fn demo_ball_state_hash_capture() {
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); 10];

        let mut runner = HeadlessRunner::new(480, 270);
        let mut game = DemoBall::new();
        let result = runner.run_sim(&mut game, 42, &inputs, RunConfig {
            turbo: false,
            capture_state_hashes: true,
        });

        assert_eq!(result.state_hashes.len(), 10);
        // Each frame should have a different hash (frame counter advances)
        let unique: std::collections::HashSet<u64> = result.state_hashes.iter().copied().collect();
        assert_eq!(unique.len(), 10, "each frame should have a unique state hash");
    }
}
