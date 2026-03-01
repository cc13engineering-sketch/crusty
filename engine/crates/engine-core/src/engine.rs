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
    pub game_over: bool,
    spawn_timer: f64,
    fire_cooldown: f64,

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
            game_over: false,
            spawn_timer: 0.0,
            fire_cooldown: 0.0,
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
        }
    }

    pub fn reset_game_state(&mut self) {
        self.game_over = false;
        self.spawn_timer = 0.0;
        self.fire_cooldown = 0.0;
        self.particles = ParticlePool::new();
        self.global_state.clear();
        self.timers.clear();
        self.rules.clear();
        self.dialogue.clear();
        self.transition = TransitionManager::new();
        self.screen_fx.clear();
        self.event_bus.clear();
    }

    pub fn tick(&mut self, dt: f64) {
        let dt = dt.min(MAX_FRAME_DT);
        self.accumulator += dt;
        self.accumulator = self.accumulator.min(5.0 * FIXED_DT);

        if self.input.keys_pressed.contains("KeyD") {
            self.debug_mode = !self.debug_mode;
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
        crate::systems::state_machine::run(&mut self.world, dt);

        // Coroutine system (advance async behavior steps)
        crate::systems::coroutine::run(&mut self.world, dt);

        // Drain coroutine-queued spawns into engine spawn queue
        let queued_spawns: Vec<_> = self.world.spawn_queue.spawns.drain(..).collect();
        for cmd in queued_spawns {
            self.spawn_queue.spawn(cmd);
        }

        // Sprite animation system (advance frame timers)
        crate::systems::sprite_animator::run(&mut self.world, dt);

        // Behavior system (AI movement)
        crate::systems::behavior::run(&mut self.world, dt);

        // Tween system (easing-curve property animation)
        crate::systems::tween::run(&mut self.world, dt);

        // Flash system (hit flash, blink, color pulse)
        crate::systems::flash::run(&mut self.world, dt);

        // Waypoint system (path-following movement)
        crate::systems::waypoint::run(&mut self.world, dt);

        while self.accumulator >= FIXED_DT {
            self.physics_step(FIXED_DT);
            // Physics joints (distance, spring, rope, hinge) — after integrator, within physics step
            crate::systems::physics_joint::run(&mut self.world, FIXED_DT);
            self.accumulator -= FIXED_DT;
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

        // Spawning system (wave spawner, fire cooldown)
        self.run_spawners(dt);

        // Ghost trail system (capture position snapshots)
        crate::systems::ghost_trail::run(&mut self.world, dt);

        // Update particles
        self.particles.update(dt);

        // Update transition
        self.transition.update(dt);

        // Update dialogue messages
        self.dialogue.tick(dt);

        // Check game over
        self.check_game_over();

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

        // HUD
        self.render_hud();

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
        self.time += dt;
        self.frame += 1;
    }

    pub fn physics_step(&mut self, dt: f64) {
        crate::systems::force_accumulator::run(&mut self.world);
        crate::systems::integrator::run(&mut self.world, dt);
        crate::systems::collision::run(&mut self.world, &mut self.events, dt);
    }

    fn run_spawners(&mut self, dt: f64) {
        if self.game_over { return; }

        // Wave spawning (check if there's a spawner entity with tags)
        let has_spawner = self.world.tags.iter().any(|(_, t)| t.has("spawner"));
        if has_spawner {
            self.spawn_timer += dt;
            let wave_interval = 4.0; // seconds between waves
            if self.spawn_timer >= wave_interval {
                self.spawn_timer -= wave_interval;
                self.spawn_wave();
            }
        }

        // Fire cooldown
        if self.fire_cooldown > 0.0 {
            self.fire_cooldown -= dt;
        }

        // Player shooting (Space key)
        if self.input.keys_held.contains("Space") && self.fire_cooldown <= 0.0 {
            self.try_fire_bullet();
        }
    }

    fn spawn_wave(&mut self) {
        use crate::rendering::particles::SimpleRng;
        let mut rng = SimpleRng::new(self.frame.wrapping_mul(7919));
        let (bw, bh) = self.config.bounds;

        // Increment wave counter on player
        for (e, tag) in self.world.tags.iter() {
            if tag.has("player") {
                if let Some(gs) = self.world.game_states.get_mut(e) {
                    gs.add("wave", 1.0);
                }
            }
        }

        let wave_num = self.world.tags.iter()
            .find(|(_, t)| t.has("player"))
            .and_then(|(e, _)| self.world.game_states.get(e))
            .map_or(1.0, |gs| gs.get("wave"));

        let asteroid_count = 3 + (wave_num as u32 / 2).min(8);
        let enemy_count = (wave_num as u32 / 3).min(4);

        for _ in 0..asteroid_count {
            let (x, y, vx, vy) = random_edge_spawn(&mut rng, bw, bh, 80.0);
            let size = 8.0 + rng.next_f64() * 16.0;
            let mut cmd = crate::spawn_queue::SpawnCommand::at(x, y);
            cmd.rigidbody = Some(crate::components::RigidBody {
                vx, vy, mass: size / 5.0,
                restitution: 0.3, damping: 0.0,
                ..Default::default()
            });
            cmd.collider = Some(crate::components::Collider {
                shape: crate::components::ColliderShape::Circle { radius: size },
                is_trigger: false,
            });
            cmd.renderable = Some(crate::components::Renderable {
                visual: crate::components::Visual::Circle {
                    radius: size, filled: true,
                    color: Color::from_rgba(
                        140 + (rng.next_f64() * 80.0) as u8,
                        120 + (rng.next_f64() * 60.0) as u8,
                        80 + (rng.next_f64() * 40.0) as u8,
                        255,
                    ),
                },
                layer: 0,
                visible: true,
            });
            cmd.tags = Some(crate::components::Tags::new(&["asteroid"]));
            cmd.game_state = Some(crate::components::GameState::from_pairs(&[
                ("health", 1.0 + (size / 15.0).floor()),
                ("damage", 15.0 + size),
                ("points", 50.0 + size * 5.0),
            ]));
            cmd.lifetime = Some(crate::components::Lifetime::new(20.0));
            self.spawn_queue.spawn(cmd);
        }

        for _ in 0..enemy_count {
            let (x, y, _, _) = random_edge_spawn(&mut rng, bw, bh, 100.0);
            let mut cmd = crate::spawn_queue::SpawnCommand::at(x, y);
            cmd.rigidbody = Some(crate::components::RigidBody {
                mass: 1.5, damping: 0.02,
                restitution: 0.2,
                ..Default::default()
            });
            cmd.collider = Some(crate::components::Collider {
                shape: crate::components::ColliderShape::Circle { radius: 8.0 },
                is_trigger: false,
            });
            cmd.renderable = Some(crate::components::Renderable {
                visual: crate::components::Visual::Circle {
                    radius: 8.0, filled: true,
                    color: Color::from_rgba(255, 60, 60, 255),
                },
                layer: 0,
                visible: true,
            });
            cmd.tags = Some(crate::components::Tags::new(&["enemy"]));
            cmd.behavior = Some(crate::components::Behavior {
                mode: crate::components::BehaviorMode::Chase,
                speed: 100.0 + wave_num * 10.0,
                turn_rate: 2.0,
                target_tag: Some("player".to_string()),
            });
            cmd.game_state = Some(crate::components::GameState::from_pairs(&[
                ("health", 2.0), ("damage", 25.0), ("points", 300.0),
            ]));
            cmd.lifetime = Some(crate::components::Lifetime::new(30.0));
            self.spawn_queue.spawn(cmd);
        }
    }

    fn try_fire_bullet(&mut self) {
        // Find player position and aim direction
        let player_data: Option<(f64, f64)> = self.world.tags.iter()
            .find(|(_, t)| t.has("player"))
            .and_then(|(e, _)| self.world.transforms.get(e).map(|t| (t.x, t.y)));

        let Some((px, py)) = player_data else { return };

        // Check ammo
        let has_ammo = self.world.tags.iter()
            .find(|(_, t)| t.has("player"))
            .and_then(|(e, _)| self.world.game_states.get(e))
            .map_or(true, |gs| gs.get("ammo") > 0.0);
        if !has_ammo { return; }

        // Aim toward mouse
        let dx = self.input.mouse_x - px;
        let dy = self.input.mouse_y - py;
        let len = (dx * dx + dy * dy).sqrt();
        let (ndx, ndy) = if len > 0.0 { (dx / len, dy / len) } else { (0.0, -1.0) };

        let bullet_speed = 500.0;
        let mut cmd = crate::spawn_queue::SpawnCommand::at(
            px + ndx * 15.0, py + ndy * 15.0,
        );
        cmd.rigidbody = Some(crate::components::RigidBody {
            vx: ndx * bullet_speed, vy: ndy * bullet_speed,
            mass: 0.1, damping: 0.0, restitution: 0.0,
            ..Default::default()
        });
        cmd.collider = Some(crate::components::Collider {
            shape: crate::components::ColliderShape::Circle { radius: 3.0 },
            is_trigger: false,
        });
        cmd.renderable = Some(crate::components::Renderable {
            visual: crate::components::Visual::Circle {
                radius: 3.0, filled: true,
                color: Color::from_rgba(200, 255, 255, 255),
            },
            layer: 1,
            visible: true,
        });
        cmd.tags = Some(crate::components::Tags::new(&["bullet"]));
        cmd.game_state = Some(crate::components::GameState::from_pairs(&[("damage", 1.0)]));
        cmd.lifetime = Some(crate::components::Lifetime::new(2.0));
        self.spawn_queue.spawn(cmd);

        // Deduct ammo
        for (e, tag) in self.world.tags.iter() {
            if tag.has("player") {
                if let Some(gs) = self.world.game_states.get_mut(e) {
                    gs.add("ammo", -1.0);
                }
            }
        }

        self.fire_cooldown = 0.15;

        // Muzzle flash particles
        self.particles.spawn_burst(
            px + ndx * 15.0, py + ndy * 15.0,
            5, 30.0, 80.0,
            0.15, 2.0, 0.5,
            Color::from_rgba(200, 255, 255, 255),
            Color::from_rgba(100, 200, 255, 0),
            self.frame,
        );
    }

    fn check_game_over(&mut self) {
        if self.game_over { return; }
        for (e, tag) in self.world.tags.iter() {
            if tag.has("player") {
                if let Some(gs) = self.world.game_states.get(e) {
                    if gs.get("health") <= 0.0 {
                        self.game_over = true;
                        self.post_fx.shake_remaining = 0.3;
                        self.post_fx.shake_intensity = 10.0;
                    }
                }
            }
        }
    }

    fn render_hud(&mut self) {
        use crate::rendering::text;
        use crate::dialogue::MessageKind;

        // Find player state
        let player_state = self.world.tags.iter()
            .find(|(_, t)| t.has("player"))
            .and_then(|(e, _)| self.world.game_states.get(e).cloned());

        if let Some(gs) = player_state {
            let score = gs.get("score") as i64;
            let health = gs.get("health") as i64;
            let shield = gs.get("shield") as i64;
            let ammo = gs.get("ammo") as i64;
            let wave = gs.get("wave") as i64;

            // Score (top-left, large)
            text::draw_text(
                &mut self.framebuffer, 10, 10,
                &format!("SCORE {}", score),
                Color::WHITE, 2,
            );

            // Wave
            text::draw_text(
                &mut self.framebuffer, 10, 30,
                &format!("WAVE {}", wave),
                Color::from_rgba(150, 150, 255, 255), 1,
            );

            // Health bar (bottom-left)
            let bar_y = self.height as i32 - 24;
            text::draw_text(
                &mut self.framebuffer, 10, bar_y,
                "HP", Color::from_rgba(255, 80, 80, 255), 1,
            );
            let bar_x = 26;
            let bar_w = 100;
            let bar_h = 6;
            let fill_w = ((health.max(0) as f64 / 100.0) * bar_w as f64) as i32;
            crate::rendering::shapes::fill_rect(
                &mut self.framebuffer,
                bar_x as f64, bar_y as f64,
                bar_w as f64, bar_h as f64,
                Color::from_rgba(40, 40, 40, 200),
            );
            if fill_w > 0 {
                let hp_color = if health > 50 {
                    Color::from_rgba(50, 255, 80, 255)
                } else if health > 25 {
                    Color::from_rgba(255, 200, 50, 255)
                } else {
                    Color::from_rgba(255, 60, 60, 255)
                };
                crate::rendering::shapes::fill_rect(
                    &mut self.framebuffer,
                    bar_x as f64, bar_y as f64,
                    fill_w as f64, bar_h as f64,
                    hp_color,
                );
            }

            // Shield bar
            if shield > 0 {
                let shield_x = bar_x + bar_w + 10;
                text::draw_text(
                    &mut self.framebuffer, shield_x, bar_y,
                    "SH", Color::from_rgba(80, 150, 255, 255), 1,
                );
                let s_bar_x = shield_x + 18;
                let s_fill = ((shield.max(0) as f64 / 50.0) * 60.0) as i32;
                crate::rendering::shapes::fill_rect(
                    &mut self.framebuffer,
                    s_bar_x as f64, bar_y as f64,
                    60.0, bar_h as f64,
                    Color::from_rgba(40, 40, 40, 200),
                );
                crate::rendering::shapes::fill_rect(
                    &mut self.framebuffer,
                    s_bar_x as f64, bar_y as f64,
                    s_fill as f64, bar_h as f64,
                    Color::from_rgba(80, 150, 255, 255),
                );
            }

            // Ammo (top-right)
            let ammo_text = format!("AMMO {}", ammo);
            let tw = text::text_width(&ammo_text, 1);
            text::draw_text(
                &mut self.framebuffer,
                self.width as i32 - tw - 10, 10,
                &ammo_text,
                Color::from_rgba(255, 200, 50, 255), 1,
            );

            // Entity count (debug)
            if self.debug_mode {
                text::draw_text(
                    &mut self.framebuffer, 10, 48,
                    &format!("ENT {} PRT {}", self.world.entity_count(), self.particles.count()),
                    Color::from_rgba(100, 100, 100, 200), 1,
                );
            }
        }

        // Dialogue / notification / floating text messages
        {
            // Collect messages to avoid borrow conflict with self.framebuffer
            let msgs: Vec<_> = self.dialogue.active().cloned().collect();
            let mut notification_y = 10i32;
            let screen_w = self.width as i32;
            let screen_h = self.height as i32;

            for msg in &msgs {
                // Fade alpha based on remaining time (fade out in last 0.5s)
                let alpha_factor = if msg.remaining < 0.5 {
                    (msg.remaining / 0.5).max(0.0).min(1.0)
                } else {
                    1.0
                };
                let msg_color = Color::from_rgba(
                    msg.color.r,
                    msg.color.g,
                    msg.color.b,
                    (msg.color.a as f64 * alpha_factor) as u8,
                );

                match msg.kind {
                    MessageKind::Dialogue => {
                        // Draw text box at bottom of screen
                        let box_h = 40i32;
                        let box_y = screen_h - box_h - 4;
                        let box_w = screen_w - 20;
                        crate::rendering::shapes::fill_rect(
                            &mut self.framebuffer,
                            10.0, box_y as f64,
                            box_w as f64, box_h as f64,
                            Color::from_rgba(0, 0, 0, 180),
                        );
                        crate::rendering::shapes::draw_rect(
                            &mut self.framebuffer,
                            10.0, box_y as f64,
                            box_w as f64, box_h as f64,
                            Color::from_rgba(200, 200, 200, 150),
                        );
                        text::draw_text(
                            &mut self.framebuffer,
                            18, box_y + 14,
                            &msg.text,
                            msg_color, 1,
                        );
                    }
                    MessageKind::Notification => {
                        // Draw toast at top-center
                        let tw = text::text_width(&msg.text, 1);
                        let tx = screen_w / 2 - tw / 2;
                        // Background pill
                        crate::rendering::shapes::fill_rect(
                            &mut self.framebuffer,
                            (tx - 6) as f64, (notification_y - 2) as f64,
                            (tw + 12) as f64, 12.0,
                            Color::from_rgba(0, 0, 0, 160),
                        );
                        text::draw_text(
                            &mut self.framebuffer,
                            tx, notification_y,
                            &msg.text,
                            msg_color, 1,
                        );
                        notification_y += 16;
                    }
                    MessageKind::FloatingText => {
                        // Draw at world_pos using camera transform
                        if let Some((wx, wy)) = msg.world_pos {
                            let (sx, sy) = self.camera.world_to_screen(wx, wy);
                            text::draw_text_centered(
                                &mut self.framebuffer,
                                sx, sy,
                                &msg.text,
                                msg_color, 1,
                            );
                        }
                    }
                }
            }
        }

        // Game over overlay
        if self.game_over {
            text::draw_text_centered(
                &mut self.framebuffer,
                self.width as i32 / 2, self.height as i32 / 2 - 20,
                "GAME OVER",
                Color::from_rgba(255, 60, 60, 255), 4,
            );

            if let Some(gs) = self.world.tags.iter()
                .find(|(_, t)| t.has("player"))
                .and_then(|(e, _)| self.world.game_states.get(e))
            {
                let final_score = format!("SCORE: {}", gs.get("score") as i64);
                text::draw_text_centered(
                    &mut self.framebuffer,
                    self.width as i32 / 2, self.height as i32 / 2 + 25,
                    &final_score,
                    Color::WHITE, 2,
                );
            }

            // Blinking restart prompt
            if (self.frame / 30) % 2 == 0 {
                text::draw_text_centered(
                    &mut self.framebuffer,
                    self.width as i32 / 2, self.height as i32 / 2 + 55,
                    "CLICK TO RESTART",
                    Color::from_rgba(180, 180, 180, 200), 1,
                );
            }
        }
    }
}

fn random_edge_spawn(
    rng: &mut crate::rendering::particles::SimpleRng,
    bw: f64, bh: f64,
    speed: f64,
) -> (f64, f64, f64, f64) {
    let side = (rng.next_f64() * 4.0) as u32;
    let margin = 30.0;
    match side {
        0 => { // top
            let x = rng.next_f64() * bw;
            let vx = (rng.next_f64() - 0.5) * speed;
            let vy = speed * (0.3 + rng.next_f64() * 0.7);
            (x, -margin, vx, vy)
        }
        1 => { // bottom
            let x = rng.next_f64() * bw;
            let vx = (rng.next_f64() - 0.5) * speed;
            let vy = -speed * (0.3 + rng.next_f64() * 0.7);
            (x, bh + margin, vx, vy)
        }
        2 => { // left
            let y = rng.next_f64() * bh;
            let vx = speed * (0.3 + rng.next_f64() * 0.7);
            let vy = (rng.next_f64() - 0.5) * speed;
            (-margin, y, vx, vy)
        }
        _ => { // right
            let y = rng.next_f64() * bh;
            let vx = -speed * (0.3 + rng.next_f64() * 0.7);
            let vy = (rng.next_f64() - 0.5) * speed;
            (bw + margin, y, vx, vy)
        }
    }
}
