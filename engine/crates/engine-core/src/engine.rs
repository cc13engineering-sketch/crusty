use crate::ecs::World;
use crate::rendering::framebuffer::Framebuffer;
use crate::rendering::color::Color;
use crate::input::Input;
use crate::events::EventQueue;

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
        }
    }

    pub fn tick(&mut self, dt: f64) {
        let dt = dt.min(MAX_FRAME_DT);
        self.accumulator += dt;
        self.accumulator = self.accumulator.min(5.0 * FIXED_DT);

        if self.input.keys_pressed.contains("KeyD") {
            self.debug_mode = !self.debug_mode;
        }

        while self.accumulator >= FIXED_DT {
            self.physics_step(FIXED_DT);
            self.accumulator -= FIXED_DT;
        }

        crate::systems::event_processor::run(&mut self.world, &self.events);
        crate::systems::input_gameplay::run(&mut self.world, &self.input, &mut self.events);

        crate::systems::renderer::run(&self.world, &mut self.framebuffer, &self.config, &self.input, &self.camera);
        if self.debug_mode {
            crate::systems::debug_render::run(&self.world, &mut self.framebuffer, &self.camera);
        }

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
}
