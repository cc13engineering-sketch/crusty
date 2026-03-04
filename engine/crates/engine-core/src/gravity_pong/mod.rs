//! Gravity Pong -- a physics puzzle game for the Crusty engine.
//!
//! Players guide particles into targets using gravity wells, repulsors,
//! waypoints, and slingshot mechanics. All coordinates live in a 0-1000
//! normalised world space, scaled uniformly to the viewport.

use crate::engine::Engine;
use crate::rendering::color::Color;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::post_fx;
use crate::rendering::shapes;
use crate::rendering::text;
use crate::simulation::Simulation;

// ─── Constants ──────────────────────────────────────────────────────────────

const FIXED_DT: f64 = 1.0 / 60.0;

// Gravity wells
const WELL_MIN_STRENGTH: f64 = 900.0;
const WELL_MAX_STRENGTH: f64 = 7200.0;
const WELL_EPSILON_FACTOR: f64 = 0.7;
const WELL_CIRC_RATE: f64 = 0.03;

// Black holes
const BLACK_HOLE_GRAVITY: f64 = 10800.0;
const BLACK_HOLE_EPSILON_FACTOR: f64 = 1.2;
const BLACK_HOLE_CIRC_RATE: f64 = 0.08;

// Repulsors
const REPULSOR_MIN_STRENGTH: f64 = 900.0;
const REPULSOR_MAX_STRENGTH: f64 = 7200.0;
const REPULSOR_EPSILON_FACTOR: f64 = 0.5;

// Drag
const BASE_DRAG: f64 = 0.5;
const SPEED_DRAG: f64 = 0.03;
const REST_THRESHOLD: f64 = 0.3;

// Physics
const EDGE_RESTITUTION: f64 = 0.8;
const MAX_SPEED: f64 = 8.0;
const MAX_SUBSTEPS: u32 = 8;
const MAX_BOUNCES: u32 = 5;

// Waypoint
const WAYPOINT_CAPTURE_RADIUS: f64 = 200.0;
const WAYPOINT_TRAVEL_SPEED: f64 = 3.0;
const WAYPOINT_TIMEOUT_FRAMES: u32 = 180;

// Sling
const SLING_MAX_SPEED: f64 = 300.0;
const SLING_MAX_PULL: f64 = 150.0;
const SLING_DECAY_RATE: f64 = 0.4;

// Visual
const FIELD_DUST_COUNT: usize = 300;
const FIELD_DUST_DAMPING: f64 = 0.85;
const PARTICLE_RADIUS: f64 = 5.0;

// Trail
const MAX_TRAIL_LEN: usize = 30;

// World size
const WORLD_SIZE: f64 = 1000.0;

// Particle collision radius in world units
const PARTICLE_WORLD_RADIUS: f64 = 8.0;

// ─── Entity Types ───────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    alive: bool,
    scored: bool,
    color: Color,
    captured: bool,
    locked: bool,
    sling_immunity: f64,
    sling_decay_rate: f64,
    sling_launch_speed: f64,
    trail: Vec<(f64, f64)>,
    scale: f64,
    flash_timer: f64,
    attractor_count: u32,
}

