/// ENGINE MODULE: EnvironmentClock
/// Global cyclical time system for day/night, seasons, weather, etc.

/// A single named phase in a cycle.
#[derive(Clone, Debug)]
pub struct ClockPhase {
    pub name: String,
    pub duration: f64,
}

/// A named cyclical time period with multiple phases.
#[derive(Clone, Debug)]
pub struct TimeCycle {
    pub name: String,
    pub phases: Vec<ClockPhase>,
    pub current: f64,
    pub speed: f64,
    total_duration: f64,
}

impl TimeCycle {
    pub fn new(name: &str, phases: Vec<(&str, f64)>) -> Self {
        let clock_phases: Vec<ClockPhase> = phases.iter()
            .map(|(n, d)| ClockPhase { name: n.to_string(), duration: *d })
            .collect();
        let total = clock_phases.iter().map(|p| p.duration).sum();
        Self {
            name: name.to_string(),
            phases: clock_phases,
            current: 0.0,
            speed: 1.0,
            total_duration: total,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        if self.total_duration <= 0.0 { return; }
        self.current = (self.current + dt * self.speed) % self.total_duration;
    }

    /// Normalized progress through entire cycle (0.0..1.0).
    pub fn progress(&self) -> f64 {
        if self.total_duration <= 0.0 { return 0.0; }
        self.current / self.total_duration
    }

    /// Current phase index and name.
    pub fn current_phase(&self) -> (usize, &str) {
        let mut elapsed = 0.0;
        for (i, phase) in self.phases.iter().enumerate() {
            elapsed += phase.duration;
            if self.current < elapsed {
                return (i, &phase.name);
            }
        }
        let last = self.phases.len().saturating_sub(1);
        (last, self.phases.get(last).map_or("", |p| &p.name))
    }

    /// Progress within the current phase (0.0..1.0).
    pub fn phase_progress(&self) -> f64 {
        let mut elapsed = 0.0;
        for phase in &self.phases {
            if self.current < elapsed + phase.duration {
                return (self.current - elapsed) / phase.duration;
            }
            elapsed += phase.duration;
        }
        1.0
    }

    /// Sinusoidal value tied to cycle (0.0 at start, 1.0 at midpoint, 0.0 at end).
    pub fn sine_value(&self) -> f64 {
        (self.progress() * std::f64::consts::TAU).sin() * 0.5 + 0.5
    }

    pub fn total_duration(&self) -> f64 { self.total_duration }
    pub fn phase_count(&self) -> usize { self.phases.len() }
}

/// Global environment clock managing multiple named time cycles.
#[derive(Default, Debug)]
pub struct EnvironmentClock {
    pub cycles: Vec<TimeCycle>,
    pub paused: bool,
}

impl EnvironmentClock {
    pub fn new() -> Self { Self::default() }

    pub fn add_cycle(&mut self, cycle: TimeCycle) {
        self.cycles.push(cycle);
    }

    pub fn tick(&mut self, dt: f64) {
        if self.paused { return; }
        for cycle in &mut self.cycles {
            cycle.tick(dt);
        }
    }

    pub fn get_cycle(&self, name: &str) -> Option<&TimeCycle> {
        self.cycles.iter().find(|c| c.name == name)
    }

    pub fn get_cycle_mut(&mut self, name: &str) -> Option<&mut TimeCycle> {
        self.cycles.iter_mut().find(|c| c.name == name)
    }

    pub fn phase(&self, cycle_name: &str) -> Option<&str> {
        self.get_cycle(cycle_name).map(|c| c.current_phase().1)
    }

    pub fn progress(&self, cycle_name: &str) -> f64 {
        self.get_cycle(cycle_name).map_or(0.0, |c| c.progress())
    }

    pub fn cycle_count(&self) -> usize { self.cycles.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day_night_cycle() -> TimeCycle {
        TimeCycle::new("day_night", vec![
            ("dawn", 10.0),
            ("day", 40.0),
            ("dusk", 10.0),
            ("night", 40.0),
        ])
    }

    #[test]
    fn new_cycle_starts_at_zero() {
        let c = day_night_cycle();
        assert_eq!(c.progress(), 0.0);
        assert_eq!(c.current_phase().1, "dawn");
    }

    #[test]
    fn tick_advances_time() {
        let mut c = day_night_cycle();
        c.tick(5.0);
        assert_eq!(c.current, 5.0);
        assert_eq!(c.current_phase().1, "dawn");
    }

    #[test]
    fn tick_transitions_phase() {
        let mut c = day_night_cycle();
        c.tick(15.0); // past dawn (10s), into day
        assert_eq!(c.current_phase().1, "day");
    }

    #[test]
    fn tick_wraps_around() {
        let mut c = day_night_cycle();
        c.tick(100.0); // full cycle = 100s, wraps to 0
        assert!((c.current).abs() < 0.001);
        assert_eq!(c.current_phase().1, "dawn");
    }

    #[test]
    fn phase_progress_midway() {
        let mut c = day_night_cycle();
        c.tick(5.0); // halfway through dawn (10s)
        let prog = c.phase_progress();
        assert!((prog - 0.5).abs() < 0.01);
    }

    #[test]
    fn speed_multiplier() {
        let mut c = day_night_cycle();
        c.speed = 2.0;
        c.tick(5.0); // effective 10s
        assert_eq!(c.current, 10.0);
        assert_eq!(c.current_phase().1, "day");
    }

    #[test]
    fn sine_value_range() {
        let mut c = day_night_cycle();
        for i in 0..100 {
            c.current = i as f64;
            let v = c.sine_value();
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn environment_clock_multiple_cycles() {
        let mut clock = EnvironmentClock::new();
        clock.add_cycle(day_night_cycle());
        clock.add_cycle(TimeCycle::new("season", vec![
            ("spring", 100.0), ("summer", 100.0), ("autumn", 100.0), ("winter", 100.0),
        ]));
        assert_eq!(clock.cycle_count(), 2);

        clock.tick(15.0);
        assert_eq!(clock.phase("day_night"), Some("day"));
        assert_eq!(clock.phase("season"), Some("spring"));
    }

    #[test]
    fn environment_clock_paused() {
        let mut clock = EnvironmentClock::new();
        clock.add_cycle(day_night_cycle());
        clock.paused = true;
        clock.tick(50.0);
        assert_eq!(clock.progress("day_night"), 0.0);
    }

    #[test]
    fn total_duration_correct() {
        let c = day_night_cycle();
        assert_eq!(c.total_duration(), 100.0);
    }

    #[test]
    fn phase_count() {
        let c = day_night_cycle();
        assert_eq!(c.phase_count(), 4);
    }
}
