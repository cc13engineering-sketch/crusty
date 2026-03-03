use crate::ecs::World;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::color::Color;
use crate::rendering::particles::ParticlePool;
use crate::rendering::starfield::Starfield;
use crate::rendering::post_fx::PostFxConfig;
use crate::rendering::layers::RenderLayerStack;
use crate::rendering::sprite::SpriteSheet;
use crate::rendering::transition::TransitionManager;
use crate::rendering::screen_fx::ScreenFxStack;
use crate::input::Input;
use crate::events::EventQueue;
use crate::spawn_queue::SpawnQueue;
use crate::game_state::GameState as GlobalGameState;
use crate::timers::TimerQueue;
use crate::templates::TemplateRegistry;
use crate::behavior::BehaviorRules;
use crate::dialogue::DialogueQueue;
use crate::scene_manager::SceneManager;
use crate::tilemap::TileMap;
use crate::entity_pool::PoolRegistry;
use crate::event_bus::EventBus;
use crate::input_map::InputMap;
use crate::flow_network::FlowNetwork;
use crate::environment_clock::EnvironmentClock;
use crate::diagnostics::DiagnosticBus;
use crate::gesture::GestureRecognizer;
use crate::auto_juice::AutoJuiceSystem;
use crate::game_flow::GameFlow;
use crate::camera_director::CameraDirector;
use crate::level_curve::LevelCurve;
use crate::ui_canvas::UiCanvas;
use crate::color_palette::ColorPalette;
use crate::frame_metrics::FrameMetrics;
use crate::rng::SeededRng;

/// Defines the canonical execution phases within a single engine tick.
///
/// Systems are grouped into phases that run in a fixed order every frame.
/// This enum exists to document and lock down that order for portability
/// and stability -- changing the sequence of phases (or moving a system
/// between phases) is a deliberate, breaking decision.
///
/// The phases execute in the order of their discriminant values:
///
/// | Phase          | Purpose                                                         |
/// |----------------|-----------------------------------------------------------------|
/// | `Input`        | Gather and pre-process player input, gestures, debug toggles.   |
/// | `Simulation`   | Fixed-dt logical update: lifecycle, AI, animation, state machines. |
/// | `Physics`      | Fixed-timestep physics: forces, integration, collision, joints. |
/// | `PostPhysics`  | Gameplay reactions, spawning, particles, camera, game-over.     |
/// | `RenderingPrep`| All drawing: background, entities, HUD, post-fx, cleanup.      |
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SystemPhase {
    /// Phase 0 -- Runs once at the start of `tick()`.
    ///
    /// Systems:
    /// - Debug-mode toggle (KeyD)
    /// - `GestureRecognizer::update` -- advance gesture timers
    /// - Drain recognized gestures and publish to `EventBus`
    Input = 0,

    /// Phase 1 -- Fixed-dt logical update (FIXED_DT = 1/60s).
    ///
    /// Systems (in order):
    /// - `lifecycle::run`       -- spawns, despawns, lifetimes, timers, behavior rules
    /// - `hierarchy::run`       -- parent-to-child transform propagation
    /// - `signal::run`          -- wire emitters to receivers
    /// - `state_machine::run`   -- tick elapsed time, check transitions
    /// - `coroutine::run`       -- advance async behavior steps
    /// - Drain coroutine-queued spawns into engine `SpawnQueue`
    /// - `EnvironmentClock::tick` -- day/night cycle, seasons
    /// - `FlowNetwork::solve`   -- transfer resources along edges
    /// - `sprite_animator::run` -- advance sprite frame timers
    /// - `behavior::run`        -- AI movement (chase, flee, patrol, etc.)
    /// - `tween::run`           -- easing-curve property animation
    /// - `flash::run`           -- hit flash, blink, color pulse
    /// - `waypoint::run`        -- path-following movement
    Simulation = 1,

    /// Phase 2 -- Fixed-timestep physics loop (`FIXED_DT = 1/60 s`).
    ///
    /// Runs zero or more times per tick depending on the accumulator.
    /// Each iteration executes:
    /// - `force_accumulator::run` -- sum external forces
    /// - `integrator::run`        -- semi-implicit Euler integration
    /// - `collision::run`         -- broadphase + narrowphase, emit collision events
    /// - `physics_joint::run`     -- distance, spring, rope, hinge constraints
    Physics = 2,

    /// Phase 3 -- Runs once after physics. Gameplay uses FIXED_DT; rendering uses variable dt.
    ///
    /// Systems (in order):
    /// - `gameplay::run`          -- collision reactions, scoring, damage
    /// - `event_processor::run`   -- legacy non-gameplay event triggers
    /// - `input_gameplay::run`    -- input-driven gameplay actions
    /// - (game logic)             -- game-specific spawners, cooldowns (via Simulation trait)
    /// - `ghost_trail::run`       -- capture position snapshots for trails
    /// - `ParticlePool::update`   -- tick particle lifetimes and positions
    /// - `TransitionManager::update` -- advance scene transitions
    /// - `DialogueQueue::tick`    -- advance dialogue message timers
    /// - `check_game_over`        -- detect player death
    /// - `Camera::update`         -- follow target, smooth, clamp to bounds
    PostPhysics = 3,

    /// Phase 4 -- All rendering and end-of-frame cleanup.
    ///
    /// Rendering (in order):
    /// - `Framebuffer::clear`     -- fill with background color
    /// - `Starfield::render`      -- optional parallax starfield
    /// - `renderer::run_entities_only` -- draw all visible entities
    /// - `ParticlePool::render`   -- draw particles
    /// - `debug_render::run`      -- debug overlays (when `debug_mode` is on)
    /// - `render_hud`             -- HUD bars, score, ammo, dialogue, game-over overlay
    /// - `ScreenFxStack::tick` + `apply` -- screen-wide tint, desaturate, flash
    /// - `TransitionManager::apply` -- scene transition overlay
    /// - `post_fx::apply`         -- CRT scanlines, shake, bloom, etc.
    ///
    /// Cleanup:
    /// - `EventQueue::clear`      -- drain frame events
    /// - `EventBus::clear`        -- drain bus events
    /// - `Input::end_frame`       -- reset per-frame input state
    /// - `DiagnosticBus::clear` + `run_checks` -- runtime diagnostics
    /// - Advance `time` and `frame` counters
    RenderingPrep = 4,
}

