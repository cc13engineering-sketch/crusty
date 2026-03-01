/// Aim preview — simulates ball trajectory and returns ghost dot positions.

#[derive(Clone, Debug)]
pub struct ArcPoint {
    pub x: f64,
    pub y: f64,
    pub speed: f64,      // velocity magnitude at this point
    pub in_hazard: bool,  // true if point is in a hazard zone
}

#[derive(Clone, Debug)]
pub struct AimConfig {
    pub dot_count: usize,       // number of preview dots (default 5)
    pub time_step: f64,         // seconds between dots (default 0.1)
    pub drag: f64,              // velocity drag per second (default 0.04)
    pub gravity_y: f64,         // gravity force (default 0.0 for top-down)
    pub ball_radius: f64,       // ball collision radius (default 5.0)
    pub restitution: f64,       // wall bounce restitution (default 0.8)
}

impl Default for AimConfig {
    fn default() -> Self {
        Self {
            dot_count: 5,
            time_step: 0.1,
            drag: 0.04,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 0.8,
        }
    }
}

/// Simulate ball trajectory and return ghost dot positions for an aim preview line.
///
/// `start_x`, `start_y`: initial ball position.
/// `vel_x`, `vel_y`: initial ball velocity.
/// `config`: simulation parameters (dot count, drag, gravity, etc.).
/// `is_solid`: closure that returns `true` if a world position is solid (wall).
///
/// Returns a `Vec<ArcPoint>` with one entry per preview dot.
pub fn compute_arc(
    start_x: f64,
    start_y: f64,
    vel_x: f64,
    vel_y: f64,
    config: &AimConfig,
    is_solid: impl Fn(f64, f64) -> bool,
) -> Vec<ArcPoint> {
    let mut points = Vec::with_capacity(config.dot_count);

    let mut px = start_x;
    let mut py = start_y;
    let mut vx = vel_x;
    let mut vy = vel_y;

    for _ in 0..config.dot_count {
        // Apply drag: reduce velocity each step
        let drag_factor = 1.0 - config.drag;
        vx *= drag_factor;
        vy *= drag_factor;

        // Apply gravity
        vy += config.gravity_y * config.time_step;

        // Compute new position via Euler integration
        let new_x = px + vx * config.time_step;
        let new_y = py + vy * config.time_step;

        // Check for wall collision
        if is_solid(new_x, new_y) {
            // Determine which axis crossed into a solid tile by testing each axis independently
            let solid_x = is_solid(new_x, py);
            let solid_y = is_solid(px, new_y);

            match (solid_x, solid_y) {
                (true, true) => {
                    // Corner collision: reflect both axes
                    vx = -vx * config.restitution;
                    vy = -vy * config.restitution;
                }
                (true, false) => {
                    // Hit a wall on the X axis
                    vx = -vx * config.restitution;
                    vy *= config.restitution;
                }
                (false, true) => {
                    // Hit a wall on the Y axis
                    vy = -vy * config.restitution;
                    vx *= config.restitution;
                }
                (false, false) => {
                    // Diagonal entry only — reflect both
                    vx = -vx * config.restitution;
                    vy = -vy * config.restitution;
                }
            }

            // Recompute position after reflection (stay at old position + reflected step)
            let reflected_x = px + vx * config.time_step;
            let reflected_y = py + vy * config.time_step;
            px = reflected_x;
            py = reflected_y;
        } else {
            px = new_x;
            py = new_y;
        }

        let speed = (vx * vx + vy * vy).sqrt();
        points.push(ArcPoint {
            x: px,
            y: py,
            speed,
            in_hazard: false,
        });
    }

    points
}

