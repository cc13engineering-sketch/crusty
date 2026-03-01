use super::SchemaInfo;

#[derive(Clone, Debug)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
}

impl Waypoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WaypointMode {
    /// Stop at the last waypoint.
    Once,
    /// Loop back to the start after reaching the end.
    Loop,
    /// Reverse direction at endpoints (ping-pong).
    PingPong,
}

#[derive(Clone, Debug)]
pub struct WaypointPath {
    pub waypoints: Vec<Waypoint>,
    pub speed: f64,
    pub current_index: usize,
    pub mode: WaypointMode,
    pub forward: bool,
    pub pause_at_waypoint: f64,
    pub pause_timer: f64,
}

impl WaypointPath {
    pub fn new(waypoints: Vec<Waypoint>, speed: f64, mode: WaypointMode) -> Self {
        Self {
            waypoints,
            speed,
            current_index: 0,
            mode,
            forward: true,
            pause_at_waypoint: 0.0,
            pause_timer: 0.0,
        }
    }
}

impl Default for WaypointPath {
    fn default() -> Self {
        Self {
            waypoints: Vec::new(),
            speed: 100.0,
            current_index: 0,
            mode: WaypointMode::Once,
            forward: true,
            pause_at_waypoint: 0.0,
            pause_timer: 0.0,
        }
    }
}

impl SchemaInfo for WaypointPath {
    fn schema_name() -> &'static str { "WaypointPath" }
    fn schema() -> serde_json::Value {
        serde_json::json!({
            "fields": {
                "waypoints": { "type": "Vec<Waypoint>", "note": "Ordered list of {x, y} positions" },
                "speed": { "type": "f64", "default": 100.0, "note": "Movement speed in units/sec" },
                "current_index": { "type": "usize", "default": 0 },
                "mode": { "type": "enum", "variants": ["Once", "Loop", "PingPong"] },
                "forward": { "type": "bool", "default": true, "note": "Direction for PingPong mode" },
                "pause_at_waypoint": { "type": "f64", "default": 0.0, "note": "Seconds to pause at each waypoint" },
                "pause_timer": { "type": "f64", "default": 0.0 }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint_new() {
        let wp = Waypoint::new(10.0, 20.0);
        assert_eq!(wp.x, 10.0);
        assert_eq!(wp.y, 20.0);
    }

    #[test]
    fn test_waypoint_path_new() {
        let path = WaypointPath::new(
            vec![Waypoint::new(0.0, 0.0), Waypoint::new(100.0, 0.0)],
            50.0,
            WaypointMode::Loop,
        );
        assert_eq!(path.waypoints.len(), 2);
        assert_eq!(path.speed, 50.0);
        assert_eq!(path.current_index, 0);
        assert_eq!(path.mode, WaypointMode::Loop);
        assert!(path.forward);
        assert_eq!(path.pause_at_waypoint, 0.0);
        assert_eq!(path.pause_timer, 0.0);
    }

    #[test]
    fn test_waypoint_path_default() {
        let path = WaypointPath::default();
        assert!(path.waypoints.is_empty());
        assert_eq!(path.speed, 100.0);
        assert_eq!(path.current_index, 0);
        assert_eq!(path.mode, WaypointMode::Once);
        assert!(path.forward);
    }

    #[test]
    fn test_waypoint_mode_clone() {
        let mode = WaypointMode::PingPong;
        let cloned = mode.clone();
        assert_eq!(cloned, WaypointMode::PingPong);
    }

    #[test]
    fn test_waypoint_path_clone() {
        let path = WaypointPath::new(
            vec![Waypoint::new(1.0, 2.0), Waypoint::new(3.0, 4.0)],
            75.0,
            WaypointMode::Once,
        );
        let cloned = path.clone();
        assert_eq!(cloned.waypoints.len(), 2);
        assert_eq!(cloned.speed, 75.0);
        assert_eq!(cloned.waypoints[0].x, 1.0);
        assert_eq!(cloned.waypoints[1].y, 4.0);
    }

    #[test]
    fn test_waypoint_debug() {
        let wp = Waypoint::new(1.5, 2.5);
        let debug_str = format!("{:?}", wp);
        assert!(debug_str.contains("1.5"));
        assert!(debug_str.contains("2.5"));
    }

    #[test]
    fn test_waypoint_path_debug() {
        let path = WaypointPath::new(
            vec![Waypoint::new(0.0, 0.0)],
            100.0,
            WaypointMode::PingPong,
        );
        let debug_str = format!("{:?}", path);
        assert!(debug_str.contains("PingPong"));
        assert!(debug_str.contains("100.0"));
    }

    #[test]
    fn test_schema_name() {
        assert_eq!(WaypointPath::schema_name(), "WaypointPath");
    }

    #[test]
    fn test_schema_has_fields() {
        let schema = WaypointPath::schema();
        assert!(schema.get("fields").is_some());
        let fields = schema.get("fields").expect("should have fields");
        assert!(fields.get("waypoints").is_some());
        assert!(fields.get("speed").is_some());
        assert!(fields.get("mode").is_some());
        assert!(fields.get("pause_at_waypoint").is_some());
    }

    #[test]
    fn test_waypoint_path_with_pause() {
        let mut path = WaypointPath::new(
            vec![Waypoint::new(0.0, 0.0), Waypoint::new(10.0, 0.0)],
            50.0,
            WaypointMode::Once,
        );
        path.pause_at_waypoint = 2.0;
        assert_eq!(path.pause_at_waypoint, 2.0);
        assert_eq!(path.pause_timer, 0.0);
    }

    #[test]
    fn test_waypoint_path_mode_variants() {
        assert_eq!(WaypointMode::Once, WaypointMode::Once);
        assert_eq!(WaypointMode::Loop, WaypointMode::Loop);
        assert_eq!(WaypointMode::PingPong, WaypointMode::PingPong);
        assert_ne!(WaypointMode::Once, WaypointMode::Loop);
        assert_ne!(WaypointMode::Loop, WaypointMode::PingPong);
    }

    #[test]
    fn test_waypoint_path_forward_flag() {
        let mut path = WaypointPath::new(
            vec![Waypoint::new(0.0, 0.0)],
            100.0,
            WaypointMode::PingPong,
        );
        assert!(path.forward);
        path.forward = false;
        assert!(!path.forward);
    }
}