#[derive(Clone, Debug)]
pub struct WorldConfig {
    pub name: String,
    pub bounds: (f64, f64),
    pub background: Color,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self { name: "Untitled".into(), bounds: (960.0, 540.0), background: Color::BLACK }
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub target_tag: Option<String>,
    pub smoothing: f64,
    pub clamp_to_bounds: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 1.0,
            target_tag: None,
            smoothing: 0.0,
            clamp_to_bounds: false,
        }
    }
}

impl Camera {
    /// Original world-to-screen conversion (no zoom). Kept for backward compatibility.
    pub fn world_to_screen(&self, wx: f64, wy: f64) -> (i32, i32) {
        ((wx - self.x).round() as i32, (wy - self.y).round() as i32)
    }

    /// World-to-screen conversion that accounts for zoom, centered on the viewport.
    /// `sw` and `sh` are the screen (viewport) width and height.
    pub fn world_to_screen_zoomed(&self, wx: f64, wy: f64, sw: f64, sh: f64) -> (i32, i32) {
        let cx = sw / 2.0;
        let cy = sh / 2.0;
        let sx = (wx - self.x) * self.zoom + cx;
        let sy = (wy - self.y) * self.zoom + cy;
        (sx.round() as i32, sy.round() as i32)
    }

    /// Smoothly follow the target entity and clamp to world bounds.
    /// `bounds` is (world_width, world_height), `viewport` is (screen_width, screen_height).
    pub fn update(&mut self, world: &crate::ecs::World, bounds: (f64, f64), viewport: (u32, u32), dt: f64) {
        // Follow the target entity by tag
        if let Some(ref tag) = self.target_tag {
            let mut target_pos: Option<(f64, f64)> = None;
            for (entity, tags) in world.tags.iter() {
                if tags.has(tag) {
                    if let Some(t) = world.transforms.get(entity) {
                        target_pos = Some((t.x, t.y));
                        break;
                    }
                }
            }

            if let Some((tx, ty)) = target_pos {
                // The camera position represents the world coordinate at the center of the viewport
                let vw = viewport.0 as f64;
                let vh = viewport.1 as f64;
                let desired_x = tx - vw / (2.0 * self.zoom);
                let desired_y = ty - vh / (2.0 * self.zoom);

                if self.smoothing <= 0.0 {
                    // Instant follow
                    self.x = desired_x;
                    self.y = desired_y;
                } else {
                    // Lerp toward the target; higher smoothing = slower convergence
                    let t = (dt / self.smoothing).min(1.0);
                    self.x += (desired_x - self.x) * t;
                    self.y += (desired_y - self.y) * t;
                }
            }
        }

        // Clamp camera to world bounds
        if self.clamp_to_bounds {
            let vw = viewport.0 as f64 / self.zoom;
            let vh = viewport.1 as f64 / self.zoom;
            let (bw, bh) = bounds;

            // If the viewport is smaller than the world, clamp so the camera
            // doesn't show outside the world. Otherwise center the world.
            if vw < bw {
                if self.x < 0.0 { self.x = 0.0; }
                if self.x + vw > bw { self.x = bw - vw; }
            } else {
                self.x = (bw - vw) / 2.0;
            }

            if vh < bh {
                if self.y < 0.0 { self.y = 0.0; }
                if self.y + vh > bh { self.y = bh - vh; }
            } else {
                self.y = (bh - vh) / 2.0;
            }
        }
    }
}

