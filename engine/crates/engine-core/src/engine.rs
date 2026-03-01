use crate::ecs::World;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::color::Color;
use crate::rendering::particles::ParticlePool;
use crate::rendering::starfield::Starfield;
use crate::rendering::post_fx::PostFxConfig;
use crate::input::Input;
use crate::events::EventQueue;
use crate::spawn_queue::SpawnQueue;
use crate::game_state::GameState as GlobalGameState;
use crate::timers::TimerQueue;
use crate::templates::TemplateRegistry;
use crate::behavior::BehaviorRules;

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
}

impl Default for Camera {
    fn default() -> Self { Self { x: 0.0, y: 0.0 } }
}

impl Camera {
    pub fn world_to_screen(&self, wx: f64, wy: f64) -> (i32, i32) {
        ((wx - self.x).round() as i32, (wy - self.y).round() as i32)
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

        // Behavior system (AI movement)
        crate::systems::behavior::run(&mut self.world, dt);

        while self.accumulator >= FIXED_DT {
            self.physics_step(FIXED_DT);
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

        // Update particles
        self.particles.update(dt);

        // Check game over
        self.check_game_over();

        // --- RENDERING ---
        // Background + starfield
        self.framebuffer.clear(self.config.background);
        if let Some(ref starfield) = self.starfield {
            starfield.render(&mut self.framebuffer, &self.camera, self.frame);
        }

        // Entity rendering
        crate::systems::renderer::run_entities_only(
            &self.world, &mut self.framebuffer, &self.input, &self.camera,
        );

        // Particles
        self.particles.render(&mut self.framebuffer, &self.camera);

        // Debug overlays
        if self.debug_mode {
            crate::systems::debug_render::run(&self.world, &mut self.framebuffer, &self.camera);
        }

        // HUD
        self.render_hud();

        // Post-processing effects
        crate::rendering::post_fx::apply(
            &mut self.framebuffer, &mut self.post_fx, dt, self.frame,
        );

        self.events.clear();
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
