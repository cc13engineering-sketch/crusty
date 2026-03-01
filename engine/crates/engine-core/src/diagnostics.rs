/// DiagnosticBus: Structured runtime error reporting.
///
/// Collects diagnostic messages during engine operation and provides
/// runtime checks for common issues like NaN transforms, excessive entity
/// counts, and out-of-bounds entities.

use crate::ecs::World;

/// Severity level of a diagnostic message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl DiagnosticLevel {
    fn as_str(&self) -> &'static str {
        match self {
            DiagnosticLevel::Info => "info",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Critical => "critical",
        }
    }
}

/// Category of diagnostic issue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticCategory {
    Physics,
    Reference,
    Performance,
    Gameplay,
    Lifecycle,
}

impl DiagnosticCategory {
    fn as_str(&self) -> &'static str {
        match self {
            DiagnosticCategory::Physics => "physics",
            DiagnosticCategory::Reference => "reference",
            DiagnosticCategory::Performance => "performance",
            DiagnosticCategory::Gameplay => "gameplay",
            DiagnosticCategory::Lifecycle => "lifecycle",
        }
    }
}

/// A single diagnostic message.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub category: DiagnosticCategory,
    pub message: String,
    pub frame: u64,
    pub entity_id: Option<u64>,
}

/// Collects and manages diagnostic messages for the engine.
#[derive(Clone, Debug)]
pub struct DiagnosticBus {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBus {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Report a new diagnostic.
    pub fn report(
        &mut self,
        level: DiagnosticLevel,
        category: DiagnosticCategory,
        message: String,
        frame: u64,
        entity_id: Option<u64>,
    ) {
        self.diagnostics.push(Diagnostic {
            level,
            category,
            message,
            frame,
            entity_id,
        });
    }

    /// Run automatic runtime checks against the world state.
    /// Checks for NaN transforms, entity count thresholds, and out-of-bounds entities.
    pub fn run_checks(
        &mut self,
        world: &World,
        config_bounds: (f64, f64),
        frame: u64,
    ) {
        let (bw, bh) = config_bounds;

        // Check for NaN transforms
        for (entity, transform) in world.transforms.iter() {
            if transform.x.is_nan() || transform.y.is_nan() {
                self.report(
                    DiagnosticLevel::Error,
                    DiagnosticCategory::Physics,
                    format!("Entity {} has NaN transform (x={}, y={})", entity.0, transform.x, transform.y),
                    frame,
                    Some(entity.0),
                );
            }

            // Check for entities far out of bounds (margin of 200 pixels beyond bounds)
            let margin = 200.0;
            if transform.x < -margin || transform.x > bw + margin
                || transform.y < -margin || transform.y > bh + margin
            {
                self.report(
                    DiagnosticLevel::Warning,
                    DiagnosticCategory::Physics,
                    format!(
                        "Entity {} is out of bounds at ({:.1}, {:.1}), world bounds are ({:.0}, {:.0})",
                        entity.0, transform.x, transform.y, bw, bh
                    ),
                    frame,
                    Some(entity.0),
                );
            }
        }

        // Check for excessive entity count
        let entity_count = world.entity_count();
        if entity_count > 500 {
            self.report(
                DiagnosticLevel::Warning,
                DiagnosticCategory::Performance,
                format!("High entity count: {} (threshold: 500)", entity_count),
                frame,
                None,
            );
        }
    }

    /// Serialize all diagnostics to a JSON string.
    pub fn to_json(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for d in &self.diagnostics {
            let entity_str = match d.entity_id {
                Some(id) => format!("{}", id),
                None => "null".to_string(),
            };
            // Escape the message for JSON safety
            let escaped_msg = d.message.replace('\\', "\\\\").replace('"', "\\\"");
            parts.push(format!(
                "{{\"level\":\"{}\",\"category\":\"{}\",\"message\":\"{}\",\"frame\":{},\"entity_id\":{}}}",
                d.level.as_str(),
                d.category.as_str(),
                escaped_msg,
                d.frame,
                entity_str,
            ));
        }
        format!("[{}]", parts.join(","))
    }