impl Particle {
    fn new(x: f64, y: f64, vx: f64, vy: f64, color: Color) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            alive: true,
            scored: false,
            color,
            captured: false,
            locked: false,
            sling_immunity: 0.0,
            sling_decay_rate: SLING_DECAY_RATE,
            sling_launch_speed: 0.0,
            trail: Vec::with_capacity(MAX_TRAIL_LEN),
            scale: 1.0,
            flash_timer: 0.0,
            attractor_count: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct GravityWell {
    x: f64,
    y: f64,
    strength: f64,
    gm: f64,
    epsilon: f64,
    visual_radius: f64,
    active: bool,
    anim_phase: f64,
}

impl GravityWell {
    fn from_params(x: f64, y: f64, strength: f64) -> Self {
        let gm = WELL_MIN_STRENGTH + (WELL_MAX_STRENGTH - WELL_MIN_STRENGTH) * (strength - 1.0) / 99.0;
        let visual_radius = 20.0 + strength * 0.3;
        let epsilon = visual_radius * WELL_EPSILON_FACTOR;
        Self {
            x,
            y,
            strength,
            gm,
            epsilon,
            visual_radius,
            active: true,
            anim_phase: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct Repulsor {
    x: f64,
    y: f64,
    strength: f64,
    gm: f64,
    epsilon: f64,
    visual_radius: f64,
    active: bool,
    anim_phase: f64,
}

impl Repulsor {
    fn from_params(x: f64, y: f64, strength: f64) -> Self {
        let gm = REPULSOR_MIN_STRENGTH
            + (REPULSOR_MAX_STRENGTH - REPULSOR_MIN_STRENGTH) * (strength - 1.0) / 99.0;
        let visual_radius = 18.0 + strength * 0.25;
        let epsilon = visual_radius * REPULSOR_EPSILON_FACTOR;
        Self {
            x,
            y,
            strength,
            gm,
            epsilon,
            visual_radius,
            active: true,
            anim_phase: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct BlackHole {
    x: f64,
    y: f64,
    horizon: f64,
    gm: f64,
    epsilon: f64,
    kill_radius: f64,
    visual_radius: f64,
    active: bool,
    anim_phase: f64,
    mote_angles: Vec<f64>,
}

impl BlackHole {
    fn from_params(x: f64, y: f64, horizon: f64) -> Self {
        let visual_radius = 15.0 + horizon * 0.35;
        let kill_radius = visual_radius * 0.6;
        let epsilon = visual_radius * BLACK_HOLE_EPSILON_FACTOR;
        let mut mote_angles = Vec::with_capacity(8);
        for i in 0..8 {
            mote_angles.push(i as f64 * std::f64::consts::PI * 2.0 / 8.0);
        }
        Self {
            x,
            y,
            horizon,
            gm: BLACK_HOLE_GRAVITY,
            epsilon,
            kill_radius,
            visual_radius,
            active: true,
            anim_phase: 0.0,
            mote_angles,
        }
    }
}

#[derive(Clone, Debug)]
struct Target {
    x: f64,
    y: f64,
    size: f64,
    hit_radius: f64,
    visual_radius: f64,
    active: bool,
    scored_count: u32,
    anim_phase: f64,
    flash_timer: f64,
}

impl Target {
    fn from_params(x: f64, y: f64, size: f64) -> Self {
        let visual_radius = 15.0 + size * 0.35;
        let hit_radius = visual_radius * 0.8;
        Self {
            x,
            y,
            size,
            hit_radius,
            visual_radius,
            active: true,
            scored_count: 0,
            anim_phase: 0.0,
            flash_timer: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct Wall {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    restitution: f64,
    is_boost: bool,
    flash_timer: f64,
}

impl Wall {
    fn from_params(x1: f64, y1: f64, x2: f64, y2: f64, restitution_pct: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            restitution: restitution_pct / 100.0,
            is_boost: false,
            flash_timer: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct Waypoint {
    x: f64,
    y: f64,
    remaining_frames: u32,
    captured_ids: Vec<usize>,
}

#[derive(Clone, Debug)]
struct DustMote {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    lifetime: f64,
    age: f64,
    alpha: f64,
}

#[derive(Clone, Debug)]
enum VisualEffect {
    Burst {
        x: f64,
        y: f64,
        particles: Vec<(f64, f64, f64, f64)>, // (x, y, vx, vy)
        color: Color,
        age: f64,
        duration: f64,
    },
    Flash {
        age: f64,
        duration: f64,
        color: Color,
    },
}

#[derive(Clone, Debug)]
struct SlingDrag {
    particle_idx: usize,
    anchor_x: f64,
    anchor_y: f64,
    pull_x: f64,
    pull_y: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum GamePhase {
    Playing,
    Won,
    Lost,
    LevelTransition(f64),
}

// ─── Level Data ─────────────────────────────────────────────────────────────

fn level_data() -> Vec<Vec<&'static str>> {
    vec![
        // Level 1 - Tutorial
        vec![
            "target:500:200:70",
            "gravity-well:500:550:40",
            "particle:350:850",
            "particle:500:850",
            "particle:650:850",
        ],
        // Level 2 - Wall Barrier
        vec![
            "target:500:150:60",
            "gravity-well:500:500:55",
            "wall:200:350:800:350:80",
            "particle:300:850",
            "particle:400:850",
            "particle:500:850",
            "particle:600:850",
            "particle:700:850",
        ],
        // Level 3 - Danger Zone
        vec![
            "target:500:100:50",
            "gravity-well:300:450:50",
            "gravity-well:700:450:50",
            "black-hole:500:300:40",
            "wall:100:200:100:800:80",
            "wall:900:200:900:800:80",
            "particle:250:850",
            "particle:400:850",
            "particle:550:850",
            "particle:700:850",
            "particle:850:850",
        ],
    ]
}

// ─── Main Struct ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct GravityPong {
    particles: Vec<Particle>,
    gravity_wells: Vec<GravityWell>,
    repulsors: Vec<Repulsor>,
    black_holes: Vec<BlackHole>,
    targets: Vec<Target>,
    walls: Vec<Wall>,
    waypoint: Option<Waypoint>,
    sling: Option<SlingDrag>,
    dust_motes: Vec<DustMote>,
    effects: Vec<VisualEffect>,
    phase: GamePhase,
    total_scored: u32,
    goal_target: u32,
    alive_count: u32,
    current_level: usize,
    scale: f64,
    mouse_down_frame: Option<u64>,
    mouse_down_pos: Option<(f64, f64)>,
    screen_w: f64,
    screen_h: f64,
}

impl GravityPong {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            gravity_wells: Vec::new(),
            repulsors: Vec::new(),
            black_holes: Vec::new(),
            targets: Vec::new(),
            walls: Vec::new(),
            waypoint: None,
            sling: None,
            dust_motes: Vec::new(),
            effects: Vec::new(),
            phase: GamePhase::Playing,
            total_scored: 0,
            goal_target: 0,
            alive_count: 0,
            current_level: 0,
            scale: 0.64,
            mouse_down_frame: None,
            mouse_down_pos: None,
            screen_w: 640.0,
            screen_h: 640.0,
        }
    }

    // ─── Coordinate helpers ─────────────────────────────────────────────

    fn w2s(&self, wx: f64, wy: f64) -> (f64, f64) {
        (wx * self.scale, wy * self.scale)
    }

    fn s2w(&self, sx: f64, sy: f64) -> (f64, f64) {
        if self.scale > 0.0 {
            (sx / self.scale, sy / self.scale)
        } else {
            (sx, sy)
        }
    }

    fn w2s_r(&self, wr: f64) -> f64 {
        wr * self.scale
    }

    // ─── Level loading ──────────────────────────────────────────────────

    fn load_level(&mut self, level_idx: usize, engine: &mut Engine) {
        self.particles.clear();
        self.gravity_wells.clear();
        self.repulsors.clear();
        self.black_holes.clear();
        self.targets.clear();
        self.walls.clear();
        self.waypoint = None;
        self.sling = None;
        self.effects.clear();
        self.total_scored = 0;
        self.alive_count = 0;
        self.phase = GamePhase::Playing;
        self.mouse_down_frame = None;
        self.mouse_down_pos = None;

        let levels = level_data();
        let clamped_idx = if level_idx < levels.len() { level_idx } else { levels.len() - 1 };
        self.current_level = clamped_idx;

        if let Some(level_lines) = levels.get(clamped_idx) {
            let mut particle_idx = 0u32;
            for line in level_lines {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.is_empty() {
                    continue;
                }
                match parts[0] {
                    "target" => {
                        if parts.len() >= 4 {
                            let x = parse_f64(parts[1]);
                            let y = parse_f64(parts[2]);
                            let size = parse_f64(parts[3]);
                            self.targets.push(Target::from_params(x, y, size));
                        }
                    }
                    "gravity-well" => {
                        if parts.len() >= 4 {
                            let x = parse_f64(parts[1]);
                            let y = parse_f64(parts[2]);
                            let strength = parse_f64(parts[3]);
                            self.gravity_wells.push(GravityWell::from_params(x, y, strength));
                        }
                    }
                    "repulsor" => {
                        if parts.len() >= 4 {
                            let x = parse_f64(parts[1]);
                            let y = parse_f64(parts[2]);
                            let strength = parse_f64(parts[3]);
                            self.repulsors.push(Repulsor::from_params(x, y, strength));
                        }
                    }
                    "black-hole" => {
                        if parts.len() >= 4 {
                            let x = parse_f64(parts[1]);
                            let y = parse_f64(parts[2]);
                            let horizon = parse_f64(parts[3]);
                            self.black_holes.push(BlackHole::from_params(x, y, horizon));
                        }
                    }
                    "wall" => {
                        if parts.len() >= 6 {
                            let x1 = parse_f64(parts[1]);
                            let y1 = parse_f64(parts[2]);
                            let x2 = parse_f64(parts[3]);
                            let y2 = parse_f64(parts[4]);
                            let rest = parse_f64(parts[5]);
                            self.walls.push(Wall::from_params(x1, y1, x2, y2, rest));
                        }
                    }
                    "particle" => {
                        let x = if parts.len() > 1 { parse_f64(parts[1]) } else { 500.0 };
                        let y = if parts.len() > 2 { parse_f64(parts[2]) } else { 500.0 };
                        let vx = if parts.len() > 3 { parse_f64(parts[3]) } else { 0.0 };
                        let vy = if parts.len() > 4 { parse_f64(parts[4]) } else { 0.0 };
                        let color = particle_color(particle_idx);
                        self.particles.push(Particle::new(x, y, vx, vy, color));
                        particle_idx += 1;
                    }
                    _ => {}
                }
            }
        }

        self.alive_count = self.particles.iter().filter(|p| p.alive && !p.scored).count() as u32;
        // Goal: get all particles into targets
        self.goal_target = self.alive_count;

        // Initialise dust motes
        self.init_dust(engine);
    }

    fn init_dust(&mut self, engine: &mut Engine) {
        self.dust_motes.clear();
        for _ in 0..FIELD_DUST_COUNT {
            let x = engine.rng.range_f64(0.0, WORLD_SIZE);
            let y = engine.rng.range_f64(0.0, WORLD_SIZE);
            let lifetime = engine.rng.range_f64(2.0, 6.0);
            let alpha = engine.rng.range_f64(40.0, 60.0);
            self.dust_motes.push(DustMote {
                x,
                y,
                vx: 0.0,
                vy: 0.0,
                lifetime,
                age: engine.rng.range_f64(0.0, lifetime),
                alpha,
            });
        }
    }

    // ─── Spawn visual effects ───────────────────────────────────────────

    fn spawn_burst(&mut self, x: f64, y: f64, color: Color, count: usize, engine: &mut Engine) {
        let mut burst_particles = Vec::with_capacity(count);
        for _ in 0..count {
            let angle = engine.rng.range_f64(0.0, std::f64::consts::TAU);
            let speed = engine.rng.range_f64(30.0, 120.0);
            burst_particles.push((x, y, angle.cos() * speed, angle.sin() * speed));
        }
        self.effects.push(VisualEffect::Burst {
            x,
            y,
            particles: burst_particles,
            color,
            age: 0.0,
            duration: 0.6,
        });
    }

    fn spawn_flash(&mut self, color: Color, duration: f64) {
        self.effects.push(VisualEffect::Flash {
            age: 0.0,
            duration,
            color,
        });
    }

    // ─── Input handling ─────────────────────────────────────────────────

    fn handle_input(&mut self, engine: &mut Engine) {
        let frame = engine.frame;

        // Mouse pressed this frame
        if engine.input.mouse_buttons_pressed.contains(&0) {
            self.mouse_down_frame = Some(frame);
            self.mouse_down_pos = Some((engine.input.mouse_x, engine.input.mouse_y));

            // Check if clicking on a locked particle to start sling
            let (mx_w, my_w) = self.s2w(engine.input.mouse_x, engine.input.mouse_y);
            let mut closest_locked: Option<(usize, f64)> = None;
            for (i, p) in self.particles.iter().enumerate() {
                if !p.alive || p.scored || !p.locked {
                    continue;
                }
                let dx = p.x - mx_w;
                let dy = p.y - my_w;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 30.0 {
                    match closest_locked {
                        Some((_, best_d)) if dist < best_d => {
                            closest_locked = Some((i, dist));
                        }
                        None => {
                            closest_locked = Some((i, dist));
                        }
                        _ => {}
                    }
                }
            }

            if let Some((idx, _)) = closest_locked {
                let p = &self.particles[idx];
                self.sling = Some(SlingDrag {
                    particle_idx: idx,
                    anchor_x: p.x,
                    anchor_y: p.y,
                    pull_x: p.x,
                    pull_y: p.y,
                });
            }
        }

        // Mouse held - update sling drag
        if engine.input.mouse_buttons_held.contains(&0) {
            let scale = self.scale;
            let mouse_world = if scale > 0.0 {
                (engine.input.mouse_x / scale, engine.input.mouse_y / scale)
            } else {
                (engine.input.mouse_x, engine.input.mouse_y)
            };
            if let Some(ref mut sling) = self.sling {
                let (mx_w, my_w) = mouse_world;
                let dx = mx_w - sling.anchor_x;
                let dy = my_w - sling.anchor_y;
                let dist = (dx * dx + dy * dy).sqrt();
                let max_pull_w = SLING_MAX_PULL / self.scale;
                if dist > max_pull_w && dist > 0.0 {
                    sling.pull_x = sling.anchor_x + dx / dist * max_pull_w;
                    sling.pull_y = sling.anchor_y + dy / dist * max_pull_w;
                } else {
                    sling.pull_x = mx_w;
                    sling.pull_y = my_w;
                }
            }
        }

        // Mouse released this frame
        if engine.input.mouse_buttons_released.contains(&0) {
            // Check for sling launch
            if let Some(sling) = self.sling.take() {
                let dx = sling.anchor_x - sling.pull_x;
                let dy = sling.anchor_y - sling.pull_y;
                let pull_dist = (dx * dx + dy * dy).sqrt();
                if pull_dist > 5.0 {
                    // Launch the particle
                    let max_pull_w = SLING_MAX_PULL / self.scale;
                    let power = (pull_dist / max_pull_w).min(1.0);
                    let launch_speed = SLING_MAX_SPEED * power;
                    let len = pull_dist;
                    if len > 0.0 {
                        if let Some(p) = self.particles.get_mut(sling.particle_idx) {
                            p.vx = dx / len * launch_speed;
                            p.vy = dy / len * launch_speed;
                            p.locked = false;
                            p.captured = false;
                            p.sling_immunity = 1.0;
                            p.sling_launch_speed = launch_speed;
                        }
                    }
                }
            } else {
                // Quick tap -> place waypoint
                let held_frames = match self.mouse_down_frame {
                    Some(df) => frame.saturating_sub(df),
                    None => 0,
                };
                if held_frames < 10 {
                    let (mx_w, my_w) = self.s2w(engine.input.mouse_x, engine.input.mouse_y);
                    // Only place waypoint if within world bounds
                    if mx_w >= 0.0 && mx_w <= WORLD_SIZE && my_w >= 0.0 && my_w <= WORLD_SIZE {
                        self.waypoint = Some(Waypoint {
                            x: mx_w,
                            y: my_w,
                            remaining_frames: WAYPOINT_TIMEOUT_FRAMES,
                            captured_ids: Vec::new(),
                        });
                    }
                }
            }

            self.mouse_down_frame = None;
            self.mouse_down_pos = None;
        }
    }

    // ─── Waypoint system ────────────────────────────────────────────────

    fn update_waypoint(&mut self) {
        let should_remove = if let Some(ref mut wp) = self.waypoint {
            if wp.remaining_frames == 0 {
                true
            } else {
                wp.remaining_frames = wp.remaining_frames.saturating_sub(1);

                // Capture nearby non-locked particles
                let capture_r = WAYPOINT_CAPTURE_RADIUS;
                let capture_r_sq = capture_r * capture_r;
                for i in 0..self.particles.len() {
                    let p = &self.particles[i];
                    if !p.alive || p.scored || p.locked {
                        continue;
                    }
                    let dx = p.x - wp.x;
                    let dy = p.y - wp.y;
                    if dx * dx + dy * dy < capture_r_sq && !wp.captured_ids.contains(&i) {
                        wp.captured_ids.push(i);
                    }
                }

                // Move captured particles toward waypoint
                let speed = WAYPOINT_TRAVEL_SPEED;
                let wp_x = wp.x;
                let wp_y = wp.y;
                let mut newly_locked = Vec::new();
                for &idx in &wp.captured_ids {
                    if let Some(p) = self.particles.get_mut(idx) {
                        if p.locked || p.scored || !p.alive {
                            continue;
                        }
                        p.captured = true;
                        let dx = wp_x - p.x;
                        let dy = wp_y - p.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < speed * 2.0 {
                            // Arrived
                            p.x = wp_x;
                            p.y = wp_y;
                            p.vx = 0.0;
                            p.vy = 0.0;
                            newly_locked.push(idx);
                        } else if dist > 0.0 {
                            p.vx = dx / dist * speed;
                            p.vy = dy / dist * speed;
                            p.x += p.vx;
                            p.y += p.vy;
                        }
                    }
                }
                for idx in newly_locked {
                    if let Some(p) = self.particles.get_mut(idx) {
                        p.locked = true;
                    }
                }

                // Check if waypoint expired
                wp.remaining_frames == 0
            }
        } else {
            false
        };

        if should_remove {
            // Release all captured particles
            if let Some(wp) = self.waypoint.take() {
                for &idx in &wp.captured_ids {
                    if let Some(p) = self.particles.get_mut(idx) {
                        p.captured = false;
                        p.locked = false;
                    }
                }
            }
        }
    }

    // ─── Physics ────────────────────────────────────────────────────────

    fn update_physics(&mut self, engine: &mut Engine) {
        let num_particles = self.particles.len();

        for i in 0..num_particles {
            // Skip particles that are not active for physics
            {
                let p = &self.particles[i];
                if !p.alive || p.scored || p.captured {
                    continue;
                }
            }

            // Accumulate acceleration from gravity fields
            let mut ax = 0.0_f64;
            let mut ay = 0.0_f64;

            let px;
            let py;
            let immunity;
            {
                let p = &self.particles[i];
                px = p.x;
                py = p.y;
                immunity = p.sling_immunity;
            }

            // Gravity wells
            for well in &self.gravity_wells {
                if !well.active {
                    continue;
                }
                let (fax, fay) = plummer_force(well.x, well.y, well.gm, well.epsilon, px, py);
                ax += fax;
                ay += fay;
            }

            // Repulsors (negate direction)
            for rep in &self.repulsors {
                if !rep.active {
                    continue;
                }
                let (fax, fay) = plummer_force(rep.x, rep.y, rep.gm, rep.epsilon, px, py);
                ax -= fax;
                ay -= fay;
            }

            // Black holes
            for bh in &self.black_holes {
                if !bh.active {
                    continue;
                }
                let (fax, fay) = plummer_force(bh.x, bh.y, bh.gm, bh.epsilon, px, py);
                ax += fax;
                ay += fay;
            }

            // Apply sling immunity (dampens field forces)
            let field_factor = 1.0 - immunity;
            ax *= field_factor;
            ay *= field_factor;

            // Determine substeps
            let max_accel = (ax * ax + ay * ay).sqrt();
            let substeps = ((max_accel / 50.0).ceil() as u32).clamp(1, MAX_SUBSTEPS);
            let sub_dt = FIXED_DT / substeps as f64;

            // Substep integration
            let p = &mut self.particles[i];

            for _sub in 0..substeps {
                // Velocity integration
                p.vx += ax * sub_dt;
                p.vy += ay * sub_dt;

                // Apply drag
                let speed = (p.vx * p.vx + p.vy * p.vy).sqrt();
                let effective_drag = BASE_DRAG + SPEED_DRAG * speed;
                let factor = (-effective_drag * sub_dt).exp();
                p.vx *= factor;
                p.vy *= factor;
                if speed * factor < REST_THRESHOLD {
                    p.vx = 0.0;
                    p.vy = 0.0;
                }

                // Clamp speed
                let speed_after = (p.vx * p.vx + p.vy * p.vy).sqrt();
                let max_speed_w = MAX_SPEED / self.scale;
                if speed_after > max_speed_w && speed_after > 0.0 {
                    let s = max_speed_w / speed_after;
                    p.vx *= s;
                    p.vy *= s;
                }

                // Position integration
                p.x += p.vx * sub_dt;
                p.y += p.vy * sub_dt;

                // Edge bouncing
                for _ in 0..MAX_BOUNCES {
                    let mut bounced = false;
                    if p.x < 0.0 {
                        p.x = -p.x;
                        p.vx = -p.vx * EDGE_RESTITUTION;
                        bounced = true;
                    }
                    if p.x > WORLD_SIZE {
                        p.x = 2.0 * WORLD_SIZE - p.x;
                        p.vx = -p.vx * EDGE_RESTITUTION;
                        bounced = true;
                    }
                    if p.y < 0.0 {
                        p.y = -p.y;
                        p.vy = -p.vy * EDGE_RESTITUTION;
                        bounced = true;
                    }
                    if p.y > WORLD_SIZE {
                        p.y = 2.0 * WORLD_SIZE - p.y;
                        p.vy = -p.vy * EDGE_RESTITUTION;
                        bounced = true;
                    }
                    if !bounced {
                        break;
                    }
                }
                // Final clamp to ensure within bounds
                p.x = p.x.clamp(0.0, WORLD_SIZE);
                p.y = p.y.clamp(0.0, WORLD_SIZE);
            }

            // Wall collision (done once after full substeps)
            let p = &mut self.particles[i];
            for wall in &self.walls {
                let (collided, new_vx, new_vy, new_x, new_y) =
                    wall_collision(p.x, p.y, p.vx, p.vy, PARTICLE_WORLD_RADIUS, wall);
                if collided {
                    p.x = new_x;
                    p.y = new_y;
                    p.vx = new_vx;
                    p.vy = new_vy;
                }
            }

            // Record trail position
            let p = &mut self.particles[i];
            p.trail.push((p.x, p.y));
            if p.trail.len() > MAX_TRAIL_LEN {
                p.trail.remove(0);
            }
        }

        // Target collision (separate pass to avoid borrow issues)
        for i in 0..num_particles {
            let mut hit_target = false;
            let mut burst_x = 0.0;
            let mut burst_y = 0.0;
            let mut burst_color = Color::GREEN;

            {
                let p = &self.particles[i];
                if !p.alive || p.scored || p.captured {
                    continue;
                }
                for target in &self.targets {
                    if !target.active {
                        continue;
                    }
                    let dx = p.x - target.x;
                    let dy = p.y - target.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < target.hit_radius {
                        hit_target = true;
                        burst_x = p.x;
                        burst_y = p.y;
                        burst_color = p.color;
                        break;
                    }
                }
            }

            if hit_target {
                let p = &mut self.particles[i];
                p.scored = true;
                p.alive = false;
                self.total_scored += 1;
                self.spawn_burst(burst_x, burst_y, burst_color, 20, engine);

                // Flash targets
                for target in &mut self.targets {
                    target.flash_timer = 0.3;
                }
            }
        }

        // Black hole kill zone (separate pass)
        for i in 0..num_particles {
            let mut killed = false;
            let mut kill_x = 0.0;
            let mut kill_y = 0.0;

            {
                let p = &self.particles[i];
                if !p.alive || p.scored || p.captured {
                    continue;
                }
                for bh in &self.black_holes {
                    if !bh.active {
                        continue;
                    }
                    let dx = p.x - bh.x;
                    let dy = p.y - bh.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < bh.kill_radius {
                        killed = true;
                        kill_x = p.x;
                        kill_y = p.y;
                        break;
                    }
                }
            }

            if killed {
                let p = &mut self.particles[i];
                p.alive = false;
                self.spawn_burst(kill_x, kill_y, Color::from_rgba(255, 100, 50, 255), 15, engine);

                // Screen shake
                engine.post_fx.shake_remaining = 0.2;
                engine.post_fx.shake_intensity = 4.0;
            }
        }

        // Sling immunity decay
        for p in &mut self.particles {
            if p.sling_immunity > 0.0 {
                p.sling_immunity -= p.sling_decay_rate * FIXED_DT;
                if p.sling_immunity < 0.0 {
                    p.sling_immunity = 0.0;
                }
            }
        }

        // Update alive count
        self.alive_count = self.particles.iter().filter(|p| p.alive && !p.scored).count() as u32;
    }

    // ─── Dust motes ─────────────────────────────────────────────────────

    fn update_dust(&mut self, engine: &mut Engine) {
        for mote in &mut self.dust_motes {
            mote.age += FIXED_DT;
            if mote.age >= mote.lifetime {
                // Respawn
                mote.x = engine.rng.range_f64(0.0, WORLD_SIZE);
                mote.y = engine.rng.range_f64(0.0, WORLD_SIZE);
                mote.age = 0.0;
                mote.lifetime = engine.rng.range_f64(2.0, 6.0);
                mote.vx = 0.0;
                mote.vy = 0.0;
                continue;
            }

            // Accumulate gravity influence
            let mut ax = 0.0_f64;
            let mut ay = 0.0_f64;

            for well in &self.gravity_wells {
                if !well.active {
                    continue;
                }
                let (fax, fay) = plummer_force(well.x, well.y, well.gm * 0.3, well.epsilon, mote.x, mote.y);
                ax += fax;
                ay += fay;
            }
            for bh in &self.black_holes {
                if !bh.active {
                    continue;
                }
                let (fax, fay) = plummer_force(bh.x, bh.y, bh.gm * 0.2, bh.epsilon, mote.x, mote.y);
                ax += fax;
                ay += fay;
            }
            for rep in &self.repulsors {
                if !rep.active {
                    continue;
                }
                let (fax, fay) = plummer_force(rep.x, rep.y, rep.gm * 0.3, rep.epsilon, mote.x, mote.y);
                ax -= fax;
                ay -= fay;
            }

            mote.vx += ax * FIXED_DT;
            mote.vy += ay * FIXED_DT;
            mote.vx *= FIELD_DUST_DAMPING;
            mote.vy *= FIELD_DUST_DAMPING;
            mote.x += mote.vx * FIXED_DT;
            mote.y += mote.vy * FIXED_DT;

            // Wrap around
            if mote.x < 0.0 {
                mote.x += WORLD_SIZE;
            }
            if mote.x > WORLD_SIZE {
                mote.x -= WORLD_SIZE;
            }
            if mote.y < 0.0 {
                mote.y += WORLD_SIZE;
            }
            if mote.y > WORLD_SIZE {
                mote.y -= WORLD_SIZE;
            }
        }
    }

    // ─── Visual effects update ──────────────────────────────────────────

    fn update_effects(&mut self) {
        for effect in &mut self.effects {
            match effect {
                VisualEffect::Burst {
                    ref mut particles,
                    ref mut age,
                    duration,
                    ..
                } => {
                    *age += FIXED_DT;
                    for part in particles.iter_mut() {
                        part.0 += part.2 * FIXED_DT;
                        part.1 += part.3 * FIXED_DT;
                        // Slow down
                        part.2 *= 0.95;
                        part.3 *= 0.95;
                    }
                    let _ = duration;
                }
                VisualEffect::Flash {
                    ref mut age,
                    duration,
                    ..
                } => {
                    *age += FIXED_DT;
                    let _ = duration;
                }
            }
        }

        // Remove expired
        self.effects.retain(|e| match e {
            VisualEffect::Burst { age, duration, .. } => *age < *duration,
            VisualEffect::Flash { age, duration, .. } => *age < *duration,
        });

        // Tick entity flash timers
        for target in &mut self.targets {
            if target.flash_timer > 0.0 {
                target.flash_timer -= FIXED_DT;
                if target.flash_timer < 0.0 {
                    target.flash_timer = 0.0;
                }
            }
            target.anim_phase += FIXED_DT;
        }

        for well in &mut self.gravity_wells {
            well.anim_phase += WELL_CIRC_RATE;
        }

        for rep in &mut self.repulsors {
            rep.anim_phase += FIXED_DT;
        }

        for bh in &mut self.black_holes {
            bh.anim_phase += BLACK_HOLE_CIRC_RATE;
            for angle in &mut bh.mote_angles {
                *angle += BLACK_HOLE_CIRC_RATE * 1.5;
            }
        }

        for wall in &mut self.walls {
            if wall.flash_timer > 0.0 {
                wall.flash_timer -= FIXED_DT;
                if wall.flash_timer < 0.0 {
                    wall.flash_timer = 0.0;
                }
            }
        }

        for p in &mut self.particles {
            if p.flash_timer > 0.0 {
                p.flash_timer -= FIXED_DT;
                if p.flash_timer < 0.0 {
                    p.flash_timer = 0.0;
                }
            }
        }
    }

    // ─── Win/Loss check ─────────────────────────────────────────────────

    fn check_win_loss(&mut self, engine: &mut Engine) {
        if self.phase != GamePhase::Playing {
            return;
        }

        if self.total_scored >= self.goal_target {
            self.phase = GamePhase::Won;
            self.spawn_flash(Color::from_rgba(200, 255, 200, 255), 0.5);

            // Screen shake (celebratory)
            engine.post_fx.shake_remaining = 0.15;
            engine.post_fx.shake_intensity = 3.0;
        } else if self.alive_count == 0 {
            self.phase = GamePhase::Lost;
            self.spawn_flash(Color::from_rgba(255, 100, 100, 255), 0.5);
        }
    }

    fn update_phase(&mut self, engine: &mut Engine) {
        match self.phase {
            GamePhase::Won => {
                // Transition to next level after a delay
                self.phase = GamePhase::LevelTransition(2.0);
            }
            GamePhase::LevelTransition(ref mut timer) => {
                *timer -= FIXED_DT;
                if *timer <= 0.0 {
                    let next = self.current_level + 1;
                    let levels = level_data();
                    if next < levels.len() {
                        self.load_level(next, engine);
                    } else {
                        // Restart from level 0 (loop)
                        self.load_level(0, engine);
                    }
                }
            }
            _ => {}
        }
    }

    // ─── Rendering helpers ──────────────────────────────────────────────

    fn render_dust(&self, fb: &mut Framebuffer) {
        for mote in &self.dust_motes {
            let life_frac = if mote.lifetime > 0.0 {
                (1.0 - mote.age / mote.lifetime).max(0.0)
            } else {
                0.0
            };
            let alpha = (mote.alpha * life_frac) as u8;
            if alpha == 0 {
                continue;
            }
            let (sx, sy) = self.w2s(mote.x, mote.y);
            let color = Color::from_rgba(180, 180, 220, alpha);
            fb.set_pixel_blended(sx.round() as i32, sy.round() as i32, color);
        }
    }

    fn render_gravity_wells(&self, fb: &mut Framebuffer) {
        for well in &self.gravity_wells {
            if !well.active {
                continue;
            }
            let (sx, sy) = self.w2s(well.x, well.y);
            let r1 = self.w2s_r(well.visual_radius);
            let r2 = r1 * 2.0;
            let r3 = r1 * 4.0;

            // Influence rings (dashed)
            let ring_color = Color::from_rgba(60, 100, 180, 40);
            shapes::draw_dashed_circle(fb, sx, sy, r3, ring_color, 8.0);
            let ring_color2 = Color::from_rgba(60, 120, 200, 60);
            shapes::draw_dashed_circle(fb, sx, sy, r2, ring_color2, 6.0);

            // Pulsating core
            let pulse = (well.anim_phase.sin() * 0.15 + 1.0).max(0.8);
            let core_r = r1 * 0.4 * pulse;

            // Blue glow
            let glow_color = Color::from_rgba(40, 80, 200, 60);
            shapes::fill_circle(fb, sx, sy, r1 * 0.8, glow_color);

            // Core
            let core_color = Color::from_rgba(80, 140, 255, 220);
            shapes::fill_circle(fb, sx, sy, core_r, core_color);

            // Cyan rim
            let rim_color = Color::from_rgba(100, 220, 255, 150);
            shapes::draw_circle(fb, sx, sy, r1 * 0.6, rim_color);
        }
    }

    fn render_repulsors(&self, fb: &mut Framebuffer) {
        for rep in &self.repulsors {
            if !rep.active {
                continue;
            }
            let (sx, sy) = self.w2s(rep.x, rep.y);
            let r = self.w2s_r(rep.visual_radius);

            // Amber glow
            let glow_color = Color::from_rgba(200, 150, 30, 50);
            shapes::fill_circle(fb, sx, sy, r * 0.7, glow_color);

            // Core
            let core_color = Color::from_rgba(255, 180, 40, 220);
            let pulse = (rep.anim_phase * 3.0).sin() * 0.1 + 1.0;
            shapes::fill_circle(fb, sx, sy, r * 0.35 * pulse, core_color);

            // Gold spokes
            let spoke_color = Color::from_rgba(255, 220, 80, 120);
            let spoke_count = 6;
            for k in 0..spoke_count {
                let angle = rep.anim_phase * 0.5 + (k as f64) * std::f64::consts::TAU / spoke_count as f64;
                let inner = r * 0.35;
                let outer = r * 0.7;
                let x0 = sx + angle.cos() * inner;
                let y0 = sy + angle.sin() * inner;
                let x1 = sx + angle.cos() * outer;
                let y1 = sy + angle.sin() * outer;
                shapes::draw_line(fb, x0, y0, x1, y1, spoke_color);
            }

            // Outline ring
            let ring_color = Color::from_rgba(255, 200, 60, 80);
            shapes::draw_circle(fb, sx, sy, r * 0.7, ring_color);
        }
    }

    fn render_black_holes(&self, fb: &mut Framebuffer) {
        for bh in &self.black_holes {
            if !bh.active {
                continue;
            }
            let (sx, sy) = self.w2s(bh.x, bh.y);
            let r = self.w2s_r(bh.visual_radius);
            let kr = self.w2s_r(bh.kill_radius);

            // Dark void
            let void_color = Color::from_rgba(5, 0, 10, 240);
            shapes::fill_circle(fb, sx, sy, kr, void_color);

            // Accretion ring (orange)
            let ring_color = Color::from_rgba(255, 120, 30, 120);
            shapes::draw_circle(fb, sx, sy, r * 0.8, ring_color);
            let ring_color2 = Color::from_rgba(255, 80, 20, 80);
            shapes::draw_circle(fb, sx, sy, r, ring_color2);

            // Orbiting motes
            let mote_color = Color::from_rgba(255, 150, 50, 160);
            for angle in &bh.mote_angles {
                let mote_r = r * 0.9;
                let mx = sx + angle.cos() * mote_r;
                let my = sy + angle.sin() * mote_r;
                shapes::fill_circle(fb, mx, my, 1.5, mote_color);
            }
        }
    }

    fn render_walls(&self, fb: &mut Framebuffer) {
        for wall in &self.walls {
            let (sx1, sy1) = self.w2s(wall.x1, wall.y1);
            let (sx2, sy2) = self.w2s(wall.x2, wall.y2);

            let base_color = if wall.flash_timer > 0.0 {
                Color::from_rgba(255, 255, 255, 200)
            } else if wall.is_boost {
                Color::from_rgba(100, 255, 100, 180)
            } else {
                Color::from_rgba(150, 150, 170, 180)
            };

            shapes::draw_line_thick(fb, sx1, sy1, sx2, sy2, 3.0, base_color);
        }
    }

    fn render_targets(&self, fb: &mut Framebuffer) {
        for target in &self.targets {
            if !target.active {
                continue;
            }
            let (sx, sy) = self.w2s(target.x, target.y);
            let r = self.w2s_r(target.visual_radius);

            let flash_boost = if target.flash_timer > 0.0 { 80 } else { 0 };

            // Outer ring
            let outer_color = Color::from_rgba(
                (60 + flash_boost).min(255),
                (200 + flash_boost as u16).min(255) as u8,
                (60 + flash_boost).min(255),
                140,
            );
            shapes::draw_circle(fb, sx, sy, r, outer_color);

            // Middle ring
            let mid_color = Color::from_rgba(
                (80 + flash_boost).min(255),
                (230 + flash_boost as u16).min(255) as u8,
                (80 + flash_boost).min(255),
                160,
            );
            shapes::draw_circle(fb, sx, sy, r * 0.65, mid_color);

            // Inner filled bullseye
            let pulse = (target.anim_phase * 2.0).sin() * 0.1 + 1.0;
            let inner_r = r * 0.3 * pulse;
            let inner_color = Color::from_rgba(
                (100 + flash_boost).min(255),
                255,
                (100 + flash_boost).min(255),
                200,
            );
            shapes::fill_circle(fb, sx, sy, inner_r, inner_color);
        }
    }

    fn render_particles(&self, fb: &mut Framebuffer) {
        for p in &self.particles {
            if !p.alive {
                continue;
            }

            // Draw trail
            let trail_len = p.trail.len();
            if trail_len > 1 {
                for t in 0..trail_len - 1 {
                    let alpha = ((t as f64 / trail_len as f64) * 80.0) as u8;
                    let trail_color = p.color.with_alpha(alpha);
                    let (sx0, sy0) = self.w2s(p.trail[t].0, p.trail[t].1);
                    let (sx1, sy1) = self.w2s(p.trail[t + 1].0, p.trail[t + 1].1);
                    shapes::draw_line(fb, sx0, sy0, sx1, sy1, trail_color);
                }
            }

            // Draw particle
            let (sx, sy) = self.w2s(p.x, p.y);
            let radius = PARTICLE_RADIUS * p.scale;

            // Glow
            let glow_color = p.color.with_alpha(40);
            shapes::fill_circle(fb, sx, sy, radius * 2.0, glow_color);

            // Main body
            if p.flash_timer > 0.0 {
                shapes::fill_circle(fb, sx, sy, radius, Color::WHITE);
            } else {
                shapes::fill_circle(fb, sx, sy, radius, p.color);
            }

            // Rim highlight
            let rim_color = Color::from_rgba(
                (p.color.r as u16 + 60).min(255) as u8,
                (p.color.g as u16 + 60).min(255) as u8,
                (p.color.b as u16 + 60).min(255) as u8,
                180,
            );
            shapes::draw_circle(fb, sx, sy, radius, rim_color);

            // Locked indicator (small diamond)
            if p.locked {
                let diamond_color = Color::from_rgba(255, 255, 200, 200);
                let ds = 3.0;
                shapes::draw_line(fb, sx - ds, sy, sx, sy - ds, diamond_color);
                shapes::draw_line(fb, sx, sy - ds, sx + ds, sy, diamond_color);
                shapes::draw_line(fb, sx + ds, sy, sx, sy + ds, diamond_color);
                shapes::draw_line(fb, sx, sy + ds, sx - ds, sy, diamond_color);
            }
        }
    }

    fn render_waypoint(&self, fb: &mut Framebuffer) {
        if let Some(ref wp) = self.waypoint {
            let (sx, sy) = self.w2s(wp.x, wp.y);

            // Diamond marker
            let ds = 8.0;
            let wp_color = Color::from_rgba(255, 255, 100, 200);
            shapes::draw_line(fb, sx - ds, sy, sx, sy - ds, wp_color);
            shapes::draw_line(fb, sx, sy - ds, sx + ds, sy, wp_color);
            shapes::draw_line(fb, sx + ds, sy, sx, sy + ds, wp_color);
            shapes::draw_line(fb, sx, sy + ds, sx - ds, sy, wp_color);

            // Timeout ring (shrinks as time runs out)
            let frac = wp.remaining_frames as f64 / WAYPOINT_TIMEOUT_FRAMES as f64;
            let capture_r_screen = self.w2s_r(WAYPOINT_CAPTURE_RADIUS);
            let ring_r = capture_r_screen * frac;
            let ring_alpha = (frac * 100.0) as u8;
            let ring_color = Color::from_rgba(255, 255, 100, ring_alpha);
            shapes::draw_dashed_circle(fb, sx, sy, ring_r, ring_color, 5.0);

            // Capture lines (from waypoint to captured particles)
            let line_color = Color::from_rgba(255, 255, 100, 60);
            for &idx in &wp.captured_ids {
                if let Some(p) = self.particles.get(idx) {
                    if p.alive && !p.scored {
                        let (px, py) = self.w2s(p.x, p.y);
                        shapes::draw_line(fb, sx, sy, px, py, line_color);
                    }
                }
            }
        }
    }

    fn render_sling(&self, fb: &mut Framebuffer) {
        if let Some(ref sling) = self.sling {
            let (anchor_sx, anchor_sy) = self.w2s(sling.anchor_x, sling.anchor_y);
            let (pull_sx, pull_sy) = self.w2s(sling.pull_x, sling.pull_y);

            // Elastic bands
            let band_color = Color::from_rgba(255, 200, 100, 180);
            shapes::draw_line_thick(fb, anchor_sx, anchor_sy, pull_sx, pull_sy, 2.0, band_color);

            // Direction arrow (from pull to opposite side = launch direction)
            let dx = sling.anchor_x - sling.pull_x;
            let dy = sling.anchor_y - sling.pull_y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > 3.0 {
                let ndx = dx / dist;
                let ndy = dy / dist;
                let arrow_len = 30.0;
                let arrow_start_sx = anchor_sx;
                let arrow_start_sy = anchor_sy;
                let arrow_end_sx = anchor_sx + ndx * arrow_len;
                let arrow_end_sy = anchor_sy + ndy * arrow_len;

                let arrow_color = Color::from_rgba(255, 255, 200, 160);
                shapes::draw_line_thick(
                    fb,
                    arrow_start_sx,
                    arrow_start_sy,
                    arrow_end_sx,
                    arrow_end_sy,
                    2.0,
                    arrow_color,
                );

                // Arrowhead
                let head_size = 6.0;
                let perp_x = -ndy;
                let perp_y = ndx;
                let hx1 = arrow_end_sx - ndx * head_size + perp_x * head_size * 0.5;
                let hy1 = arrow_end_sy - ndy * head_size + perp_y * head_size * 0.5;
                let hx2 = arrow_end_sx - ndx * head_size - perp_x * head_size * 0.5;
                let hy2 = arrow_end_sy - ndy * head_size - perp_y * head_size * 0.5;
                shapes::draw_line(fb, arrow_end_sx, arrow_end_sy, hx1, hy1, arrow_color);
                shapes::draw_line(fb, arrow_end_sx, arrow_end_sy, hx2, hy2, arrow_color);
            }

            // Pull dot
            let dot_color = Color::from_rgba(255, 220, 120, 220);
            shapes::fill_circle(fb, pull_sx, pull_sy, 4.0, dot_color);
        }
    }

    fn render_effects(&self, fb: &mut Framebuffer) {
        for effect in &self.effects {
            match effect {
                VisualEffect::Burst {
                    particles,
                    color,
                    age,
                    duration,
                    ..
                } => {
                    let frac = if *duration > 0.0 {
                        (1.0 - age / duration).max(0.0)
                    } else {
                        0.0
                    };
                    let alpha = (frac * 255.0) as u8;
                    let c = color.with_alpha(alpha);
                    for part in particles {
                        let (sx, sy) = self.w2s(part.0, part.1);
                        let r = 2.0 * frac;
                        shapes::fill_circle(fb, sx, sy, r, c);
                    }
                }
                VisualEffect::Flash {
                    age,
                    duration,
                    color,
                } => {
                    let frac = if *duration > 0.0 {
                        (1.0 - age / duration).max(0.0)
                    } else {
                        0.0
                    };
                    let alpha = (frac * frac * 80.0) as u8;
                    if alpha > 0 {
                        let c = color.with_alpha(alpha);
                        shapes::fill_rect(fb, 0.0, 0.0, fb.width as f64, fb.height as f64, c);
                    }
                }
            }
        }
    }

    fn render_hud(&self, fb: &mut Framebuffer) {
        // Score counter
        let score_text = format!(
            "{}/{}",
            self.total_scored, self.goal_target
        );
        let hud_color = Color::from_rgba(220, 220, 240, 220);
        text::draw_text(fb, 10, 10, &score_text, hud_color, 2);

        // Level indicator
        let level_text = format!("Level {}", self.current_level + 1);
        let level_w = text::text_width(&level_text, 1);
        text::draw_text(
            fb,
            fb.width as i32 - level_w - 10,
            10,
            &level_text,
            Color::from_rgba(180, 180, 200, 180),
            1,
        );

        // Alive count
        let alive_text = format!("Alive: {}", self.alive_count);
        text::draw_text(
            fb,
            10,
            fb.height as i32 - 20,
            &alive_text,
            Color::from_rgba(180, 180, 200, 160),
            1,
        );
    }

    fn render_phase_overlay(&self, fb: &mut Framebuffer) {
        let cx = fb.width as i32 / 2;
        let cy = fb.height as i32 / 2;

        match &self.phase {
            GamePhase::Won => {
                // Dark overlay
                shapes::fill_rect(
                    fb,
                    0.0,
                    0.0,
                    fb.width as f64,
                    fb.height as f64,
                    Color::from_rgba(0, 0, 0, 100),
                );
                text::draw_text_centered(
                    fb,
                    cx,
                    cy - 20,
                    "LEVEL COMPLETE!",
                    Color::from_rgba(100, 255, 100, 255),
                    3,
                );
                text::draw_text_centered(
                    fb,
                    cx,
                    cy + 20,
                    "Next level loading...",
                    Color::from_rgba(200, 200, 220, 200),
                    1,
                );
            }
            GamePhase::Lost => {
                shapes::fill_rect(
                    fb,
                    0.0,
                    0.0,
                    fb.width as f64,
                    fb.height as f64,
                    Color::from_rgba(0, 0, 0, 120),
                );
                text::draw_text_centered(
                    fb,
                    cx,
                    cy - 20,
                    "ALL PARTICLES LOST",
                    Color::from_rgba(255, 80, 80, 255),
                    3,
                );
                text::draw_text_centered(
                    fb,
                    cx,
                    cy + 20,
                    "Click to retry",
                    Color::from_rgba(200, 200, 220, 200),
                    1,
                );
            }
            GamePhase::LevelTransition(timer) => {
                let alpha = ((*timer / 2.0) * 150.0).min(150.0) as u8;
                shapes::fill_rect(
                    fb,
                    0.0,
                    0.0,
                    fb.width as f64,
                    fb.height as f64,
                    Color::from_rgba(0, 0, 0, alpha),
                );
                text::draw_text_centered(
                    fb,
                    cx,
                    cy,
                    "LEVEL COMPLETE!",
                    Color::from_rgba(100, 255, 100, 255),
                    3,
                );
            }
            GamePhase::Playing => {}
        }
    }
}

// ─── Simulation trait ───────────────────────────────────────────────────────

impl Simulation for GravityPong {
    fn setup(&mut self, engine: &mut Engine) {
        engine.config.bounds = (640.0, 640.0);
        engine.config.background = Color::from_rgba(10, 10, 20, 255);
        engine.debug_mode = false;

        self.screen_w = engine.width as f64;
        self.screen_h = engine.height as f64;
        self.scale = self.screen_w / WORLD_SIZE;

        self.load_level(0, engine);
    }

    fn step(&mut self, engine: &mut Engine) {
        // Handle retry on lost
        if self.phase == GamePhase::Lost {
            if engine.input.mouse_buttons_pressed.contains(&0) {
                self.load_level(self.current_level, engine);
                return;
            }
            // Still tick effects for visual feedback
            self.update_effects();
            return;
        }

        // Don't process input during transitions
        if let GamePhase::LevelTransition(_) = self.phase {
            self.update_phase(engine);
            self.update_effects();
            return;
        }

        // 1. Input
        self.handle_input(engine);

        // 2. Waypoint
        self.update_waypoint();

        // 3. Physics
        self.update_physics(engine);

        // 4. Dust
        self.update_dust(engine);

        // 5. Effects
        self.update_effects();

        // 6. Win/Loss
        self.check_win_loss(engine);

        // 7. Phase transitions
        self.update_phase(engine);
    }

    fn render(&self, engine: &mut Engine) {
        let fb = &mut engine.framebuffer;

        // 1. Background already cleared by engine config

        // 2. Vignette
        post_fx::vignette(fb, 0.3);

        // 3. Dust motes
        self.render_dust(fb);

        // 4-6. Gravity wells, repulsors, black holes
        self.render_gravity_wells(fb);
        self.render_repulsors(fb);
        self.render_black_holes(fb);

        // 7. Walls
        self.render_walls(fb);

        // 8. Targets
        self.render_targets(fb);

        // 9. Particles (with trails)
        self.render_particles(fb);

        // 10. Waypoint UI
        self.render_waypoint(fb);

        // 11. Sling UI
        self.render_sling(fb);

        // 12. Visual effects
        self.render_effects(fb);

        // 13. HUD
        self.render_hud(fb);

        // 14. Phase overlays
        self.render_phase_overlay(fb);
    }
}

// ─── Free Functions ─────────────────────────────────────────────────────────

/// Plummer gravitational force: returns (ax, ay) acceleration.
fn plummer_force(
    source_x: f64,
    source_y: f64,
    gm: f64,
    epsilon: f64,
    px: f64,
    py: f64,
) -> (f64, f64) {
    let dx = source_x - px;
    let dy = source_y - py;
    let r_sq = dx * dx + dy * dy;
    let eps_sq = epsilon * epsilon;
    let denom = (r_sq + eps_sq).powf(1.5);
    if denom < 1e-10 {
        return (0.0, 0.0);
    }
    let ax = gm * dx / denom;
    let ay = gm * dy / denom;
    (ax, ay)
}

/// Wall collision: returns (collided, new_vx, new_vy, new_x, new_y).
fn wall_collision(
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    wall: &Wall,
) -> (bool, f64, f64, f64, f64) {
    let wx = wall.x2 - wall.x1;
    let wy = wall.y2 - wall.y1;
    let wall_len_sq = wx * wx + wy * wy;
    if wall_len_sq < 1e-10 {
        return (false, vx, vy, px, py);
    }

    // Project particle onto wall line segment
    let t = ((px - wall.x1) * wx + (py - wall.y1) * wy) / wall_len_sq;
    let t_clamped = t.clamp(0.0, 1.0);
    let closest_x = wall.x1 + t_clamped * wx;
    let closest_y = wall.y1 + t_clamped * wy;

    let dx = px - closest_x;
    let dy = py - closest_y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < radius && dist > 1e-10 {
        // Collision: push particle out and reflect velocity
        let nx = dx / dist;
        let ny = dy / dist;

        // Push out
        let penetration = radius - dist;
        let new_x = px + nx * penetration;
        let new_y = py + ny * penetration;

        // Reflect velocity
        let dot = vx * nx + vy * ny;
        let new_vx = (vx - 2.0 * dot * nx) * wall.restitution;
        let new_vy = (vy - 2.0 * dot * ny) * wall.restitution;

        (true, new_vx, new_vy, new_x, new_y)
    } else {
        (false, vx, vy, px, py)
    }
}

/// Parse f64 from string, returning 0.0 on failure.
fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}

/// Deterministic particle colour palette.
fn particle_color(idx: u32) -> Color {
    match idx % 6 {
        0 => Color::from_rgba(255, 120, 80, 255),  // warm orange
        1 => Color::from_rgba(80, 200, 255, 255),   // sky blue
        2 => Color::from_rgba(255, 80, 160, 255),   // pink
        3 => Color::from_rgba(120, 255, 120, 255),  // lime
        4 => Color::from_rgba(200, 140, 255, 255),  // lavender
        5 => Color::from_rgba(255, 220, 80, 255),   // gold
        _ => Color::WHITE,
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity_pong_new_defaults() {
        let gp = GravityPong::new();
        assert_eq!(gp.current_level, 0);
        assert_eq!(gp.total_scored, 0);
        assert_eq!(gp.phase, GamePhase::Playing);
        assert!(gp.particles.is_empty());
    }

    #[test]
    fn plummer_force_attracts_toward_source() {
        let (ax, ay) = plummer_force(100.0, 0.0, 1000.0, 10.0, 0.0, 0.0);
        assert!(ax > 0.0, "should attract toward source in x");
        assert!(ay.abs() < 1e-10, "should not attract in y when aligned on x");
    }

    #[test]
    fn plummer_force_zero_at_source() {
        let (ax, ay) = plummer_force(50.0, 50.0, 1000.0, 10.0, 50.0, 50.0);
        // At source, force should be very small (epsilon softens it)
        assert!(ax.abs() < 1e-5);
        assert!(ay.abs() < 1e-5);
    }

    #[test]
    fn wall_collision_detects_nearby_particle() {
        let wall = Wall::from_params(0.0, 500.0, 1000.0, 500.0, 100.0);
        let (collided, _, _, _, _) = wall_collision(500.0, 503.0, 0.0, -10.0, 8.0, &wall);
        assert!(collided, "particle within radius should collide");
    }

    #[test]
    fn wall_collision_misses_far_particle() {
        let wall = Wall::from_params(0.0, 500.0, 1000.0, 500.0, 100.0);
        let (collided, _, _, _, _) = wall_collision(500.0, 400.0, 0.0, -10.0, 8.0, &wall);
        assert!(!collided, "particle far from wall should not collide");
    }

    #[test]
    fn parse_f64_valid() {
        assert!((parse_f64("42.5") - 42.5).abs() < 1e-10);
    }

    #[test]
    fn parse_f64_invalid() {
        assert!((parse_f64("abc") - 0.0).abs() < 1e-10);
    }

    #[test]
    fn particle_color_cycles() {
        let c0 = particle_color(0);
        let c6 = particle_color(6);
        assert_eq!(c0, c6, "colors should cycle every 6");
    }

    #[test]
    fn level_data_has_three_levels() {
        let levels = level_data();
        assert_eq!(levels.len(), 3);
    }

    #[test]
    fn coordinate_round_trip() {
        let gp = GravityPong::new();
        let (sx, sy) = gp.w2s(500.0, 500.0);
        let (wx, wy) = gp.s2w(sx, sy);
        assert!((wx - 500.0).abs() < 1e-10);
        assert!((wy - 500.0).abs() < 1e-10);
    }

    #[test]
    fn gravity_well_from_params_computes_gm() {
        let well = GravityWell::from_params(500.0, 500.0, 50.0);
        assert!(well.gm > WELL_MIN_STRENGTH);
        assert!(well.gm < WELL_MAX_STRENGTH);
        assert!(well.epsilon > 0.0);
    }

    #[test]
    fn black_hole_from_params() {
        let bh = BlackHole::from_params(500.0, 300.0, 40.0);
        assert_eq!(bh.gm, BLACK_HOLE_GRAVITY);
        assert!(bh.kill_radius > 0.0);
        assert!(bh.kill_radius < bh.visual_radius);
        assert_eq!(bh.mote_angles.len(), 8);
    }

    #[test]
    fn target_from_params() {
        let t = Target::from_params(500.0, 200.0, 70.0);
        assert!(t.hit_radius > 0.0);
        assert!(t.hit_radius < t.visual_radius);
        assert!(t.active);
    }

    #[test]
    fn wall_restitution_scales() {
        let wall = Wall::from_params(0.0, 0.0, 100.0, 0.0, 80.0);
        assert!((wall.restitution - 0.8).abs() < 1e-10);
    }

    #[test]
    fn setup_initialises_level() {
        let mut gp = GravityPong::new();
        let mut engine = Engine::new(640, 640);
        gp.setup(&mut engine);
        assert!(!gp.particles.is_empty());
        assert!(!gp.targets.is_empty());
        assert_eq!(gp.phase, GamePhase::Playing);
        assert!(gp.dust_motes.len() == FIELD_DUST_COUNT);
    }

    #[test]
    fn load_level_clamps_index() {
        let mut gp = GravityPong::new();
        let mut engine = Engine::new(640, 640);
        gp.screen_w = 640.0;
        gp.screen_h = 640.0;
        gp.scale = 0.64;
        gp.load_level(999, &mut engine);
        assert_eq!(gp.current_level, 2); // clamped to last level
    }
}
