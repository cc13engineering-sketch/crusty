use std::collections::HashMap;
use crate::engine::Engine;
use crate::game_state::StateValue;
use super::runner::HeadlessRunner;

/// A captured snapshot of game state at a specific frame.
#[derive(Clone, Debug)]
pub struct FrameSnapshot {
    /// The frame number when this snapshot was taken.
    pub frame: u64,
    /// Game state key-value pairs at this frame.
    pub state: HashMap<String, StateValue>,
    /// Framebuffer hash at this frame.
    pub framebuffer_hash: u64,
}

impl FrameSnapshot {
    /// Get a numeric state value.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.state.get(key).and_then(|v| v.as_f64())
    }

    /// Get a string state value.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.state.get(key).and_then(|v| v.as_str())
    }
}

/// Result of a simulation with mid-frame snapshots.
#[derive(Clone, Debug)]
pub struct SnapshotResult {
    /// The final simulation result.
    pub sim: super::runner::SimResult,
    /// Snapshots captured at requested frames.
    pub snapshots: Vec<FrameSnapshot>,
}

impl SnapshotResult {
    /// Get the snapshot for a specific frame, if captured.
    pub fn at_frame(&self, frame: u64) -> Option<&FrameSnapshot> {
        self.snapshots.iter().find(|s| s.frame == frame)
    }

    /// Get a state value across all snapshots as a series.
    pub fn series(&self, key: &str) -> Vec<(u64, f64)> {
        self.snapshots
            .iter()
            .filter_map(|s| {
                s.get_f64(key).map(|v| (s.frame, v))
            })
            .collect()
    }

    /// Check if a state value changed between two captured frames.
    pub fn value_changed(&self, key: &str, frame_a: u64, frame_b: u64) -> Option<bool> {
        let a = self.at_frame(frame_a)?.get_f64(key)?;
        let b = self.at_frame(frame_b)?.get_f64(key)?;
        Some((a - b).abs() > f64::EPSILON)
    }
}

/// Run a simulation capturing snapshots at specified frames.
///
/// Game-agnostic: supply your own setup/update/render/action_dispatch.
pub fn run_with_snapshots(
    setup_fn: fn(&mut Engine),
    update_fn: fn(&mut Engine, f64),
    render_fn: fn(&mut Engine),
    action_dispatch: fn(&mut Engine, &super::scenario::ScheduledAction),
    actions: &[super::scenario::ScheduledAction],
    total_frames: u64,
    snapshot_frames: &[u64],
) -> SnapshotResult {
    let mut runner = HeadlessRunner::new(480, 720);
    let mut sorted_actions = actions.to_vec();
    sorted_actions.sort_by_key(|a| a.frame());
    let mut snapshot_set: Vec<u64> = snapshot_frames.to_vec();
    snapshot_set.sort();

    let mut snapshots = Vec::new();

    setup_fn(&mut runner.engine);
    let dt = 1.0 / 60.0;

    for frame in 0..total_frames {
        runner.engine.tick(dt);

        for action in &sorted_actions {
            if action.frame() == frame {
                action_dispatch(&mut runner.engine, action);
            }
        }

        update_fn(&mut runner.engine, dt);
        render_fn(&mut runner.engine);

        if snapshot_set.binary_search(&frame).is_ok() {
            let state: HashMap<String, StateValue> = runner
                .engine
                .global_state
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect();
            snapshots.push(FrameSnapshot {
                frame,
                state,
                framebuffer_hash: super::framebuffer_hash(&runner.engine.framebuffer),
            });
        }
    }

    let sim = super::runner::SimResult {
        frames_run: total_frames,
        final_metrics: runner.engine.frame_metrics.clone(),
        game_state: runner
            .engine
            .global_state
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect(),
        framebuffer_hash: super::framebuffer_hash(&runner.engine.framebuffer),
        elapsed_sim_time: runner.engine.time,
    };

    SnapshotResult { sim, snapshots }
}