    /// Returns true if any diagnostic is Error or Critical level.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| {
            matches!(d.level, DiagnosticLevel::Error | DiagnosticLevel::Critical)
        })
    }

    /// Returns a human-readable summary of all diagnostics.
    pub fn summary(&self) -> String {
        let mut info_count = 0u32;
        let mut warn_count = 0u32;
        let mut error_count = 0u32;
        let mut critical_count = 0u32;

        for d in &self.diagnostics {
            match d.level {
                DiagnosticLevel::Info => info_count += 1,
                DiagnosticLevel::Warning => warn_count += 1,
                DiagnosticLevel::Error => error_count += 1,
                DiagnosticLevel::Critical => critical_count += 1,
            }
        }

        format!(
            "Diagnostics: {} total ({} info, {} warnings, {} errors, {} critical)",
            self.diagnostics.len(),
            info_count,
            warn_count,
            error_count,
            critical_count,
        )
    }

    /// Clear all diagnostics.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    /// Return the number of diagnostics currently stored.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Return true if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Return a slice of all diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

impl Default for DiagnosticBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Transform;

    #[test]
    fn new_bus_is_empty() {
        let bus = DiagnosticBus::new();
        assert!(bus.is_empty());
        assert_eq!(bus.len(), 0);
        assert!(!bus.has_errors());
    }

    #[test]
    fn report_adds_diagnostic() {
        let mut bus = DiagnosticBus::new();
        bus.report(
            DiagnosticLevel::Warning,
            DiagnosticCategory::Physics,
            "Test warning".to_string(),
            10,
            Some(42),
        );
        assert_eq!(bus.len(), 1);
        let d = &bus.diagnostics()[0];
        assert_eq!(d.level, DiagnosticLevel::Warning);
        assert_eq!(d.category, DiagnosticCategory::Physics);
        assert_eq!(d.message, "Test warning");
        assert_eq!(d.frame, 10);
        assert_eq!(d.entity_id, Some(42));
    }

    #[test]
    fn has_errors_detects_error_level() {
        let mut bus = DiagnosticBus::new();
        bus.report(
            DiagnosticLevel::Info,
            DiagnosticCategory::Gameplay,
            "info".to_string(),
            0,
            None,
        );
        assert!(!bus.has_errors());

        bus.report(
            DiagnosticLevel::Error,
            DiagnosticCategory::Physics,
            "error".to_string(),
            0,
            None,
        );
        assert!(bus.has_errors());
    }

    #[test]
    fn has_errors_detects_critical_level() {
        let mut bus = DiagnosticBus::new();
        bus.report(
            DiagnosticLevel::Critical,
            DiagnosticCategory::Lifecycle,
            "critical".to_string(),
            0,
            None,
        );
        assert!(bus.has_errors());
    }

    #[test]
    fn clear_removes_all_diagnostics() {
        let mut bus = DiagnosticBus::new();
        bus.report(
            DiagnosticLevel::Info,
            DiagnosticCategory::Gameplay,
            "a".to_string(),
            0,
            None,
        );
        bus.report(
            DiagnosticLevel::Error,
            DiagnosticCategory::Physics,
            "b".to_string(),
            1,
            Some(1),
        );
        assert_eq!(bus.len(), 2);
        bus.clear();
        assert!(bus.is_empty());
        assert!(!bus.has_errors());
    }

    #[test]
    fn summary_counts_levels() {
        let mut bus = DiagnosticBus::new();
        bus.report(DiagnosticLevel::Info, DiagnosticCategory::Gameplay, "i".to_string(), 0, None);
        bus.report(DiagnosticLevel::Warning, DiagnosticCategory::Performance, "w".to_string(), 0, None);
        bus.report(DiagnosticLevel::Error, DiagnosticCategory::Physics, "e".to_string(), 0, None);
        bus.report(DiagnosticLevel::Critical, DiagnosticCategory::Lifecycle, "c".to_string(), 0, None);

        let s = bus.summary();
        assert!(s.contains("4 total"));
        assert!(s.contains("1 info"));
        assert!(s.contains("1 warnings"));
        assert!(s.contains("1 errors"));
        assert!(s.contains("1 critical"));
    }

    #[test]
    fn to_json_produces_valid_json_array() {
        let mut bus = DiagnosticBus::new();
        bus.report(
            DiagnosticLevel::Warning,
            DiagnosticCategory::Physics,
            "NaN detected".to_string(),
            5,
            Some(99),
        );
        let json = bus.to_json();
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("\"level\":\"warning\""));
        assert!(json.contains("\"category\":\"physics\""));
        assert!(json.contains("\"message\":\"NaN detected\""));
        assert!(json.contains("\"frame\":5"));
        assert!(json.contains("\"entity_id\":99"));
    }

    #[test]
    fn to_json_empty_bus_returns_empty_array() {
        let bus = DiagnosticBus::new();
        assert_eq!(bus.to_json(), "[]");
    }

    #[test]
    fn to_json_null_entity_id() {
        let mut bus = DiagnosticBus::new();
        bus.report(
            DiagnosticLevel::Info,
            DiagnosticCategory::Performance,
            "test".to_string(),
            0,
            None,
        );
        let json = bus.to_json();
        assert!(json.contains("\"entity_id\":null"));
    }

    #[test]
    fn run_checks_detects_nan_transform() {
        let mut bus = DiagnosticBus::new();
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform {
            x: f64::NAN,
            y: 10.0,
            rotation: 0.0,
            scale: 1.0,
        });
        bus.run_checks(&world, (960.0, 540.0), 42);
        assert!(bus.has_errors());
        assert_eq!(bus.len(), 1);
        let d = &bus.diagnostics()[0];
        assert_eq!(d.level, DiagnosticLevel::Error);
        assert_eq!(d.category, DiagnosticCategory::Physics);
        assert_eq!(d.entity_id, Some(e.0));
        assert!(d.message.contains("NaN"));
    }

    #[test]
    fn run_checks_detects_out_of_bounds_entity() {
        let mut bus = DiagnosticBus::new();
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform {
            x: 2000.0,
            y: 10.0,
            rotation: 0.0,
            scale: 1.0,
        });
        bus.run_checks(&world, (960.0, 540.0), 1);
        assert_eq!(bus.len(), 1);
        let d = &bus.diagnostics()[0];
        assert_eq!(d.level, DiagnosticLevel::Warning);
        assert_eq!(d.category, DiagnosticCategory::Physics);
        assert!(d.message.contains("out of bounds"));
    }

    #[test]
    fn run_checks_detects_high_entity_count() {
        let mut bus = DiagnosticBus::new();
        let mut world = World::new();
        for _ in 0..501 {
            world.spawn();
        }
        bus.run_checks(&world, (960.0, 540.0), 0);
        assert!(bus.diagnostics().iter().any(|d| {
            d.level == DiagnosticLevel::Warning
                && d.category == DiagnosticCategory::Performance
                && d.message.contains("501")
        }));
    }

    #[test]
    fn run_checks_no_issues_for_healthy_world() {
        let mut bus = DiagnosticBus::new();
        let mut world = World::new();
        let e = world.spawn();
        world.transforms.insert(e, Transform {
            x: 100.0,
            y: 100.0,
            rotation: 0.0,
            scale: 1.0,
        });
        bus.run_checks(&world, (960.0, 540.0), 0);
        assert!(bus.is_empty());
    }

    #[test]
    fn default_impl_creates_empty_bus() {
        let bus = DiagnosticBus::default();
        assert!(bus.is_empty());
    }
}
