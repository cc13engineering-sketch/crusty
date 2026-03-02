use crate::engine::Engine;
use super::runner::HeadlessRunner;

/// A recorded time-series of game state values across frames.
///
/// Use this to analyze object trajectories, velocity profiles, and state
/// transitions without modifying game code.
#[derive(Clone, Debug)]
pub struct StateTimeline {
    /// The keys being tracked.
    pub keys: Vec<String>,
    /// Per-frame snapshots: samples[frame_index][key_index] = value (NaN if missing).
    pub samples: Vec<Vec<f64>>,
}

impl StateTimeline {
    /// Get the full series for a single key.
    pub fn series(&self, key: &str) -> Option<Vec<f64>> {
        let idx = self.keys.iter().position(|k| k == key)?;
        Some(self.samples.iter().map(|row| row[idx]).collect())
    }

    /// Find the first frame where a key's value satisfies a predicate.
    pub fn first_frame_where(&self, key: &str, predicate: impl Fn(f64) -> bool) -> Option<usize> {
        let idx = self.keys.iter().position(|k| k == key)?;
        self.samples.iter().position(|row| predicate(row[idx]))
    }

    /// Compute (min, max, mean) for a tracked key.
    pub fn stats(&self, key: &str) -> Option<(f64, f64, f64)> {
        let series = self.series(key)?;
        let valid: Vec<f64> = series.into_iter().filter(|v| v.is_finite()).collect();
        if valid.is_empty() {
            return None;
        }
        let min = valid.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = valid.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = valid.iter().sum::<f64>() / valid.len() as f64;
        Some((min, max, mean))
    }

    /// Number of frames recorded.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

/// Record a timeline during a headless simulation.
///
/// Captures the specified game state keys at every frame.
pub fn record_timeline(
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    frames: u64,
    track_keys: &[&str],
) -> StateTimeline {
    let keys: Vec<String> = track_keys.iter().map(|s| s.to_string()).collect();
    let mut samples: Vec<Vec<f64>> = Vec::with_capacity(frames as usize);
    let mut runner = HeadlessRunner::new(480, 720);

    setup_fn(&mut runner.engine);
    let dt = 1.0 / 60.0;

    for _ in 0..frames {
        runner.engine.tick(dt);
        update_fn(&mut runner.engine, dt);
        render_fn(&mut runner.engine);

        let row: Vec<f64> = keys
            .iter()
            .map(|k| runner.engine.global_state.get_f64(k).unwrap_or(f64::NAN))
            .collect();
        samples.push(row);
    }

    StateTimeline { keys, samples }
}

/// Record a timeline with scheduled actions.
///
/// Supply your own `action_dispatch` to route ScheduledActions to your
/// game's input handlers.
pub fn record_timeline_with_actions(
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &super::scenario::ScheduledAction),
    actions: &[super::scenario::ScheduledAction],
    frames: u64,
    track_keys: &[&str],
) -> StateTimeline {
    let keys: Vec<String> = track_keys.iter().map(|s| s.to_string()).collect();
    let mut samples: Vec<Vec<f64>> = Vec::with_capacity(frames as usize);
    let mut runner = HeadlessRunner::new(480, 720);
    let mut sorted_actions = actions.to_vec();
    sorted_actions.sort_by_key(|a| a.frame());

    setup_fn(&mut runner.engine);
    let dt = 1.0 / 60.0;

    for frame in 0..frames {
        runner.engine.tick(dt);
        for action in &sorted_actions {
            if action.frame() == frame {
                action_dispatch(&mut runner.engine, action);
            }
        }
        update_fn(&mut runner.engine, dt);
        render_fn(&mut runner.engine);

        let row: Vec<f64> = keys
            .iter()
            .map(|k| runner.engine.global_state.get_f64(k).unwrap_or(f64::NAN))
            .collect();
        samples.push(row);
    }

    StateTimeline { keys, samples }
}
