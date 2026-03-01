/// Timer / Scheduled Action system.
///
/// Timers fire after a delay and can optionally repeat. When a timer fires,
/// it produces a named event string that the BehaviorRule system can react to.

/// A single scheduled timer.
#[derive(Clone, Debug)]
pub struct Timer {
    /// Unique name for this timer (used to cancel or reference it).
    pub name: String,
    /// Seconds remaining until next fire.
    pub remaining: f64,
    /// Interval for repeating timers. None = one-shot.
    pub interval: Option<f64>,
    /// How many times this timer has fired.
    pub fire_count: u64,
    /// Maximum number of fires (0 = unlimited for repeating timers).
    pub max_fires: u64,
}

impl Timer {
    /// Create a one-shot timer that fires after `delay` seconds.
    pub fn one_shot(name: &str, delay: f64) -> Self {
        Self {
            name: name.to_string(),
            remaining: delay,
            interval: None,
            fire_count: 0,
            max_fires: 1,
        }
    }

    /// Create a repeating timer that fires every `interval` seconds,
    /// starting after `delay` seconds.
    pub fn repeating(name: &str, delay: f64, interval: f64) -> Self {
        Self {
            name: name.to_string(),
            remaining: delay,
            interval: Some(interval),
            fire_count: 0,
            max_fires: 0, // unlimited
        }
    }

    /// Create a repeating timer with a maximum fire count.
    pub fn repeating_n(name: &str, delay: f64, interval: f64, max_fires: u64) -> Self {
        Self {
            name: name.to_string(),
            remaining: delay,
            interval: Some(interval),
            fire_count: 0,
            max_fires,
        }
    }

    /// Returns true if this timer has exhausted its fire count.
    pub fn is_exhausted(&self) -> bool {
        self.max_fires > 0 && self.fire_count >= self.max_fires
    }
}

/// Manages all active timers and collects fired timer names each frame.
#[derive(Default, Clone, Debug)]
pub struct TimerQueue {
    pub timers: Vec<Timer>,
    /// Timer names that fired this frame (consumed by behavior rules).
    pub fired: Vec<String>,
}

impl TimerQueue {
    pub fn new() -> Self {
        Self { timers: Vec::new(), fired: Vec::new() }
    }

    /// Add a timer to the queue.
    pub fn add(&mut self, timer: Timer) {
        self.timers.push(timer);
    }

    /// Cancel a timer by name. Returns true if found and removed.
    pub fn cancel(&mut self, name: &str) -> bool {
        let before = self.timers.len();
        self.timers.retain(|t| t.name != name);
        self.timers.len() < before
    }

    /// Check if a timer with the given name exists.
    pub fn has(&self, name: &str) -> bool {
        self.timers.iter().any(|t| t.name == name)
    }

    /// Tick all timers by `dt` seconds. Populates `self.fired` with names of
    /// timers that triggered this frame. Removes exhausted one-shot timers.
    pub fn tick(&mut self, dt: f64) {
        self.fired.clear();

        for timer in &mut self.timers {
            timer.remaining -= dt;
            while timer.remaining <= 0.0 && !timer.is_exhausted() {
                timer.fire_count += 1;
                self.fired.push(timer.name.clone());

                match timer.interval {
                    Some(interval) if !timer.is_exhausted() => {
                        timer.remaining += interval;
                    }
                    _ => {
                        // One-shot or exhausted: break, will be cleaned up
                        timer.remaining = 0.0;
                        break;
                    }
                }
            }
        }

        // Remove exhausted timers
        self.timers.retain(|t| !t.is_exhausted());
    }

    /// Clear all timers and fired events.
    pub fn clear(&mut self) {
        self.timers.clear();
        self.fired.clear();
    }

    pub fn len(&self) -> usize {
        self.timers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.timers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_shot_fires_after_delay() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("test", 1.0));
        tq.tick(0.5);
        assert!(tq.fired.is_empty());
        tq.tick(0.5);
        assert_eq!(tq.fired, vec!["test"]);
    }

    #[test]
    fn one_shot_removed_after_firing() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("test", 0.5));
        tq.tick(1.0);
        assert_eq!(tq.fired, vec!["test"]);
        assert!(tq.is_empty());
    }

    #[test]
    fn repeating_fires_multiple_times() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::repeating("spawn", 1.0, 1.0));

        tq.tick(1.0);
        assert_eq!(tq.fired, vec!["spawn"]);
        assert_eq!(tq.len(), 1); // still alive

        tq.tick(1.0);
        assert_eq!(tq.fired, vec!["spawn"]);
        assert_eq!(tq.len(), 1);
    }

    #[test]
    fn repeating_n_exhausts_after_max() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::repeating_n("wave", 1.0, 1.0, 3));

        tq.tick(1.0);
        assert_eq!(tq.fired.len(), 1);
        tq.tick(1.0);
        assert_eq!(tq.fired.len(), 1);
        tq.tick(1.0);
        assert_eq!(tq.fired.len(), 1);
        assert!(tq.is_empty()); // exhausted after 3 fires

        tq.tick(1.0);
        assert!(tq.fired.is_empty()); // no more fires
    }

    #[test]
    fn cancel_removes_timer() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("test", 5.0));
        assert!(tq.cancel("test"));
        assert!(tq.is_empty());
    }

    #[test]
    fn cancel_nonexistent_returns_false() {
        let mut tq = TimerQueue::new();
        assert!(!tq.cancel("nothing"));
    }

    #[test]
    fn has_returns_true_for_active_timer() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("test", 1.0));
        assert!(tq.has("test"));
    }

    #[test]
    fn has_returns_false_for_missing() {
        let tq = TimerQueue::new();
        assert!(!tq.has("test"));
    }

    #[test]
    fn multiple_timers_fire_independently() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("fast", 0.5));
        tq.add(Timer::one_shot("slow", 1.5));

        tq.tick(0.5);
        assert_eq!(tq.fired, vec!["fast"]);
        assert_eq!(tq.len(), 1); // only slow remains

        tq.tick(1.0);
        assert_eq!(tq.fired, vec!["slow"]);
        assert!(tq.is_empty());
    }

    #[test]
    fn clear_removes_all() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("a", 1.0));
        tq.add(Timer::repeating("b", 1.0, 1.0));
        tq.clear();
        assert!(tq.is_empty());
        assert!(tq.fired.is_empty());
    }

    #[test]
    fn large_dt_fires_multiple_repeats() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::repeating("pulse", 1.0, 1.0));
        tq.tick(3.5); // should fire 3 times (at t=1, t=2, t=3)
        assert_eq!(tq.fired.iter().filter(|n| *n == "pulse").count(), 3);
    }

    #[test]
    fn default_is_empty() {
        let tq = TimerQueue::default();
        assert!(tq.is_empty());
        assert!(tq.fired.is_empty());
    }

    #[test]
    fn fired_cleared_each_tick() {
        let mut tq = TimerQueue::new();
        tq.add(Timer::one_shot("test", 0.5));
        tq.tick(1.0);
        assert_eq!(tq.fired.len(), 1);
        tq.tick(1.0);
        assert!(tq.fired.is_empty()); // cleared since no new fires
    }
}
