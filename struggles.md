# Gravity Pong Development Struggles

Each line is a struggle encountered during development. Append only.

- Deciding on resolution/aspect ratio: 640x640 square vs widescreen. Went with 640x640 for clean 1:1 world mapping.
- Coordinate system complexity: 0-1000 world space -> screen space scaling. Uniform scale factor = screen_size / 1000.
- Engine's Simulation::render() takes &self not &mut self - all state mutations must happen in step(), render is read-only.
- Engine's tick() clears framebuffer and renders Renderables before we can draw. Our render() runs after tick(), drawing on top of cleared buffer.
- The SIM thread_local in lib.rs is typed to DemoBall specifically - need to change to Box<dyn Simulation> trait object.
- Engine's built-in physics (force_accumulator, integrator, collision) are too simple for Plummer-softened gravity. All physics must be custom in step().