pub struct Engine {
    pub world: World,
    pub framebuffer: Framebuffer,
    pub input: Input,
    pub events: EventQueue,
    pub config: WorldConfig,
    pub camera: Camera,
    pub width: u32,
    pub height: u32,
    pub time: f64,
    pub frame: u64,
    pub debug_mode: bool,
    accumulator: f64,

    // Innovation features
    pub spawn_queue: SpawnQueue,
    pub particles: ParticlePool,
    pub starfield: Option<Starfield>,
    pub post_fx: PostFxConfig,

    // Entity Lifecycle & Runtime Dynamics (Innovation Agent 4)
    pub global_state: GlobalGameState,
    pub timers: TimerQueue,
    pub templates: TemplateRegistry,
    pub rules: BehaviorRules,

    // Innovation Round 2: Render layers + Sprite sheets
    pub layers: RenderLayerStack,
    pub sprite_sheets: Vec<SpriteSheet>,

    // Innovation Round 2: Scene transitions + Dialogue
    pub transition: TransitionManager,
    pub dialogue: DialogueQueue,

    // Innovation Round 3: Animation, effects, gameplay logic, scene management
    pub screen_fx: ScreenFxStack,
    pub scene_manager: SceneManager,

    // Innovation Round 4: Spatial systems, tile maps, entity pooling
    pub tilemap: Option<TileMap>,
    pub pool_registry: PoolRegistry,

    // Innovation Round 6: Event bus, input mapping
    pub event_bus: EventBus,
    pub input_map: InputMap,

    // Innovation Round 7: Flow network, environment clock
    pub flow_network: FlowNetwork,
    pub environment_clock: EnvironmentClock,

    // SoundScape: command-buffer audio system
    pub sound_queue: crate::sound::SoundCommandQueue,
    pub sound_palette: crate::sound::SoundPalette,

    // Diagnostics: structured runtime error reporting
    pub diagnostic_bus: DiagnosticBus,

    // Touch gesture recognition
    pub gestures: GestureRecognizer,

    // AutoJuice: automatic game-feel pipeline
    pub auto_juice: AutoJuiceSystem,

    // GameFlow: declarative game lifecycle state machine
    pub game_flow: GameFlow,

    // CameraDirector: cinematic camera orchestration
    pub camera_director: CameraDirector,

    // LevelCurve: progression and difficulty scaling
    pub level_curve: LevelCurve,

    // ColorPalette: procedural art identity system
    pub color_palette: ColorPalette,

    // UiCanvas: declarative UI widget overlay
    pub ui_canvas: UiCanvas,

    // Performance telemetry
    pub frame_metrics: FrameMetrics,

    // Canonical deterministic RNG
    pub rng: SeededRng,
}

const FIXED_DT: f64 = 1.0 / 60.0;
const MAX_FRAME_DT: f64 = 0.05;

