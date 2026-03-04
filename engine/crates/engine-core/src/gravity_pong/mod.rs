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
const WELL_MIN_STRENGTH: f64 = 40_000.0;
const WELL_MAX_STRENGTH: f64 = 300_000.0;
const WELL_EPSILON_FACTOR: f64 = 0.5;
const WELL_CIRC_RATE: f64 = 0.03;

// Black holes
const BLACK_HOLE_GRAVITY: f64 = 600_000.0;
const BLACK_HOLE_EPSILON_FACTOR: f64 = 1.2;
const BLACK_HOLE_CIRC_RATE: f64 = 0.08;

// Repulsors
const REPULSOR_MIN_STRENGTH: f64 = 200_000.0;
const REPULSOR_MAX_STRENGTH: f64 = 1_200_000.0;
const REPULSOR_EPSILON_FACTOR: f64 = 0.3;

// Drag
const BASE_DRAG: f64 = 0.35;
const SPEED_DRAG: f64 = 0.0;
const REST_THRESHOLD: f64 = 0.5;

// Physics
const EDGE_RESTITUTION: f64 = 0.85;
const MAX_SPEED: f64 = 350.0;
const MAX_SUBSTEPS: u32 = 8;
const MAX_BOUNCES: u32 = 5;

// Waypoint
const WAYPOINT_CAPTURE_RADIUS: f64 = 150.0;
const WAYPOINT_TRAVEL_SPEED: f64 = 12.0;
const WAYPOINT_TIMEOUT_FRAMES: u32 = 180;

// Sling
const SLING_MAX_SPEED: f64 = 420.0;
const SLING_MAX_PULL: f64 = 150.0;
const SLING_DECAY_RATE: f64 = 2.5;

// Visual
const FIELD_DUST_COUNT: usize = 300;
const FIELD_DUST_DAMPING: f64 = 0.85;
const PARTICLE_RADIUS: f64 = 5.0;

// Trail
const MAX_TRAIL_LEN: usize = 60;

// World size
const WORLD_SIZE: f64 = 1000.0;

// Particle collision radius in world units
const PARTICLE_WORLD_RADIUS: f64 = 8.0;

// Wormhole
const WORMHOLE_CAPTURE_RADIUS: f64 = 25.0;
const WORMHOLE_COOLDOWN: f64 = 0.5;
const WORMHOLE_GM: f64 = 8_000.0;
const WORMHOLE_EPSILON: f64 = 25.0;

// Supernova
const SUPERNOVA_FLASH_HZ_EARLY: f64 = 4.0;
const SUPERNOVA_FLASH_HZ_LATE: f64 = 8.0;

// Field visualization grid
const FIELD_GRID_RES: usize = 50;

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
    ExpandingRing {
        x: f64,
        y: f64,
        current_radius: f64,
        max_radius: f64,
        color: Color,
        age: f64,
        duration: f64,
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

#[derive(Clone, Debug)]
struct Wormhole {
    x1: f64,
    y1: f64, // mouth A position
    x2: f64,
    y2: f64, // mouth B position
    capture_radius: f64,
    cooldown1: f64, // remaining cooldown for mouth A (seconds)
    cooldown2: f64, // remaining cooldown for mouth B (seconds)
    anim_phase: f64,
    color: Color,
}

impl Wormhole {
    fn from_params(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            capture_radius: WORMHOLE_CAPTURE_RADIUS,
            cooldown1: 0.0,
            cooldown2: 0.0,
            anim_phase: 0.0,
            color: Color::from_rgba(160, 80, 255, 220),
        }
    }
}

#[derive(Clone, Debug)]
struct PlasmaCurrent {
    x1: f64,
    y1: f64, // start of corridor
    x2: f64,
    y2: f64, // end of corridor
    half_width: f64, // corridor half-width
    strength: f64,   // force strength
    anim_phase: f64,
}

