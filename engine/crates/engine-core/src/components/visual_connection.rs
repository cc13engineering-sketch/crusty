/// COMPONENT: VisualConnection
/// Renders a visual link between two entities (lines, dashes, flow indicators).

use crate::ecs::Entity;
use crate::rendering::color::Color;

/// Style of the connection line.
#[derive(Clone, Debug)]
pub enum ConnectionStyle {
    Line { thickness: f64 },
    Dashed { thickness: f64, dash_len: f64, gap_len: f64 },
    Catenary { thickness: f64, sag: f64 },
    FlowLine { thickness: f64, dot_spacing: f64, dot_speed: f64 },
}

/// A visual connection drawn between this entity and a target entity.
#[derive(Clone, Debug)]
pub struct VisualConnection {
    pub target: Entity,
    pub style: ConnectionStyle,
    pub color: Color,
    pub color_end: Option<Color>,
    pub layer: i32,
    pub visible: bool,
    pub phase: f64,
    pub flow_intensity: f64,
}

impl VisualConnection {
    pub fn line(target: Entity, thickness: f64, color: Color) -> Self {
        Self {
            target,
            style: ConnectionStyle::Line { thickness },
            color,
            color_end: None,
            layer: 0,
            visible: true,
            phase: 0.0,
            flow_intensity: 1.0,
        }
    }

    pub fn dashed(target: Entity, thickness: f64, dash_len: f64, gap_len: f64, color: Color) -> Self {
        Self {
            target,
            style: ConnectionStyle::Dashed { thickness, dash_len, gap_len },
            color,
            color_end: None,
            layer: 0,
            visible: true,
            phase: 0.0,
            flow_intensity: 1.0,
        }
    }

    pub fn flow_line(target: Entity, color: Color) -> Self {
        Self {
            target,
            style: ConnectionStyle::FlowLine {
                thickness: 2.0, dot_spacing: 12.0, dot_speed: 60.0,
            },
            color,
            color_end: None,
            layer: 0,
            visible: true,
            phase: 0.0,
            flow_intensity: 1.0,
        }
    }

    pub fn with_end_color(mut self, color: Color) -> Self {
        self.color_end = Some(color);
        self
    }

    pub fn with_layer(mut self, layer: i32) -> Self {
        self.layer = layer;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_connection() {
        let conn = VisualConnection::line(Entity(5), 2.0, Color::RED);
        assert_eq!(conn.target, Entity(5));
        assert!(conn.visible);
        assert_eq!(conn.flow_intensity, 1.0);
    }

    #[test]
    fn flow_line_defaults() {
        let conn = VisualConnection::flow_line(Entity(3), Color::GREEN);
        match conn.style {
            ConnectionStyle::FlowLine { dot_spacing, dot_speed, .. } => {
                assert_eq!(dot_spacing, 12.0);
                assert_eq!(dot_speed, 60.0);
            }
            _ => panic!("Expected FlowLine"),
        }
    }

    #[test]
    fn dashed_connection() {
        let conn = VisualConnection::dashed(Entity(1), 1.0, 5.0, 3.0, Color::WHITE);
        match conn.style {
            ConnectionStyle::Dashed { dash_len, gap_len, .. } => {
                assert_eq!(dash_len, 5.0);
                assert_eq!(gap_len, 3.0);
            }
            _ => panic!("Expected Dashed"),
        }
    }

    #[test]
    fn builder_methods() {
        let conn = VisualConnection::line(Entity(1), 1.0, Color::RED)
            .with_end_color(Color::BLUE)
            .with_layer(5);
        assert!(conn.color_end.is_some());
        assert_eq!(conn.layer, 5);
    }
}