impl Engine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            world: World::new(),
            framebuffer: Framebuffer::new(width, height),
            input: Input::new(),
            events: EventQueue::default(),
            config: WorldConfig::default(),
            camera: Camera::default(),
            width,
            height,
            time: 0.0,
            frame: 0,
            debug_mode: true,
            accumulator: 0.0,
            spawn_queue: SpawnQueue::default(),
            particles: ParticlePool::new(),
            starfield: None,
            post_fx: PostFxConfig::default(),
            global_state: GlobalGameState::new(),
            timers: TimerQueue::new(),
            templates: TemplateRegistry::new(),
            rules: BehaviorRules::new(),
            layers: RenderLayerStack::new(),
            sprite_sheets: Vec::new(),
            transition: TransitionManager::new(),
            dialogue: DialogueQueue::new(),
            screen_fx: ScreenFxStack::new(),
            scene_manager: SceneManager::new(),
            tilemap: None,
            pool_registry: PoolRegistry::new(),
            event_bus: EventBus::new(),
            input_map: InputMap::new(),
            flow_network: FlowNetwork::new(),
            environment_clock: EnvironmentClock::new(),
            sound_queue: crate::sound::SoundCommandQueue::new(),
            sound_palette: crate::sound::SoundPalette::default_palette(),
            diagnostic_bus: DiagnosticBus::new(),
            gestures: GestureRecognizer::new(),
            auto_juice: AutoJuiceSystem::new(),
            game_flow: GameFlow::new(),
            camera_director: CameraDirector::new(),
            level_curve: LevelCurve::new(),
            color_palette: ColorPalette::default(),
            ui_canvas: UiCanvas::new(),
            frame_metrics: FrameMetrics::new(),
            rng: SeededRng::new(42),
        }
    }

    /// Compute a deterministic hash of the simulation state (FNV-1a).
    ///
    /// Hashes: entity count, all transforms (sorted by entity ID),
    /// all rigidbodies (sorted by entity ID), global_state contents
    /// (sorted by key), frame counter, and rng state.
    ///
    /// Deliberately excludes rendering state (framebuffer, particles,
    /// starfield, post_fx) so the hash reflects simulation truth only.
    pub fn state_hash(&self) -> u64 {
        const FNV_OFFSET: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;

        let mut hash = FNV_OFFSET;

        // Helper: fold a u64 into the hash
        macro_rules! hash_u64 {
            ($val:expr) => {{
                let bytes = ($val).to_le_bytes();
                for &b in &bytes {
                    hash ^= b as u64;
                    hash = hash.wrapping_mul(FNV_PRIME);
                }
            }};
        }

        // Helper: fold an f64 into the hash via its bit pattern
        macro_rules! hash_f64 {
            ($val:expr) => {
                hash_u64!(($val).to_bits())
            };
        }

        // Entity count
        hash_u64!(self.world.transforms.len() as u64);

        // Transforms (sorted by entity ID for determinism)
        let sorted_t = self.world.transforms.sorted_entities();
        for e in &sorted_t {
            hash_u64!(e.0);
            if let Some(t) = self.world.transforms.get(*e) {
                hash_f64!(t.x);
                hash_f64!(t.y);
                hash_f64!(t.rotation);
                hash_f64!(t.scale);
            }
        }

        // RigidBodies (sorted by entity ID for determinism)
        hash_u64!(self.world.rigidbodies.len() as u64);
        let sorted_rb = self.world.rigidbodies.sorted_entities();
        for e in &sorted_rb {
            hash_u64!(e.0);
            if let Some(rb) = self.world.rigidbodies.get(*e) {
                hash_f64!(rb.vx);
                hash_f64!(rb.vy);
                hash_f64!(rb.mass);
            }
        }

        // Global state (sorted by key for determinism)
        let mut keys: Vec<&str> = self.global_state.iter().map(|(k, _)| k).collect();
        keys.sort();
        hash_u64!(keys.len() as u64);
        for key in &keys {
            for &b in key.as_bytes() {
                hash ^= b as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
            }
            if let Some(val) = self.global_state.get(key) {
                match val {
                    crate::game_state::StateValue::F64(v) => {
                        hash_u64!(0u64); // type tag
                        hash_f64!(*v);
                    }
                    crate::game_state::StateValue::Bool(v) => {
                        hash_u64!(1u64);
                        hash_u64!(if *v { 1u64 } else { 0u64 });
                    }
                    crate::game_state::StateValue::Str(v) => {
                        hash_u64!(2u64);
                        for &b in v.as_bytes() {
                            hash ^= b as u64;
                            hash = hash.wrapping_mul(FNV_PRIME);
                        }
                    }
                }
            }
        }

        // Frame counter
        hash_u64!(self.frame);

        // RNG state
        hash_u64!(self.rng.state);

        hash
    }

    /// Reset the engine to a clean initial state with the given RNG seed.
    ///
    /// This is the single entry point for reproducible simulation. Every
    /// headless run should start here. Clears all world state, all subsystems,
    /// reseeds the RNG, and resets frame/time counters to zero.
    pub fn reset(&mut self, seed: u64) {
        // Clear all entities and components
        self.world.clear();

        // Clear all subsystems
        self.particles = ParticlePool::new();
        self.global_state.clear();
        self.timers.clear();
        self.rules.clear();
        self.dialogue.clear();
        self.transition = TransitionManager::new();
        self.screen_fx.clear();
        self.event_bus.clear();
        self.flow_network.clear();
        self.environment_clock = EnvironmentClock::new();
        self.sound_queue.clear();
        self.diagnostic_bus.clear();
        self.auto_juice.clear();
        self.game_flow.clear();
        self.camera_director.clear();
        self.level_curve.clear();
        self.color_palette = ColorPalette::default();
        self.ui_canvas.clear();
        self.spawn_queue.clear();
        self.events.clear();
        self.input.end_frame();
        self.scene_manager.clear();
        self.gestures = GestureRecognizer::new();
        self.camera = Camera::default();

        // Reseed RNG
        self.rng = SeededRng::new(seed);

        // Reset timing
        self.frame = 0;
        self.time = 0.0;
        self.accumulator = 0.0;
    }

    /// Advance the engine by one frame.
    ///
    /// `dt` is the wall-clock delta in seconds since the last call (clamped
    /// internally to [`MAX_FRAME_DT`]).
    ///
    /// # System execution order
    ///
    /// The tick is split into five sequential phases (see [`SystemPhase`] for
    /// the authoritative reference). Changing this order is a breaking change.
    ///
    /// ```text
    /// Phase 0  Input
    ///   |- debug toggle (KeyD)
    ///   |- GestureRecognizer::update
    ///   '- drain gestures -> EventBus
    ///
    /// Phase 1  Simulation  (FIXED_DT)
    ///   |- lifecycle::run        (spawns, despawns, lifetimes, timers, rules)
    ///   |- hierarchy::run        (parent->child transforms)
    ///   |- signal::run           (emitter->receiver wiring)
    ///   |- state_machine::run    (FSM transitions)
    ///   |- coroutine::run        (async behavior steps)
    ///   |- drain coroutine spawns
    ///   |- EnvironmentClock::tick (day/night, seasons)
    ///   |- FlowNetwork::solve    (resource transfer)
    ///   |- sprite_animator::run  (frame timers)
    ///   |- behavior::run         (AI movement)
    ///   |- tween::run            (easing animation)
    ///   |- flash::run            (hit flash, blink)
    ///   '- waypoint::run         (path following)
    ///
    /// Phase 2  Physics  (fixed dt, 0..N iterations)
    ///   |- force_accumulator::run
    ///   |- integrator::run
    ///   |- collision::run
    ///   '- physics_joint::run
    ///
    /// Phase 3  PostPhysics  (FIXED_DT for gameplay, variable dt for rendering)
    ///   |- gameplay::run         (collision reactions, scoring, damage)
    ///   |- event_processor::run  (legacy event triggers)
    ///   |- input_gameplay::run   (input-driven actions)
    ///   |- (game-specific logic via Simulation trait)
    ///   |- ghost_trail::run      (position snapshots)
    ///   |- ParticlePool::update
    ///   |- TransitionManager::update
    ///   |- DialogueQueue::tick
    ///   '- Camera::update        (follow, smooth, clamp)
    ///
    /// Phase 4  RenderingPrep
    ///   |- Framebuffer::clear
    ///   |- Starfield::render
    ///   |- renderer::run_entities_only
    ///   |- ParticlePool::render
    ///   |- debug_render::run     (if debug_mode)
    ///   |- ScreenFxStack tick + apply
    ///   |- TransitionManager::apply
    ///   |- post_fx::apply
    ///   |- EventQueue::clear + EventBus::clear + Input::end_frame
    ///   |- DiagnosticBus clear + run_checks
    ///   '- advance time + frame
    /// ```
    pub fn tick(&mut self, dt: f64) {
        let dt = dt.min(MAX_FRAME_DT);
        self.accumulator += dt;
        self.accumulator = self.accumulator.min(5.0 * FIXED_DT);

        if self.input.keys_pressed.contains("KeyD") {
            self.debug_mode = !self.debug_mode;
        }

        // Touch gesture recognition: update timers and drain recognized gestures
        self.gestures.update(dt);
        for gesture in self.gestures.drain_gestures() {
            match &gesture {
                crate::gesture::Gesture::Tap { x, y } => {
                    self.event_bus.publish(
                        crate::event_bus::BusEvent::new("gesture:tap")
                            .with_f64("x", *x)
                            .with_f64("y", *y)
                    );
                }
                crate::gesture::Gesture::DoubleTap { x, y } => {
                    self.event_bus.publish(
                        crate::event_bus::BusEvent::new("gesture:double_tap")
                            .with_f64("x", *x)
                            .with_f64("y", *y)
                    );
                }
                crate::gesture::Gesture::LongPress { x, y } => {
                    self.event_bus.publish(
                        crate::event_bus::BusEvent::new("gesture:long_press")
                            .with_f64("x", *x)
                            .with_f64("y", *y)
                    );
                }
                crate::gesture::Gesture::Swipe { direction, velocity, start_x, start_y, end_x, end_y } => {
                    let dir_str = match direction {
                        crate::gesture::SwipeDirection::Up => "up",
                        crate::gesture::SwipeDirection::Down => "down",
                        crate::gesture::SwipeDirection::Left => "left",
                        crate::gesture::SwipeDirection::Right => "right",
                    };
                    self.event_bus.publish(
                        crate::event_bus::BusEvent::new("gesture:swipe")
                            .with_text("direction", dir_str)
                            .with_f64("velocity", *velocity)
                            .with_f64("start_x", *start_x)
                            .with_f64("start_y", *start_y)
                            .with_f64("end_x", *end_x)
                            .with_f64("end_y", *end_y)
                    );
                }
                crate::gesture::Gesture::Pinch { scale_delta } => {
                    self.event_bus.publish(
                        crate::event_bus::BusEvent::new("gesture:pinch")
                            .with_f64("scale_delta", *scale_delta)
                    );
                }
            }
        }

        // Lifecycle: process spawns/despawns/lifetimes, tick timers, evaluate behavior rules
        crate::systems::lifecycle::run(
            &mut self.world, &mut self.spawn_queue, &self.config,
            &self.events, &mut self.global_state, &mut self.timers,
            &self.templates, &mut self.rules, FIXED_DT,
        );

        // Hierarchy system (parent→child transform propagation)
        crate::systems::hierarchy::run(&mut self.world);

        // Signal system (wire emitters → receivers)
        crate::systems::signal::run(&mut self.world);

        // State machine system (tick elapsed, check transitions)
        crate::systems::state_machine::run(&mut self.world, FIXED_DT);

        // Coroutine system (advance async behavior steps)
        crate::systems::coroutine::run(&mut self.world, FIXED_DT);

        // Drain coroutine-queued spawns into engine spawn queue
        let queued_spawns: Vec<_> = self.world.spawn_queue.spawns.drain(..).collect();
        for cmd in queued_spawns {
            self.spawn_queue.spawn(cmd);
        }

        // Environment clock (day/night, seasons)
        self.environment_clock.tick(FIXED_DT);

        // Resource flow network (transfer resources along edges)
        self.flow_network.solve(&mut self.world, FIXED_DT);

        // Sprite animation system (advance frame timers)
        crate::systems::sprite_animator::run(&mut self.world, FIXED_DT);

        // Behavior system (AI movement)
        crate::systems::behavior::run(&mut self.world, FIXED_DT);

        // Tween system (easing-curve property animation)
        crate::systems::tween::run(&mut self.world, FIXED_DT);

        // Flash system (hit flash, blink, color pulse)
        crate::systems::flash::run(&mut self.world, FIXED_DT);

        // Waypoint system (path-following movement)
        crate::systems::waypoint::run(&mut self.world, FIXED_DT);

        let mut physics_steps: u32 = 0;
        while self.accumulator >= FIXED_DT {
            self.physics_step(FIXED_DT);
            // Physics joints (distance, spring, rope, hinge) — after integrator, within physics step
            crate::systems::physics_joint::run(&mut self.world, FIXED_DT);
            self.accumulator -= FIXED_DT;
            physics_steps += 1;
        }

        // Game logic: collision reactions, scoring, damage
        crate::systems::gameplay::run(
            &mut self.world, &self.events, &mut self.spawn_queue,
            &mut self.post_fx, &mut self.particles, self.frame,
        );

        // Legacy event processor (for non-gameplay triggers like goal/pickup messages)
        crate::systems::event_processor::run(&mut self.world, &self.events);

        // Input & gameplay
        crate::systems::input_gameplay::run(&mut self.world, &self.input, &mut self.events);

        // Ghost trail system (capture position snapshots)
        crate::systems::ghost_trail::run(&mut self.world, FIXED_DT);

        // Update particles
        self.particles.update(dt);

        // Update transition
        self.transition.update(dt);

        // Update dialogue messages
        self.dialogue.tick(dt);

        // Camera follow + zoom
        self.camera.update(
            &self.world,
            self.config.bounds,
            (self.width, self.height),
            dt,
        );

        // --- RENDERING ---
        // Background + starfield
        self.framebuffer.clear(self.config.background);
        if let Some(ref starfield) = self.starfield {
            starfield.render(&mut self.framebuffer, &self.camera, self.frame);
        }

        // Entity rendering
        crate::systems::renderer::run_entities_only(
            &self.world, &mut self.framebuffer, &self.input, &self.camera,
            &self.sprite_sheets,
        );

        // Particles
        self.particles.render(&mut self.framebuffer, &self.camera);

        // Debug overlays
        if self.debug_mode {
            crate::systems::debug_render::run(&self.world, &mut self.framebuffer, &self.camera);
        }

        // Screen effects stack (tint, desaturate, flash)
        self.screen_fx.tick(dt);
        self.screen_fx.apply(&mut self.framebuffer);

        // Scene transition overlay (after all rendering, before post_fx)
        self.transition.apply(&mut self.framebuffer);

        // Post-processing effects
        crate::rendering::post_fx::apply(
            &mut self.framebuffer, &mut self.post_fx, dt, self.frame,
        );

        self.events.clear();
        self.event_bus.clear();
        self.input.end_frame();

        // Runtime diagnostics: clear previous frame and run checks
        self.diagnostic_bus.clear();
        self.diagnostic_bus.run_checks(&self.world, self.config.bounds, self.frame);

        // Update performance telemetry
        self.frame_metrics.frame_time_ms = dt * 1000.0;
        self.frame_metrics.physics_time_ms = physics_steps as f64 * FIXED_DT * 1000.0;
        self.frame_metrics.entity_count = self.world.entity_count();
        self.frame_metrics.frame_number = self.frame;

        self.time += dt;
        self.frame += 1;
    }

    pub fn physics_step(&mut self, dt: f64) {
        crate::systems::force_accumulator::run(&mut self.world);
        crate::systems::integrator::run(&mut self.world, dt);
        crate::systems::collision::run(&mut self.world, &mut self.events, dt);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_hash_deterministic() {
        let e1 = Engine::new(100, 100);
        let e2 = Engine::new(100, 100);
        assert_eq!(e1.state_hash(), e2.state_hash());
    }

    #[test]
    fn state_hash_changes_after_tick() {
        let mut engine = Engine::new(100, 100);
        let h1 = engine.state_hash();
        engine.tick(FIXED_DT);
        let h2 = engine.state_hash();
        assert_ne!(h1, h2, "hash must change after a tick (frame counter advances)");
    }

    #[test]
    fn state_hash_differs_with_different_state() {
        let mut e1 = Engine::new(100, 100);
        let e2 = Engine::new(100, 100);
        e1.global_state.set_f64("score", 10.0);
        // e2 has no score set — different state
        assert_ne!(e1.state_hash(), e2.state_hash());
    }

    #[test]
    fn state_hash_differs_with_entity_changes() {
        let mut e1 = Engine::new(100, 100);
        let mut e2 = Engine::new(100, 100);
        let ent = e1.world.spawn();
        e1.world.transforms.insert(ent, crate::components::Transform {
            x: 10.0, y: 20.0, rotation: 0.0, scale: 1.0,
        });
        assert_ne!(e1.state_hash(), e2.state_hash());

        // Add same entity+transform to e2 — hashes should match
        let ent2 = e2.world.spawn();
        e2.world.transforms.insert(ent2, crate::components::Transform {
            x: 10.0, y: 20.0, rotation: 0.0, scale: 1.0,
        });
        assert_eq!(e1.state_hash(), e2.state_hash());
    }

    #[test]
    fn state_hash_sensitive_to_rng_state() {
        let mut e1 = Engine::new(100, 100);
        let mut e2 = Engine::new(100, 100);
        e1.rng.next_u64(); // advance rng
        assert_ne!(e1.state_hash(), e2.state_hash());
    }

    // ─── Engine::reset(seed) ──────────────────────────────────────────

    #[test]
    fn reset_clears_entities() {
        let mut engine = Engine::new(100, 100);
        let ent = engine.world.spawn();
        engine.world.transforms.insert(ent, crate::components::Transform {
            x: 5.0, y: 10.0, rotation: 0.0, scale: 1.0,
        });
        assert_eq!(engine.world.entity_count(), 1);
        engine.reset(42);
        assert_eq!(engine.world.entity_count(), 0);
    }

    #[test]
    fn reset_clears_global_state() {
        let mut engine = Engine::new(100, 100);
        engine.global_state.set_f64("score", 100.0);
        engine.reset(42);
        assert!(engine.global_state.get("score").is_none());
    }

    #[test]
    fn reset_reseeds_rng() {
        let mut engine = Engine::new(100, 100);
        engine.rng.next_u64();
        engine.rng.next_u64();
        let rng_state_before = engine.rng.state;

        engine.reset(42);
        let fresh = SeededRng::new(42);
        assert_eq!(engine.rng.state, fresh.state, "rng must be reseeded to the given seed");
    }

    #[test]
    fn reset_different_seeds_produce_different_rng() {
        let mut e1 = Engine::new(100, 100);
        let mut e2 = Engine::new(100, 100);
        e1.reset(1);
        e2.reset(2);
        assert_ne!(e1.rng.state, e2.rng.state);
    }

    #[test]
    fn reset_zeroes_frame_and_time() {
        let mut engine = Engine::new(100, 100);
        engine.tick(FIXED_DT);
        engine.tick(FIXED_DT);
        assert!(engine.frame > 0);
        assert!(engine.time > 0.0);

        engine.reset(42);
        assert_eq!(engine.frame, 0);
        assert_eq!(engine.time, 0.0);
    }

    #[test]
    fn reset_produces_same_state_hash_as_fresh_engine() {
        let mut engine = Engine::new(100, 100);
        // Mutate engine state heavily
        engine.tick(FIXED_DT);
        engine.tick(FIXED_DT);
        engine.global_state.set_f64("score", 999.0);
        let ent = engine.world.spawn();
        engine.world.transforms.insert(ent, crate::components::Transform {
            x: 42.0, y: 99.0, rotation: 1.0, scale: 2.0,
        });

        engine.reset(42);
        let fresh = Engine::new(100, 100);
        assert_eq!(engine.state_hash(), fresh.state_hash(),
            "reset engine must have same state hash as a fresh engine with seed 42");
    }

    #[test]
    fn reset_clears_accumulator() {
        let mut engine = Engine::new(100, 100);
        // Feed a partial dt that won't trigger a physics step
        engine.tick(FIXED_DT * 0.5);
        engine.reset(42);
        // After reset, accumulator should be 0. Ticking with a tiny dt
        // should still advance the frame (simulation systems run once per tick).
        let hash_before = engine.state_hash();
        engine.tick(FIXED_DT);
        assert_ne!(engine.state_hash(), hash_before);
        assert_eq!(engine.frame, 1);
    }
}