impl PlasmaCurrent {
    fn from_params(x1: f64, y1: f64, x2: f64, y2: f64, width: f64, strength_raw: f64) -> Self {
        // Strength mapping: value 1 -> 30.0, value 100 -> 200.0
        let strength = 30.0 + (200.0 - 30.0) * (strength_raw - 1.0) / 99.0;
        Self {
            x1,
            y1,
            x2,
            y2,
            half_width: width / 2.0,
            strength,
            anim_phase: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct Supernova {
    x: f64,
    y: f64,
    size: f64,
    visual_radius: f64,
    blast_radius: f64,
    countdown: f64,         // remaining seconds until detonation
    initial_countdown: f64,  // for progress calculation
    detonated: bool,
    anim_phase: f64,
}

impl Supernova {
    fn from_params(x: f64, y: f64, size: f64, countdown: f64) -> Self {
        let visual_radius = 15.0 + size * 0.3;
        let blast_radius = visual_radius * 3.5;
        Self {
            x, y, size, visual_radius, blast_radius,
            countdown,
            initial_countdown: countdown,
            detonated: false,
            anim_phase: 0.0,
        }
    }
}

// ─── Level Data ─────────────────────────────────────────────────────────────

fn level_data() -> Vec<Vec<&'static str>> {
    vec![
        // Level 1 - "First Contact" (tutorial): simple direct path
        vec![
            "target:500:200:70",
            "gravity-well:500:550:40",
            "particle:350:850",
            "particle:500:850",
            "particle:650:850",
        ],
        // Level 2 - "The Sling": must use waypoint+sling
        vec![
            "target:500:100:60",
            "particle:350:800",
            "particle:500:800",
            "particle:650:800",
        ],
        // Level 3 - "Orbital": two gravity wells, wall barrier
        vec![
            "target:500:120:60",
            "gravity-well:300:450:55",
            "gravity-well:700:450:55",
            "wall:200:320:800:320:80",
            "particle:250:850",
            "particle:400:850",
            "particle:600:850",
            "particle:750:850",
        ],
        // Level 4 - "Through the Wormhole": target behind wall, wormhole bypass
        vec![
            "target:500:120:65",
            "wall:100:400:900:400:80",
            "wormhole:500:700:500:270",
            "particle:300:850",
            "particle:500:850",
            "particle:700:850",
            "particle:400:850",
        ],
        // Level 5 - "Current Affairs": plasma currents as wind tunnel
        vec![
            "target:850:200:65",
            "plasma-current:100:800:900:200:120:60",
            "plasma-current:100:600:600:100:80:40",
            "particle:150:850",
            "particle:250:900",
            "particle:350:880",
            "particle:200:750",
            "particle:300:800",
        ],
        // Level 6 - "Dark Side": two wells, black hole guarding target
        vec![
            "target:500:100:50",
            "gravity-well:300:450:50",
            "gravity-well:700:450:50",
            "black-hole:500:280:40",
            "wall:100:200:100:800:80",
            "wall:900:200:900:800:80",
            "particle:250:850",
            "particle:400:850",
            "particle:550:850",
            "particle:700:850",
            "particle:850:850",
        ],
        // Level 7 - "Chaos Theory": full complexity
        vec![
            "target:150:150:65",
            "target:850:150:65",
            "gravity-well:300:500:50",
            "gravity-well:700:500:50",
            "gravity-well:500:300:35",
            "repulsor:500:600:40",
            "black-hole:500:200:35",
            "wormhole:200:800:800:800",
            "particle:200:900",
            "particle:350:900",
            "particle:500:900",
            "particle:650:900",
            "particle:800:900",
            "particle:300:850",
            "particle:700:850",
        ],
        // Level 8 - "The Gauntlet": maximum challenge
        vec![
            "target:200:150:65",
            "target:800:150:65",
            "gravity-well:350:400:55",
            "gravity-well:650:400:55",
            "black-hole:500:500:45",
            "black-hole:200:600:35",
            "wall:300:300:700:300:80",
            "wall:100:600:400:600:80",
            "wall:600:600:900:600:80",
            "plasma-current:100:900:900:100:100:55",
            "wormhole:150:750:850:750",
            "particle:200:900",
            "particle:350:900",
            "particle:500:900",
            "particle:650:900",
            "particle:800:900",
            "particle:300:850",
            "particle:700:850",
            "particle:450:950",
        ],
        // Level 9 - "Countdown": supernova timed hazard
        vec![
            "target:500:150:65",
            "gravity-well:400:500:45",
            "gravity-well:600:500:45",
            "supernova:500:400:50:8",
            "particle:300:850",
            "particle:500:850",
            "particle:700:850",
            "particle:400:900",
            "particle:600:900",
        ],
        // Level 10 - "Final Frontier": everything combined
        vec![
            "target:200:100:60",
            "target:800:100:60",
            "gravity-well:300:350:50",
            "gravity-well:700:350:50",
            "black-hole:500:450:40",
            "repulsor:500:700:45",
            "supernova:250:550:40:10",
            "supernova:750:550:40:12",
            "wormhole:150:900:850:200",
            "plasma-current:100:500:400:200:80:50",
            "wall:400:250:600:250:80",
            "particle:150:950",
            "particle:300:950",
            "particle:450:950",
            "particle:600:950",
            "particle:750:950",
            "particle:350:850",
            "particle:650:850",
        ],
    ]
}

fn level_names() -> Vec<&'static str> {
    vec![
        "First Contact",
        "The Sling",
        "Orbital",
        "Through the Wormhole",
        "Current Affairs",
        "Dark Side",
        "Chaos Theory",
        "The Gauntlet",
        "Countdown",
        "Final Frontier",
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
    wormholes: Vec<Wormhole>,
    plasma_currents: Vec<PlasmaCurrent>,
    supernovas: Vec<Supernova>,
    elapsed_time: f64,
    field_grid: Vec<f64>,
    waypoint_preview: Option<(f64, f64)>,
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
            wormholes: Vec::new(),
            plasma_currents: Vec::new(),
            supernovas: Vec::new(),
            elapsed_time: 0.0,
            field_grid: Vec::new(),
            waypoint_preview: None,
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
        self.wormholes.clear();
        self.plasma_currents.clear();
        self.supernovas.clear();
        self.waypoint = None;
        self.sling = None;
        self.effects.clear();
        self.total_scored = 0;
        self.alive_count = 0;
        self.phase = GamePhase::Playing;
        self.mouse_down_frame = None;
        self.mouse_down_pos = None;
        self.elapsed_time = 0.0;

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
                    "wormhole" => {
                        if parts.len() >= 5 {
                            let x1 = parse_f64(parts[1]);
                            let y1 = parse_f64(parts[2]);
                            let x2 = parse_f64(parts[3]);
                            let y2 = parse_f64(parts[4]);
                            self.wormholes.push(Wormhole::from_params(x1, y1, x2, y2));
                        }
                    }
                    "plasma-current" => {
                        if parts.len() >= 7 {
                            let x1 = parse_f64(parts[1]);
                            let y1 = parse_f64(parts[2]);
                            let x2 = parse_f64(parts[3]);
                            let y2 = parse_f64(parts[4]);
                            let width = parse_f64(parts[5]);
                            let strength_raw = parse_f64(parts[6]);
                            self.plasma_currents.push(PlasmaCurrent::from_params(
                                x1,
                                y1,
                                x2,
                                y2,
                                width,
                                strength_raw,
                            ));
                        }
                    }
                    "supernova" => {
                        if parts.len() >= 5 {
                            let x = parse_f64(parts[1]);
                            let y = parse_f64(parts[2]);
                            let size = parse_f64(parts[3]);
                            let countdown = parse_f64(parts[4]);
                            self.supernovas.push(Supernova::from_params(x, y, size, countdown));
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

        // Precompute gravitational field visualization
        self.compute_field_grid();
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

    fn compute_field_grid(&mut self) {
        let cell_count = FIELD_GRID_RES * FIELD_GRID_RES;
        self.field_grid.clear();
        self.field_grid.resize(cell_count, 0.0);
        let cell_size = WORLD_SIZE / FIELD_GRID_RES as f64;

        for gy in 0..FIELD_GRID_RES {
            for gx in 0..FIELD_GRID_RES {
                let wx = (gx as f64 + 0.5) * cell_size;
                let wy = (gy as f64 + 0.5) * cell_size;
                let mut potential = 0.0_f64;

                for well in &self.gravity_wells {
                    if !well.active { continue; }
                    let dx = wx - well.x;
                    let dy = wy - well.y;
                    let r_sq = dx * dx + dy * dy;
                    let eps_sq = well.epsilon * well.epsilon;
                    potential += well.gm / (r_sq + eps_sq).sqrt();
                }
                for rep in &self.repulsors {
                    if !rep.active { continue; }
                    let dx = wx - rep.x;
                    let dy = wy - rep.y;
                    let r_sq = dx * dx + dy * dy;
                    let eps_sq = rep.epsilon * rep.epsilon;
                    potential -= rep.gm / (r_sq + eps_sq).sqrt();
                }
                for bh in &self.black_holes {
                    if !bh.active { continue; }
                    let dx = wx - bh.x;
                    let dy = wy - bh.y;
                    let r_sq = dx * dx + dy * dy;
                    let eps_sq = bh.epsilon * bh.epsilon;
                    potential += bh.gm / (r_sq + eps_sq).sqrt();
                }

                self.field_grid[gy * FIELD_GRID_RES + gx] = potential;
            }
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

    fn place_waypoint(&mut self, wx: f64, wy: f64) {
        // Release particles from old waypoint before replacing
        if let Some(old_wp) = self.waypoint.take() {
            for &idx in &old_wp.captured_ids {
                if let Some(p) = self.particles.get_mut(idx) {
                    p.captured = false;
                    p.locked = false;
                    p.scale = 1.0;
                }
            }
        }
        self.waypoint = Some(Waypoint {
            x: wx,
            y: wy,
            remaining_frames: WAYPOINT_TIMEOUT_FRAMES,
            captured_ids: Vec::new(),
        });
    }

    fn handle_input(&mut self, engine: &mut Engine) {
        let frame = engine.frame;

        // Mouse pressed this frame
        if engine.input.mouse_buttons_pressed.contains(&0) {
            self.mouse_down_frame = Some(frame);
            self.mouse_down_pos = Some((engine.input.mouse_x, engine.input.mouse_y));

            // If any locked particle exists, start sling from anywhere (minigolf style)
            let mut locked_idx: Option<usize> = None;
            for (i, p) in self.particles.iter().enumerate() {
                if p.alive && !p.scored && p.locked {
                    locked_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = locked_idx {
                let p = &self.particles[idx];
                self.sling = Some(SlingDrag {
                    particle_idx: idx,
                    anchor_x: p.x,
                    anchor_y: p.y,
                    pull_x: p.x, // Start at particle (zero pull distance)
                    pull_y: p.y,
                });
            }
        }

        // Mouse held — update sling via drag delta, or show waypoint preview
        if engine.input.mouse_buttons_held.contains(&0) {
            let scale = self.scale;
            let s2w = |sx: f64, sy: f64| -> (f64, f64) {
                if scale > 0.0 { (sx / scale, sy / scale) } else { (sx, sy) }
            };

            if let Some(ref mut sling) = self.sling {
                // Delta-based sling: pull offset = mouse delta from press point
                if let Some((start_sx, start_sy)) = self.mouse_down_pos {
                    let (start_wx, start_wy) = s2w(start_sx, start_sy);
                    let (curr_wx, curr_wy) =
                        s2w(engine.input.mouse_x, engine.input.mouse_y);
                    let delta_wx = curr_wx - start_wx;
                    let delta_wy = curr_wy - start_wy;

                    let target_x = sling.anchor_x + delta_wx;
                    let target_y = sling.anchor_y + delta_wy;
                    let dx = target_x - sling.anchor_x;
                    let dy = target_y - sling.anchor_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let max_pull_w = SLING_MAX_PULL / scale;

                    if dist > max_pull_w && dist > 0.0 {
                        sling.pull_x = sling.anchor_x + dx / dist * max_pull_w;
                        sling.pull_y = sling.anchor_y + dy / dist * max_pull_w;
                    } else {
                        sling.pull_x = target_x;
                        sling.pull_y = target_y;
                    }
                }
                self.waypoint_preview = None;
            } else {
                // No sling active — show waypoint capture radius preview
                let (mx_w, my_w) = s2w(engine.input.mouse_x, engine.input.mouse_y);
                if mx_w >= 0.0 && mx_w <= WORLD_SIZE && my_w >= 0.0 && my_w <= WORLD_SIZE {
                    self.waypoint_preview = Some((mx_w, my_w));
                }
            }
        }

        // Mouse released this frame
        if engine.input.mouse_buttons_released.contains(&0) {
            self.waypoint_preview = None;

            if let Some(sling) = self.sling.take() {
                let dx = sling.anchor_x - sling.pull_x;
                let dy = sling.anchor_y - sling.pull_y;
                let pull_dist = (dx * dx + dy * dy).sqrt();
                let min_pull = 15.0 / self.scale;

                if pull_dist > min_pull {
                    // Significant pull → LAUNCH particle
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
                            p.scale = 1.0;
                            p.flash_timer = 0.08;

                            // Sling served its purpose — remove waypoint entirely
                            self.waypoint = None;

                            // Micro-shake on launch
                            engine.post_fx.shake_remaining = 0.04;
                            engine.post_fx.shake_intensity = 2.0;

                            // Backward spray particles
                            let spray_dx = sling.pull_x - sling.anchor_x;
                            let spray_dy = sling.pull_y - sling.anchor_y;
                            let spray_len =
                                (spray_dx * spray_dx + spray_dy * spray_dy).sqrt();
                            if spray_len > 0.01 {
                                let ndx = spray_dx / spray_len;
                                let ndy = spray_dy / spray_len;
                                let mut spray = Vec::with_capacity(4);
                                for k in 0..4 {
                                    let angle_offset = (k as f64 - 1.5) * 0.3;
                                    let cos_a = angle_offset.cos();
                                    let sin_a = angle_offset.sin();
                                    let svx = (ndx * cos_a - ndy * sin_a) * 80.0;
                                    let svy = (ndx * sin_a + ndy * cos_a) * 80.0;
                                    spray.push((
                                        sling.anchor_x,
                                        sling.anchor_y,
                                        svx,
                                        svy,
                                    ));
                                }
                                self.effects.push(VisualEffect::Burst {
                                    x: sling.anchor_x,
                                    y: sling.anchor_y,
                                    particles: spray,
                                    color: Color::from_rgba(255, 255, 200, 200),
                                    age: 0.0,
                                    duration: 0.3,
                                });
                            }
                        }
                    }
                } else {
                    // Tiny pull → just cancel sling, don't place waypoint
                    // (particle stays locked, user can try again)
                }
            } else {
                // No sling (no locked particle) → place waypoint on release
                let (mx_w, my_w) =
                    self.s2w(engine.input.mouse_x, engine.input.mouse_y);
                if mx_w >= 0.0 && mx_w <= WORLD_SIZE && my_w >= 0.0 && my_w <= WORLD_SIZE {
                    self.place_waypoint(mx_w, my_w);
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

                // Capture only the closest particle (one at a time)
                if wp.captured_ids.is_empty() {
                    let capture_r_sq =
                        WAYPOINT_CAPTURE_RADIUS * WAYPOINT_CAPTURE_RADIUS;
                    let mut closest_idx: Option<usize> = None;
                    let mut closest_dist_sq = f64::MAX;
                    for i in 0..self.particles.len() {
                        let p = &self.particles[i];
                        if !p.alive || p.scored || p.locked || p.captured
                            || p.sling_immunity > 0.0
                        {
                            continue;
                        }
                        let dx = p.x - wp.x;
                        let dy = p.y - wp.y;
                        let dist_sq = dx * dx + dy * dy;
                        if dist_sq < capture_r_sq && dist_sq < closest_dist_sq {
                            closest_dist_sq = dist_sq;
                            closest_idx = Some(i);
                        }
                    }
                    if let Some(idx) = closest_idx {
                        wp.captured_ids.push(idx);
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
                        p.scale = 1.5; // Enlarge to signal "ready to sling"
                        p.flash_timer = 0.1;
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
        let mut wall_hit_indices: Vec<usize> = Vec::new();

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
            let pvx;
            let pvy;
            let immunity;
            {
                let p = &self.particles[i];
                px = p.x;
                py = p.y;
                pvx = p.vx;
                pvy = p.vy;
                immunity = p.sling_immunity;
            }

            // Count attractors for this particle
            let mut attractor_count = 0u32;

            // Gravity wells
            for well in &self.gravity_wells {
                if !well.active {
                    continue;
                }
                let (fax, fay) = plummer_force(well.x, well.y, well.gm, well.epsilon, px, py);
                ax += fax;
                ay += fay;
                // Count as attractor if meaningfully close
                let dx = well.x - px;
                let dy = well.y - py;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < (well.visual_radius * 8.0) * (well.visual_radius * 8.0) {
                    attractor_count += 1;
                }
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
                let dx = bh.x - px;
                let dy = bh.y - py;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < (bh.visual_radius * 8.0) * (bh.visual_radius * 8.0) {
                    attractor_count += 1;
                }
            }

            // Wormhole attraction (weak Plummer from each mouth)
            for wh in &self.wormholes {
                let (fax1, fay1) =
                    plummer_force(wh.x1, wh.y1, WORMHOLE_GM, WORMHOLE_EPSILON, px, py);
                ax += fax1;
                ay += fay1;
                let (fax2, fay2) =
                    plummer_force(wh.x2, wh.y2, WORMHOLE_GM, WORMHOLE_EPSILON, px, py);
                ax += fax2;
                ay += fay2;
            }

            // Plasma current forces
            for pc in &self.plasma_currents {
                let (pax, pay) = plasma_current_force(pc, px, py, pvx, pvy);
                ax += pax;
                ay += pay;
            }

            // Store attractor count
            self.particles[i].attractor_count = attractor_count;

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

                // Apply drag (reduced while sling immunity active)
                let speed = (p.vx * p.vx + p.vy * p.vy).sqrt();
                let drag_scale = 1.0 - 0.8 * p.sling_immunity;
                let effective_drag = (BASE_DRAG + SPEED_DRAG * speed) * drag_scale;
                let factor = (-effective_drag * sub_dt).exp();
                p.vx *= factor;
                p.vy *= factor;
                if speed * factor < REST_THRESHOLD {
                    p.vx = 0.0;
                    p.vy = 0.0;
                }

                // Clamp speed (sling immunity allows higher speeds)
                let speed_after = (p.vx * p.vx + p.vy * p.vy).sqrt();
                let base_max = MAX_SPEED / self.scale;
                let max_speed_w = base_max
                    + (SLING_MAX_SPEED - base_max) * p.sling_immunity;
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
            for (wi, wall) in self.walls.iter().enumerate() {
                let (collided, new_vx, new_vy, new_x, new_y) =
                    wall_collision(p.x, p.y, p.vx, p.vy, PARTICLE_WORLD_RADIUS, wall);
                if collided {
                    p.x = new_x;
                    p.y = new_y;
                    p.vx = new_vx;
                    p.vy = new_vy;
                    wall_hit_indices.push(wi);
                }
            }

            // Record trail position
            let p = &mut self.particles[i];
            p.trail.push((p.x, p.y));
            if p.trail.len() > MAX_TRAIL_LEN {
                p.trail.remove(0);
            }
        }

        // Flash walls that were hit
        for wi in wall_hit_indices {
            if let Some(wall) = self.walls.get_mut(wi) {
                wall.flash_timer = 0.1;
            }
        }

        // Wormhole teleportation pass
        for i in 0..num_particles {
            {
                let p = &self.particles[i];
                if !p.alive || p.scored || p.captured {
                    continue;
                }
            }

            let num_wormholes = self.wormholes.len();
            for wh_idx in 0..num_wormholes {
                let px = self.particles[i].x;
                let py = self.particles[i].y;
                let pvx = self.particles[i].vx;
                let pvy = self.particles[i].vy;

                // Check mouth A
                let dx1 = px - self.wormholes[wh_idx].x1;
                let dy1 = py - self.wormholes[wh_idx].y1;
                let dist1 = (dx1 * dx1 + dy1 * dy1).sqrt();
                if dist1 < self.wormholes[wh_idx].capture_radius
                    && self.wormholes[wh_idx].cooldown1 <= 0.0
                {
                    let dest_x = self.wormholes[wh_idx].x2;
                    let dest_y = self.wormholes[wh_idx].y2;
                    let cap_r = self.wormholes[wh_idx].capture_radius;
                    let p = &mut self.particles[i];
                    let speed = (pvx * pvx + pvy * pvy).sqrt();
                    if speed > 0.01 {
                        p.x = dest_x + (pvx / speed) * (cap_r + PARTICLE_WORLD_RADIUS + 1.0);
                        p.y = dest_y + (pvy / speed) * (cap_r + PARTICLE_WORLD_RADIUS + 1.0);
                    } else {
                        p.x = dest_x;
                        p.y = dest_y;
                    }
                    p.trail.clear();
                    p.flash_timer = 0.15;
                    self.wormholes[wh_idx].cooldown2 = WORMHOLE_COOLDOWN;
                    break;
                }

                // Check mouth B
                let dx2 = px - self.wormholes[wh_idx].x2;
                let dy2 = py - self.wormholes[wh_idx].y2;
                let dist2 = (dx2 * dx2 + dy2 * dy2).sqrt();
                if dist2 < self.wormholes[wh_idx].capture_radius
                    && self.wormholes[wh_idx].cooldown2 <= 0.0
                {
                    let dest_x = self.wormholes[wh_idx].x1;
                    let dest_y = self.wormholes[wh_idx].y1;
                    let cap_r = self.wormholes[wh_idx].capture_radius;
                    let p = &mut self.particles[i];
                    let speed = (pvx * pvx + pvy * pvy).sqrt();
                    if speed > 0.01 {
                        p.x = dest_x + (pvx / speed) * (cap_r + PARTICLE_WORLD_RADIUS + 1.0);
                        p.y = dest_y + (pvy / speed) * (cap_r + PARTICLE_WORLD_RADIUS + 1.0);
                    } else {
                        p.x = dest_x;
                        p.y = dest_y;
                    }
                    p.trail.clear();
                    p.flash_timer = 0.15;
                    self.wormholes[wh_idx].cooldown1 = WORMHOLE_COOLDOWN;
                    break;
                }
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
                let (fax, fay) =
                    plummer_force(well.x, well.y, well.gm * 0.3, well.epsilon, mote.x, mote.y);
                ax += fax;
                ay += fay;
            }
            for bh in &self.black_holes {
                if !bh.active {
                    continue;
                }
                let (fax, fay) =
                    plummer_force(bh.x, bh.y, bh.gm * 0.2, bh.epsilon, mote.x, mote.y);
                ax += fax;
                ay += fay;
            }
            for rep in &self.repulsors {
                if !rep.active {
                    continue;
                }
                let (fax, fay) =
                    plummer_force(rep.x, rep.y, rep.gm * 0.3, rep.epsilon, mote.x, mote.y);
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
                VisualEffect::ExpandingRing {
                    ref mut age,
                    ref mut current_radius,
                    max_radius,
                    duration,
                    ..
                } => {
                    *age += FIXED_DT;
                    let progress = (*age / *duration).min(1.0);
                    *current_radius = *max_radius * progress;
                }
            }
        }

        // Remove expired
        self.effects.retain(|e| match e {
            VisualEffect::Burst { age, duration, .. } => *age < *duration,
            VisualEffect::Flash { age, duration, .. } => *age < *duration,
            VisualEffect::ExpandingRing { age, duration, .. } => *age < *duration,
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
            // Locked particles pulse gently at 1.5x scale
            if p.locked {
                let pulse = (self.elapsed_time * std::f64::consts::TAU).sin() * 0.05;
                p.scale = 1.5 + pulse;
            }
        }

        // Tick wormhole cooldowns and animation
        for wh in &mut self.wormholes {
            if wh.cooldown1 > 0.0 {
                wh.cooldown1 -= FIXED_DT;
                if wh.cooldown1 < 0.0 {
                    wh.cooldown1 = 0.0;
                }
            }
            if wh.cooldown2 > 0.0 {
                wh.cooldown2 -= FIXED_DT;
                if wh.cooldown2 < 0.0 {
                    wh.cooldown2 = 0.0;
                }
            }
            wh.anim_phase += FIXED_DT;
        }

        // Tick plasma current animation
        for pc in &mut self.plasma_currents {
            pc.anim_phase += FIXED_DT;
        }

        // Tick supernovas countdown and animation
        for sn in &mut self.supernovas {
            if !sn.detonated {
                sn.countdown -= FIXED_DT;
                sn.anim_phase += FIXED_DT;
            }
        }
    }

    fn update_supernovas(&mut self, engine: &mut Engine) {
        let mut detonations: Vec<(f64, f64, f64)> = Vec::new();

        for sn in &mut self.supernovas {
            if sn.detonated || sn.countdown > 0.0 {
                continue;
            }
            sn.detonated = true;
            detonations.push((sn.x, sn.y, sn.blast_radius));
        }

        for (sx, sy, blast_r) in &detonations {
            // Kill particles in blast radius
            for p in &mut self.particles {
                if !p.alive || p.scored || p.captured {
                    continue;
                }
                let dx = p.x - sx;
                let dy = p.y - sy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < *blast_r {
                    p.alive = false;
                }
            }

            // Visual: expanding ring
            self.effects.push(VisualEffect::ExpandingRing {
                x: *sx,
                y: *sy,
                current_radius: 0.0,
                max_radius: *blast_r,
                color: Color::from_rgba(255, 200, 50, 255),
                age: 0.0,
                duration: 0.3,
            });

            // Burst particles
            self.spawn_burst(*sx, *sy, Color::from_rgba(255, 180, 50, 255), 35, engine);

            // Screen effects
            self.spawn_flash(Color::from_rgba(255, 255, 200, 255), 0.3);
            engine.post_fx.shake_remaining = 0.3;
            engine.post_fx.shake_intensity = 8.0;
        }

        // Update alive count after detonations
        if !detonations.is_empty() {
            self.alive_count = self.particles.iter().filter(|p| p.alive && !p.scored).count() as u32;
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

    fn render_field_grid(&self, fb: &mut Framebuffer) {
        if self.field_grid.is_empty() {
            return;
        }
        let cell_size_screen = (WORLD_SIZE / FIELD_GRID_RES as f64) * self.scale;

        // Find max absolute potential for normalization
        let mut max_pot = 0.0_f64;
        for &pot in &self.field_grid {
            let abs_pot = pot.abs();
            if abs_pot > max_pot {
                max_pot = abs_pot;
            }
        }
        if max_pot < 1.0 {
            return;
        }

        for gy in 0..FIELD_GRID_RES {
            for gx in 0..FIELD_GRID_RES {
                let pot = self.field_grid[gy * FIELD_GRID_RES + gx];
                let normalized = pot / max_pot;
                let abs_n = normalized.abs();
                if abs_n < 0.03 {
                    continue;
                }

                let sx = gx as f64 * cell_size_screen;
                let sy = gy as f64 * cell_size_screen;

                let alpha = (abs_n * 55.0).min(50.0) as u8;
                let color = if pot > 0.0 {
                    // Attractive field: blue-cyan gradient
                    let t = abs_n.min(1.0);
                    Color::from_rgba(
                        (20.0 + 30.0 * t) as u8,
                        (40.0 + 80.0 * t) as u8,
                        (120.0 + 120.0 * t) as u8,
                        alpha,
                    )
                } else {
                    // Repulsive field: amber-yellow gradient
                    let t = abs_n.min(1.0);
                    Color::from_rgba(
                        (180.0 + 60.0 * t) as u8,
                        (100.0 + 80.0 * t) as u8,
                        (10.0 + 30.0 * t) as u8,
                        alpha,
                    )
                };

                shapes::fill_rect(
                    fb,
                    sx,
                    sy,
                    cell_size_screen + 1.0,
                    cell_size_screen + 1.0,
                    color,
                );
            }
        }
    }

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

            // Tint dust by nearby fields
            let mut r_tint = 180u16;
            let mut g_tint = 180u16;
            let mut b_tint = 220u16;

            for well in &self.gravity_wells {
                if !well.active {
                    continue;
                }
                let dx = mote.x - well.x;
                let dy = mote.y - well.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let influence = (1.0 - (dist / (well.visual_radius * 6.0)).min(1.0)).max(0.0);
                if influence > 0.05 {
                    b_tint = (b_tint + (60.0 * influence) as u16).min(255);
                }
            }
            for rep in &self.repulsors {
                if !rep.active {
                    continue;
                }
                let dx = mote.x - rep.x;
                let dy = mote.y - rep.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let influence = (1.0 - (dist / (rep.visual_radius * 6.0)).min(1.0)).max(0.0);
                if influence > 0.05 {
                    r_tint = (r_tint + (50.0 * influence) as u16).min(255);
                    g_tint = (g_tint + (30.0 * influence) as u16).min(255);
                }
            }
            for bh in &self.black_holes {
                if !bh.active {
                    continue;
                }
                let dx = mote.x - bh.x;
                let dy = mote.y - bh.y;
                let dist = (dx * dx + dy * dy).sqrt();
                let influence = (1.0 - (dist / (bh.visual_radius * 8.0)).min(1.0)).max(0.0);
                if influence > 0.05 {
                    r_tint = (r_tint + (60.0 * influence) as u16).min(255);
                    g_tint = g_tint.saturating_sub((40.0 * influence) as u16);
                    b_tint = b_tint.saturating_sub((30.0 * influence) as u16);
                }
            }

            let color = Color::from_rgba(r_tint as u8, g_tint as u8, b_tint as u8, alpha);
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
                let angle = rep.anim_phase * 0.5
                    + (k as f64) * std::f64::consts::TAU / spoke_count as f64;
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

            // Determine trail color (gold/orange tint for three-body)
            let trail_color_base = if p.attractor_count >= 2 {
                // Three-body indicator: gold/orange tint
                Color::from_rgba(
                    ((p.color.r as u16 + 200).min(255)) as u8,
                    ((p.color.g as u16 + 140).min(255) / 2) as u8,
                    (p.color.b as u16 / 3) as u8,
                    255,
                )
            } else {
                p.color
            };

            // Draw trail
            let trail_len = p.trail.len();
            if trail_len > 1 {
                for t in 0..trail_len - 1 {
                    let alpha = ((t as f64 / trail_len as f64) * 80.0) as u8;
                    let trail_color = trail_color_base.with_alpha(alpha);
                    let (sx0, sy0) = self.w2s(p.trail[t].0, p.trail[t].1);
                    let (sx1, sy1) = self.w2s(p.trail[t + 1].0, p.trail[t + 1].1);
                    shapes::draw_line(fb, sx0, sy0, sx1, sy1, trail_color);
                }
            }

            // Draw particle
            let (sx, sy) = self.w2s(p.x, p.y);
            let radius = PARTICLE_RADIUS * p.scale;

            {
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
            }

            // Locked particle: dramatic "READY TO SHOOT" state
            if p.locked {
                let t = self.elapsed_time;

                // Large pulsing outer halo
                let halo_pulse = (t * 3.0).sin() * 0.3 + 1.0;
                let halo_r = radius * 4.0 * halo_pulse;
                let halo_color = p.color.with_alpha(30);
                shapes::fill_circle(fb, sx, sy, halo_r, halo_color);
                let halo_color2 = p.color.with_alpha(50);
                shapes::draw_circle(fb, sx, sy, halo_r, halo_color2);

                // Spinning crosshair lines (4 cardinal + 4 diagonal)
                let spin = t * 1.5;
                let arm_len = radius * 5.0;
                let arm_inner = radius * 1.8;
                for k in 0..8 {
                    let angle = spin + (k as f64) * std::f64::consts::TAU / 8.0;
                    let alpha = if k % 2 == 0 { 120 } else { 60 };
                    let line_color = Color::from_rgba(255, 255, 200, alpha);
                    let x0 = sx + angle.cos() * arm_inner;
                    let y0 = sy + angle.sin() * arm_inner;
                    let x1 = sx + angle.cos() * arm_len;
                    let y1 = sy + angle.sin() * arm_len;
                    shapes::draw_line(fb, x0, y0, x1, y1, line_color);
                }

                // Concentric expanding rings (2 rings at different phases)
                for ring_i in 0..2 {
                    let phase = (t * 2.0 + ring_i as f64 * 0.5) % 1.0;
                    let ring_r = radius * 2.0 + phase * radius * 6.0;
                    let ring_alpha = ((1.0 - phase) * 100.0) as u8;
                    let ring_color = Color::from_rgba(255, 255, 180, ring_alpha);
                    shapes::draw_circle(fb, sx, sy, ring_r, ring_color);
                }

                // "DRAG TO SHOOT" text below
                let text_alpha = ((t * 2.5).sin() * 0.3 + 0.7).max(0.0);
                let text_color = Color::from_rgba(
                    255,
                    255,
                    180,
                    (text_alpha * 220.0) as u8,
                );
                text::draw_text_centered(
                    fb,
                    sx as i32,
                    sy as i32 + radius as i32 + 18,
                    "DRAG TO SHOOT",
                    text_color,
                    1,
                );
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

    fn render_waypoint_preview(&self, fb: &mut Framebuffer) {
        if let Some((wx, wy)) = self.waypoint_preview {
            let (sx, sy) = self.w2s(wx, wy);
            let capture_r = self.w2s_r(WAYPOINT_CAPTURE_RADIUS);

            // Outlined circle showing capture radius
            let preview_color = Color::from_rgba(255, 255, 100, 80);
            shapes::draw_dashed_circle(fb, sx, sy, capture_r, preview_color, 6.0);

            // Center crosshair
            let dot_color = Color::from_rgba(255, 255, 100, 120);
            shapes::fill_circle(fb, sx, sy, 3.0, dot_color);
        }
    }

    fn render_wormholes(&self, fb: &mut Framebuffer) {
        for wh in &self.wormholes {
            let (sx1, sy1) = self.w2s(wh.x1, wh.y1);
            let (sx2, sy2) = self.w2s(wh.x2, wh.y2);
            let r = self.w2s_r(wh.capture_radius);

            // Dashed connection line between mouths
            let line_color = wh.color.with_alpha(60);
            shapes::draw_dashed_circle(fb, sx1, sy1, r * 0.3, line_color, 4.0);
            shapes::draw_line(fb, sx1, sy1, sx2, sy2, line_color);

            // Mouth A: spinning double circles (counter-rotating)
            let phase_a = wh.anim_phase * 2.0;
            let phase_b = -wh.anim_phase * 1.5;
            let inner_r = r * 0.55;
            let outer_r = r * 0.9;

            let core_color = wh.color.with_alpha(180);
            let rim_color = wh.color.with_alpha(100);

            // Outer glow mouth A
            let glow_a = wh.color.with_alpha(40);
            shapes::fill_circle(fb, sx1, sy1, r, glow_a);
            // Inner circle (rotating dashes simulated as circle)
            shapes::draw_dashed_circle(fb, sx1, sy1, inner_r, core_color, 5.0 + phase_a.sin());
            // Outer ring
            shapes::draw_dashed_circle(fb, sx1, sy1, outer_r, rim_color, 5.0 + phase_b.sin());
            // Center dot
            shapes::fill_circle(fb, sx1, sy1, 3.0, core_color);

            // Mouth B: same but opposite phase
            let phase_a2 = -wh.anim_phase * 2.0;
            let phase_b2 = wh.anim_phase * 1.5;
            let glow_b = wh.color.with_alpha(40);
            shapes::fill_circle(fb, sx2, sy2, r, glow_b);
            shapes::draw_dashed_circle(fb, sx2, sy2, inner_r, core_color, 5.0 + phase_a2.sin());
            shapes::draw_dashed_circle(fb, sx2, sy2, outer_r, rim_color, 5.0 + phase_b2.sin());
            shapes::fill_circle(fb, sx2, sy2, 3.0, core_color);
        }
    }

    fn render_plasma_currents(&self, fb: &mut Framebuffer) {
        for pc in &self.plasma_currents {
            let dx = pc.x2 - pc.x1;
            let dy = pc.y2 - pc.y1;
            let len = (dx * dx + dy * dy).sqrt();
            if len < 1.0 {
                continue;
            }

            let nx = dx / len;
            let ny = dy / len;
            // Perpendicular
            let px = -ny;
            let py = nx;
            let hw = pc.half_width;

            // Boundary lines in dim cyan
            let boundary_color = Color::from_rgba(0, 180, 200, 60);
            let (bx1a, by1a) = self.w2s(pc.x1 + px * hw, pc.y1 + py * hw);
            let (bx2a, by2a) = self.w2s(pc.x2 + px * hw, pc.y2 + py * hw);
            let (bx1b, by1b) = self.w2s(pc.x1 - px * hw, pc.y1 - py * hw);
            let (bx2b, by2b) = self.w2s(pc.x2 - px * hw, pc.y2 - py * hw);
            shapes::draw_line(fb, bx1a, by1a, bx2a, by2a, boundary_color);
            shapes::draw_line(fb, bx1b, by1b, bx2b, by2b, boundary_color);

            // Animated flow dots moving from start to end
            let dot_spacing = 60.0;
            let num_dots = ((len / dot_spacing) as usize).max(1).min(20);
            let anim_offset = (pc.anim_phase * 0.4) % 1.0;
            let flow_color = Color::from_rgba(0, 220, 255, 120);

            for k in 0..num_dots {
                let t = ((k as f64 / num_dots as f64) + anim_offset) % 1.0;
                let wx = pc.x1 + nx * len * t;
                let wy = pc.y1 + ny * len * t;
                let (sdx, sdy) = self.w2s(wx, wy);
                let fade = (1.0 - (t - 0.5).abs() * 2.0).max(0.0);
                let dot_alpha = (120.0 * fade) as u8;
                let dc = flow_color.with_alpha(dot_alpha);
                shapes::fill_circle(fb, sdx, sdy, 2.0, dc);
            }
        }
    }

    fn render_supernovas(&self, fb: &mut Framebuffer) {
        for sn in &self.supernovas {
            if sn.detonated {
                continue;
            }
            let (sx, sy) = self.w2s(sn.x, sn.y);
            let vr = self.w2s_r(sn.visual_radius);
            let br = self.w2s_r(sn.blast_radius);

            // Countdown progress (0.0 = just started, 1.0 = about to blow)
            let progress = if sn.initial_countdown > 0.0 {
                1.0 - (sn.countdown / sn.initial_countdown)
            } else {
                1.0
            };

            // Star shape: 6 radiating spokes
            let spoke_count = 6;
            let spoke_color = Color::from_rgba(255, 160, 40, (120.0 + progress * 135.0).min(255.0) as u8);
            for k in 0..spoke_count {
                let angle = sn.anim_phase * 0.3 + (k as f64) * std::f64::consts::TAU / spoke_count as f64;
                let inner = vr * 0.2;
                let outer = vr * (0.6 + progress * 0.4);
                let x0 = sx + angle.cos() * inner;
                let y0 = sy + angle.sin() * inner;
                let x1 = sx + angle.cos() * outer;
                let y1 = sy + angle.sin() * outer;
                shapes::draw_line(fb, x0, y0, x1, y1, spoke_color);
            }

            // Core: hot white-yellow, flashing in final seconds
            let flash_hz = if sn.countdown < 2.0 { SUPERNOVA_FLASH_HZ_LATE } else { SUPERNOVA_FLASH_HZ_EARLY };
            let flash_on = if sn.countdown < 2.0 {
                (sn.anim_phase * flash_hz * std::f64::consts::TAU).sin() > 0.0
            } else {
                true
            };
            let core_alpha = if flash_on { (180.0 + progress * 75.0).min(255.0) as u8 } else { 80 };
            let core_color = Color::from_rgba(255, 240, 180, core_alpha);
            shapes::fill_circle(fb, sx, sy, vr * 0.25 * (1.0 + progress * 0.3), core_color);

            // Countdown ring: shrinks from blast_radius toward visual_radius
            let ring_radius = br - (br - vr) * progress;
            let ring_alpha = (40.0 + 160.0 * progress).min(255.0) as u8;
            let ring_thickness = 1.0 + 2.0 * progress;
            let ring_color = Color::from_rgba(255, 60, 40, ring_alpha);
            // Draw multiple circles for thickness
            shapes::draw_circle(fb, sx, sy, ring_radius, ring_color);
            if ring_thickness > 1.5 {
                shapes::draw_circle(fb, sx, sy, ring_radius - 1.0, ring_color.with_alpha(ring_alpha / 2));
            }
            if ring_thickness > 2.5 {
                shapes::draw_circle(fb, sx, sy, ring_radius + 1.0, ring_color.with_alpha(ring_alpha / 3));
            }

            // Countdown text
            let secs = sn.countdown.ceil() as i32;
            let countdown_text = format!("{}", secs);
            let text_color = Color::from_rgba(255, (200.0 * (1.0 - progress)).max(60.0) as u8, (200.0 * (1.0 - progress)).max(60.0) as u8, 220);
            text::draw_text_centered(fb, sx as i32, sy as i32 + vr as i32 + 8, &countdown_text, text_color, 1);
        }
    }

    fn render_aim_preview(&self, fb: &mut Framebuffer) {
        if let Some(ref sling) = self.sling {
            // Calculate launch velocity
            let dx = sling.anchor_x - sling.pull_x;
            let dy = sling.anchor_y - sling.pull_y;
            let pull_dist = (dx * dx + dy * dy).sqrt();
            if pull_dist < 5.0 {
                return;
            }

            let max_pull_w = SLING_MAX_PULL / self.scale;
            let power = (pull_dist / max_pull_w).min(1.0);
            let launch_speed = SLING_MAX_SPEED * power;

            if pull_dist < 0.01 {
                return;
            }
            let mut sim_vx = dx / pull_dist * launch_speed;
            let mut sim_vy = dy / pull_dist * launch_speed;
            let mut sim_x = sling.anchor_x;
            let mut sim_y = sling.anchor_y;

            let steps = 120;
            let sim_dt = FIXED_DT;

            for step in 0..steps {
                // No gravity in preview — player must read the field and guess the bend
                // Drag only
                let speed = (sim_vx * sim_vx + sim_vy * sim_vy).sqrt();
                let effective_drag = BASE_DRAG + SPEED_DRAG * speed;
                let factor = (-effective_drag * sim_dt).exp();
                sim_vx *= factor;
                sim_vy *= factor;

                sim_x += sim_vx * sim_dt;
                sim_y += sim_vy * sim_dt;

                // Edge bounce
                if sim_x < 0.0 {
                    sim_x = -sim_x;
                    sim_vx = -sim_vx * EDGE_RESTITUTION;
                }
                if sim_x > WORLD_SIZE {
                    sim_x = 2.0 * WORLD_SIZE - sim_x;
                    sim_vx = -sim_vx * EDGE_RESTITUTION;
                }
                if sim_y < 0.0 {
                    sim_y = -sim_y;
                    sim_vy = -sim_vy * EDGE_RESTITUTION;
                }
                if sim_y > WORLD_SIZE {
                    sim_y = 2.0 * WORLD_SIZE - sim_y;
                    sim_vy = -sim_vy * EDGE_RESTITUTION;
                }
                sim_x = sim_x.clamp(0.0, WORLD_SIZE);
                sim_y = sim_y.clamp(0.0, WORLD_SIZE);

                // Check for black hole kill zone hit
                let mut hit_bh = false;
                for bh in &self.black_holes {
                    if !bh.active { continue; }
                    let bdx = sim_x - bh.x;
                    let bdy = sim_y - bh.y;
                    if bdx * bdx + bdy * bdy < bh.kill_radius * bh.kill_radius {
                        hit_bh = true;
                        break;
                    }
                }

                // Check for target hit
                let mut hit_target = false;
                for target in &self.targets {
                    if !target.active { continue; }
                    let tdx = sim_x - target.x;
                    let tdy = sim_y - target.y;
                    if tdx * tdx + tdy * tdy < target.hit_radius * target.hit_radius {
                        hit_target = true;
                        break;
                    }
                }

                // Draw dot every 3 steps
                if step % 3 == 0 {
                    let alpha = (200.0 * (1.0 - step as f64 / steps as f64)) as u8;
                    let (sx, sy) = self.w2s(sim_x, sim_y);
                    let c = Color::from_rgba(255, 255, 200, alpha);
                    shapes::fill_circle(fb, sx, sy, 1.5, c);
                }

                // Draw danger/success markers and stop
                if hit_bh {
                    let (sx, sy) = self.w2s(sim_x, sim_y);
                    let red = Color::from_rgba(255, 60, 60, 220);
                    // Red X
                    shapes::draw_line(fb, sx - 5.0, sy - 5.0, sx + 5.0, sy + 5.0, red);
                    shapes::draw_line(fb, sx + 5.0, sy - 5.0, sx - 5.0, sy + 5.0, red);
                    break;
                }
                if hit_target {
                    let (sx, sy) = self.w2s(sim_x, sim_y);
                    let green = Color::from_rgba(80, 255, 80, 200);
                    shapes::draw_circle(fb, sx, sy, 6.0, green);
                    shapes::draw_circle(fb, sx, sy, 4.0, green);
                    break;
                }
            }
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
                VisualEffect::ExpandingRing {
                    x, y, current_radius, color, age, duration, ..
                } => {
                    let frac = if *duration > 0.0 { (1.0 - age / duration).max(0.0) } else { 0.0 };
                    let alpha = (frac * 255.0) as u8;
                    if alpha > 0 {
                        let (sx, sy) = self.w2s(*x, *y);
                        let r = self.w2s_r(*current_radius);
                        let c = color.with_alpha(alpha);
                        shapes::draw_circle(fb, sx, sy, r, c);
                        if r > 2.0 {
                            shapes::draw_circle(fb, sx, sy, r - 1.0, c.with_alpha(alpha / 2));
                        }
                    }
                }
            }
        }
    }

    fn render_hud(&self, fb: &mut Framebuffer) {
        // Score counter
        let score_text = format!("{}/{}", self.total_scored, self.goal_target);
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
                let names = level_names();
                let next_level = self.current_level + 1;
                if next_level < names.len() {
                    text::draw_text_centered(
                        fb,
                        cx,
                        cy + 40,
                        names[next_level],
                        Color::from_rgba(180, 180, 220, 200),
                        2,
                    );
                }
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
                let names = level_names();
                let next_level = self.current_level + 1;
                if next_level < names.len() {
                    text::draw_text_centered(
                        fb,
                        cx,
                        cy + 30,
                        names[next_level],
                        Color::from_rgba(180, 180, 220, 200),
                        2,
                    );
                }
            }
            GamePhase::Playing => {}
        }
    }
}

// ─── Simulation trait ───────────────────────────────────────────────────────

impl Simulation for GravityPong {
    fn setup(&mut self, engine: &mut Engine) {
        engine.config.bounds = (800.0, 800.0);
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

        // 5b. Supernova detonation
        self.update_supernovas(engine);

        // 6. Win/Loss
        self.check_win_loss(engine);

        // 7. Phase transitions
        self.update_phase(engine);

        // 8. Elapsed time (only during active play)
        if self.phase == GamePhase::Playing {
            self.elapsed_time += FIXED_DT;
        }

        // 9. HUD state for JS
        engine.global_state.set_f64("scored", self.total_scored as f64);
        engine.global_state.set_f64("goal", self.goal_target as f64);
        engine.global_state.set_f64("level", (self.current_level + 1) as f64);
        engine.global_state.set_f64("time", self.elapsed_time);
    }

    fn render(&self, engine: &mut Engine) {
        let fb = &mut engine.framebuffer;

        // 1. Background already cleared by engine config

        // 2. Vignette
        post_fx::vignette(fb, 0.3);

        // 2b. Field contour visualization
        self.render_field_grid(fb);

        // 3. Dust motes
        self.render_dust(fb);

        // 4-6. Gravity wells, repulsors, black holes
        self.render_gravity_wells(fb);
        self.render_repulsors(fb);
        self.render_black_holes(fb);

        // 7. Walls
        self.render_walls(fb);

        // 8. Wormholes
        self.render_wormholes(fb);

        // 9. Plasma currents
        self.render_plasma_currents(fb);

        // 9b. Supernovas
        self.render_supernovas(fb);

        // 10. Targets
        self.render_targets(fb);

        // 11. Particles (with trails)
        self.render_particles(fb);

        // 12. Waypoint UI
        self.render_waypoint(fb);

        // 13. Sling UI
        self.render_sling(fb);

        // 13b. Waypoint placement preview
        self.render_waypoint_preview(fb);

        // 14. Aim preview trajectory
        self.render_aim_preview(fb);

        // 15. Visual effects
        self.render_effects(fb);

        // 16. HUD
        self.render_hud(fb);

        // 17. Tutorial hint (first level, first few seconds)
        if self.current_level == 0 && self.elapsed_time < 6.0 && self.phase == GamePhase::Playing {
            let cx = fb.width as i32 / 2;
            let alpha = if self.elapsed_time < 4.0 {
                180
            } else {
                (180.0 * (1.0 - (self.elapsed_time - 4.0) / 2.0)).max(0.0) as u8
            };
            let hint_color = Color::from_rgba(200, 200, 230, alpha);
            text::draw_text_centered(
                fb,
                cx,
                fb.height as i32 - 50,
                "Tap to place waypoint  |  Drag anywhere to aim & shoot",
                hint_color,
                1,
            );
        }

        // 18. Phase overlays
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

/// Plasma current force: returns (ax, ay) acceleration for a particle in the corridor.
fn plasma_current_force(
    pc: &PlasmaCurrent,
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,
) -> (f64, f64) {
    let dx = pc.x2 - pc.x1;
    let dy = pc.y2 - pc.y1;
    let len_sq = dx * dx + dy * dy;
    if len_sq < 1e-10 {
        return (0.0, 0.0);
    }
    let len = len_sq.sqrt();

    // Project particle onto line
    let t = ((px - pc.x1) * dx + (py - pc.y1) * dy) / len_sq;

    // Only apply force if within the segment
    if t < 0.0 || t > 1.0 {
        return (0.0, 0.0);
    }

    // Closest point on line
    let closest_x = pc.x1 + t * dx;
    let closest_y = pc.y1 + t * dy;

    // Perpendicular distance
    let perp_dx = px - closest_x;
    let perp_dy = py - closest_y;
    let perp_dist = (perp_dx * perp_dx + perp_dy * perp_dy).sqrt();

    // Only apply within corridor half-width
    if perp_dist > pc.half_width {
        return (0.0, 0.0);
    }

    // Gaussian falloff
    let sigma = pc.half_width / 2.0;
    let ratio = perp_dist / sigma;
    let factor = (-ratio * ratio).exp();

    // Force direction = normalize(end - start)
    let dir_x = dx / len;
    let dir_y = dy / len;

    // Velocity-dependent coupling
    let v_len = (vx * vx + vy * vy).sqrt();
    let dot = if v_len > 1e-10 {
        (vx / v_len) * dir_x + (vy / v_len) * dir_y
    } else {
        0.0
    };
    let effective = pc.strength * (1.0 - 0.5 * dot);

    (dir_x * effective * factor, dir_y * effective * factor)
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
        1 => Color::from_rgba(80, 200, 255, 255),  // sky blue
        2 => Color::from_rgba(255, 80, 160, 255),  // pink
        3 => Color::from_rgba(120, 255, 120, 255), // lime
        4 => Color::from_rgba(200, 140, 255, 255), // lavender
        5 => Color::from_rgba(255, 220, 80, 255),  // gold
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
    fn level_data_has_ten_levels() {
        let levels = level_data();
        assert_eq!(levels.len(), 10);
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
        assert_eq!(gp.current_level, 9); // clamped to last level (index 9 = level 10)
    }

    #[test]
    fn wormhole_from_params() {
        let wh = Wormhole::from_params(100.0, 200.0, 800.0, 700.0);
        assert!((wh.x1 - 100.0).abs() < 1e-10);
        assert!((wh.y1 - 200.0).abs() < 1e-10);
        assert!((wh.x2 - 800.0).abs() < 1e-10);
        assert!((wh.y2 - 700.0).abs() < 1e-10);
        assert!((wh.capture_radius - WORMHOLE_CAPTURE_RADIUS).abs() < 1e-10);
        assert_eq!(wh.cooldown1, 0.0);
        assert_eq!(wh.cooldown2, 0.0);
    }

    #[test]
    fn plasma_current_from_params_maps_strength() {
        let pc = PlasmaCurrent::from_params(0.0, 0.0, 1000.0, 0.0, 100.0, 1.0);
        assert!((pc.strength - 30.0).abs() < 1e-6, "strength=1 maps to 30.0");
        let pc2 = PlasmaCurrent::from_params(0.0, 0.0, 1000.0, 0.0, 100.0, 100.0);
        assert!((pc2.strength - 200.0).abs() < 1e-6, "strength=100 maps to 200.0");
        assert!((pc.half_width - 50.0).abs() < 1e-10);
    }

    #[test]
    fn plasma_current_force_zero_outside_corridor() {
        let pc = PlasmaCurrent::from_params(0.0, 500.0, 1000.0, 500.0, 100.0, 50.0);
        // Far outside half-width (half_width = 50, particle is 200 away)
        let (ax, ay) = plasma_current_force(&pc, 500.0, 300.0, 0.0, 0.0);
        assert_eq!(ax, 0.0);
        assert_eq!(ay, 0.0);
    }

    #[test]
    fn plasma_current_force_nonzero_inside_corridor() {
        let pc = PlasmaCurrent::from_params(0.0, 500.0, 1000.0, 500.0, 200.0, 50.0);
        // Within corridor (half_width = 100, particle is at center)
        let (ax, _ay) = plasma_current_force(&pc, 500.0, 500.0, 0.0, 0.0);
        assert!(ax > 0.0, "force along current direction should be positive");
    }

    #[test]
    fn supernova_from_params() {
        let sn = Supernova::from_params(500.0, 300.0, 50.0, 5.0);
        assert!(sn.blast_radius > sn.visual_radius);
        assert!(!sn.detonated);
        assert!((sn.countdown - 5.0).abs() < 1e-10);
    }

    #[test]
    fn elapsed_time_initialises_zero() {
        let gp = GravityPong::new();
        assert_eq!(gp.elapsed_time, 0.0);
    }

    #[test]
    fn load_level_resets_elapsed_time() {
        let mut gp = GravityPong::new();
        let mut engine = Engine::new(640, 640);
        gp.screen_w = 640.0;
        gp.screen_h = 640.0;
        gp.scale = 0.64;
        gp.elapsed_time = 99.0;
        gp.load_level(0, &mut engine);
        assert_eq!(gp.elapsed_time, 0.0);
    }
}
