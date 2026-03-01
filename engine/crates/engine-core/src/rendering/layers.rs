/// Render layer system with parallax scrolling support.
/// Each layer has an ID (draw order), a name, a scroll factor for parallax,
/// and a visibility flag.

#[derive(Clone, Debug)]
pub struct RenderLayer {
    pub id: i32,
    pub name: String,
    pub scroll_factor: f64,  // 0.0 = fixed (HUD), 0.5 = half-speed parallax, 1.0 = normal
    pub visible: bool,
}

#[derive(Debug, Default)]
pub struct RenderLayerStack {
    layers: Vec<RenderLayer>,
}

impl RenderLayerStack {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a layer. If a layer with the same ID already exists, it is replaced.
    pub fn add(&mut self, layer: RenderLayer) {
        // Replace existing layer with same ID
        if let Some(existing) = self.layers.iter_mut().find(|l| l.id == layer.id) {
            *existing = layer;
        } else {
            self.layers.push(layer);
        }
    }

    /// Get a reference to a layer by its ID.
    pub fn get(&self, id: i32) -> Option<&RenderLayer> {
        self.layers.iter().find(|l| l.id == id)
    }

    /// Return layer IDs sorted in ascending order (back-to-front draw order).
    pub fn sorted_ids(&self) -> Vec<i32> {
        let mut ids: Vec<i32> = self.layers.iter().map(|l| l.id).collect();
        ids.sort();
        ids
    }

    /// Get the scroll factor for a layer. Returns 1.0 if the layer is not found.
    pub fn scroll_factor(&self, id: i32) -> f64 {
        self.layers
            .iter()
            .find(|l| l.id == id)
            .map_or(1.0, |l| l.scroll_factor)
    }
}
