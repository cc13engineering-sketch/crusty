use crate::rendering::color::Color;
use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Renderable {
    pub visual: Visual,
    pub layer: i32,
    pub visible: bool,
}

#[derive(Clone, Debug)]
pub enum Visual {
    Circle { radius: f64, color: Color, filled: bool },
    Rect { width: f64, height: f64, color: Color, filled: bool },
    Line { x2: f64, y2: f64, color: Color, thickness: f64 },
    Sprite { sheet_id: u32, tile_index: u32 },
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            visual: Visual::Circle { radius: 10.0, color: Color::WHITE, filled: true },
            layer: 0,
            visible: true,
        }
    }
}

impl SchemaInfo for Renderable {
    fn schema_name() -> &'static str { "Renderable" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "visual": { "type": "enum", "variants": ["Circle", "Rect", "Line", "Sprite"] },
                "layer": { "type": "i32", "default": 0 },
                "visible": { "type": "bool", "default": true }
            }
        })
    }
}