/// Compute arc with hazard zone detection.
///
/// Same as `compute_arc`, but also accepts a hazard-checking closure.
/// Points inside a hazard zone will have `in_hazard` set to `true`.
pub fn compute_arc_with_hazards(
    start_x: f64,
    start_y: f64,
    vel_x: f64,
    vel_y: f64,
    config: &AimConfig,
    is_solid: impl Fn(f64, f64) -> bool,
    is_hazard: impl Fn(f64, f64) -> bool,
) -> Vec<ArcPoint> {
    let mut points = compute_arc(start_x, start_y, vel_x, vel_y, config, is_solid);
    for point in &mut points {
        point.in_hazard = is_hazard(point.x, point.y);
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn no_walls(_x: f64, _y: f64) -> bool {
        false
    }

    // ── Test 1: Straight shot (no walls) produces linear dots ─────────

    #[test]
    fn straight_shot_no_walls_produces_linear_dots() {
        let config = AimConfig {
            dot_count: 5,
            time_step: 0.1,
            drag: 0.0,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 0.8,
        };

        let points = compute_arc(0.0, 0.0, 100.0, 0.0, &config, no_walls);

        assert_eq!(points.len(), 5);
        // With no drag and no gravity, ball moves 100 * 0.1 = 10 units per step in X
        for (i, p) in points.iter().enumerate() {
            let expected_x = (i + 1) as f64 * 10.0;
            assert!(
                approx_eq(p.x, expected_x),
                "dot {} x: expected {}, got {}", i, expected_x, p.x
            );
            assert!(
                approx_eq(p.y, 0.0),
                "dot {} y: expected 0.0, got {}", i, p.y
            );
        }
    }

    // ── Test 2: Drag reduces speed over time ─────────────────────────

    #[test]
    fn drag_reduces_speed_over_time() {
        let config = AimConfig {
            dot_count: 5,
            time_step: 0.1,
            drag: 0.1,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 0.8,
        };

        let points = compute_arc(0.0, 0.0, 100.0, 0.0, &config, no_walls);

        assert_eq!(points.len(), 5);
        // Speed should decrease each step
        for i in 1..points.len() {
            assert!(
                points[i].speed < points[i - 1].speed,
                "speed should decrease: dot {} speed {} >= dot {} speed {}",
                i, points[i].speed, i - 1, points[i - 1].speed
            );
        }
        // First dot speed should be 100 * 0.9 = 90
        assert!(approx_eq(points[0].speed, 90.0));
    }

    // ── Test 3: Wall bounce reflects correctly ───────────────────────

    #[test]
    fn wall_bounce_reflects_correctly() {
        let config = AimConfig {
            dot_count: 3,
            time_step: 0.1,
            drag: 0.0,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 1.0, // perfect bounce
        };

        // Wall at x >= 5: any position with x >= 5 is solid
        let wall_at_x5 = |x: f64, _y: f64| -> bool { x >= 5.0 };

        // Moving right at 100 units/s, would reach x=10 in 0.1s
        // But x=10 is solid, and (10, 0) is solid while (0, 0) is not
        // So it's a wall on the X axis: vx reflects
        let points = compute_arc(0.0, 0.0, 100.0, 0.0, &config, wall_at_x5);

        assert_eq!(points.len(), 3);
        // First dot: should have bounced, vx is now -100
        // After reflection: new_x = 0 + (-100) * 0.1 = -10
        assert!(
            points[0].x < 0.0,
            "should have bounced back: x = {}", points[0].x
        );
    }

    // ── Test 4: Restitution scales post-bounce velocity ──────────────

    #[test]
    fn restitution_scales_post_bounce_velocity() {
        let config_full = AimConfig {
            dot_count: 1,
            time_step: 0.1,
            drag: 0.0,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 1.0,
        };
        let config_half = AimConfig {
            restitution: 0.5,
            ..config_full.clone()
        };

        let wall_at_x5 = |x: f64, _y: f64| -> bool { x >= 5.0 };

        let points_full = compute_arc(0.0, 0.0, 100.0, 0.0, &config_full, wall_at_x5);
        let points_half = compute_arc(0.0, 0.0, 100.0, 0.0, &config_half, wall_at_x5);

        // With restitution 0.5, the post-bounce speed should be half of restitution 1.0
        assert!(
            approx_eq(points_half[0].speed, points_full[0].speed * 0.5),
            "half restitution speed {} should be half of full {} (expected {})",
            points_half[0].speed, points_full[0].speed, points_full[0].speed * 0.5
        );
    }

    // ── Test 5: Zero velocity produces all dots at start ─────────────

    #[test]
    fn zero_velocity_produces_all_dots_at_start() {
        let config = AimConfig::default();

        let points = compute_arc(50.0, 75.0, 0.0, 0.0, &config, no_walls);

        assert_eq!(points.len(), config.dot_count);
        for (i, p) in points.iter().enumerate() {
            assert!(
                approx_eq(p.x, 50.0) && approx_eq(p.y, 75.0),
                "dot {} should be at start: ({}, {})", i, p.x, p.y
            );
            assert!(
                approx_eq(p.speed, 0.0),
                "dot {} speed should be 0: {}", i, p.speed
            );
        }
    }

    // ── Test 6: dot_count controls output length ─────────────────────

    #[test]
    fn dot_count_controls_output_length() {
        for count in [0, 1, 3, 10, 20] {
            let config = AimConfig {
                dot_count: count,
                ..AimConfig::default()
            };
            let points = compute_arc(0.0, 0.0, 50.0, 50.0, &config, no_walls);
            assert_eq!(
                points.len(), count,
                "expected {} dots, got {}", count, points.len()
            );
        }
    }

    // ── Test 7: Gravity affects trajectory ───────────────────────────

    #[test]
    fn gravity_affects_trajectory() {
        let config_no_grav = AimConfig {
            dot_count: 5,
            time_step: 0.1,
            drag: 0.0,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 0.8,
        };
        let config_with_grav = AimConfig {
            gravity_y: 500.0, // strong downward gravity
            ..config_no_grav.clone()
        };

        let points_no_grav = compute_arc(0.0, 0.0, 100.0, 0.0, &config_no_grav, no_walls);
        let points_with_grav = compute_arc(0.0, 0.0, 100.0, 0.0, &config_with_grav, no_walls);

        // Without gravity, all Y values should be 0
        for p in &points_no_grav {
            assert!(approx_eq(p.y, 0.0));
        }

        // With gravity, Y values should increase (move downward) over time
        for p in &points_with_grav {
            assert!(
                p.y > 0.0,
                "gravity should push dots downward, but y = {}", p.y
            );
        }

        // Each successive dot should be further down than the last
        for i in 1..points_with_grav.len() {
            assert!(
                points_with_grav[i].y > points_with_grav[i - 1].y,
                "dot {} y ({}) should be > dot {} y ({})",
                i, points_with_grav[i].y, i - 1, points_with_grav[i - 1].y
            );
        }
    }

    // ── Test 8: in_hazard detection works ────────────────────────────

    #[test]
    fn in_hazard_detection_works() {
        let config = AimConfig {
            dot_count: 5,
            time_step: 0.1,
            drag: 0.0,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 0.8,
        };

        // Hazard zone: any position with x >= 20
        let is_hazard = |x: f64, _y: f64| -> bool { x >= 20.0 };

        let points = compute_arc_with_hazards(
            0.0, 0.0, 100.0, 0.0, &config, no_walls, is_hazard,
        );

        assert_eq!(points.len(), 5);
        // Dots at x=10, 20, 30, 40, 50
        // Dots 0 (x=10): not in hazard
        assert!(!points[0].in_hazard, "dot 0 at x={} should not be in hazard", points[0].x);
        // Dots 1..4 (x=20,30,40,50): in hazard
        for i in 1..5 {
            assert!(
                points[i].in_hazard,
                "dot {} at x={} should be in hazard", i, points[i].x
            );
        }
    }

    // ── Test 9: Default config produces expected values ──────────────

    #[test]
    fn default_config_values() {
        let config = AimConfig::default();
        assert_eq!(config.dot_count, 5);
        assert!(approx_eq(config.time_step, 0.1));
        assert!(approx_eq(config.drag, 0.04));
        assert!(approx_eq(config.gravity_y, 0.0));
        assert!(approx_eq(config.ball_radius, 5.0));
        assert!(approx_eq(config.restitution, 0.8));
    }

    // ── Test 10: Y-axis wall bounce ──────────────────────────────────

    #[test]
    fn y_axis_wall_bounce() {
        let config = AimConfig {
            dot_count: 3,
            time_step: 0.1,
            drag: 0.0,
            gravity_y: 0.0,
            ball_radius: 5.0,
            restitution: 1.0,
        };

        // Wall at y >= 5
        let wall_at_y5 = |_x: f64, y: f64| -> bool { y >= 5.0 };

        // Moving downward at 100 units/s
        let points = compute_arc(0.0, 0.0, 0.0, 100.0, &config, wall_at_y5);

        assert_eq!(points.len(), 3);
        // Should bounce upward: first dot y should be negative after bounce
        assert!(
            points[0].y < 0.0,
            "should have bounced up: y = {}", points[0].y
        );
    }
}
