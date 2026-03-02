use std::collections::HashMap;
use super::scenario::ScheduledAction;

/// A recorded simulation replay: inputs + state snapshots at every frame.
///
/// Designed for AI-driven analysis: record a run, then query state at any
/// frame, diff against other replays, or detect anomalies.
#[derive(Clone, Debug)]
pub struct Replay {
    /// Human-readable name for this replay.
    pub name: String,
    /// The actions that were scheduled during this run.
    pub actions: Vec<ScheduledAction>,
    /// Per-frame state snapshots (index = frame number).
    pub frames: Vec<ReplayFrame>,
    /// Final framebuffer hash.
    pub final_fb_hash: u64,
}

/// State captured at a single frame.
#[derive(Clone, Debug)]
pub struct ReplayFrame {
    pub frame: u64,
    pub state: HashMap<String, f64>,
    pub fb_hash: u64,
}

impl Replay {
    /// Get state at a specific frame.
    pub fn at(&self, frame: usize) -> Option<&ReplayFrame> {
        self.frames.get(frame)
    }

    /// Get a state value series across all frames.
    pub fn series(&self, key: &str) -> Vec<f64> {
        self.frames.iter().map(|f| f.state.get(key).copied().unwrap_or(0.0)).collect()
    }

    /// Number of frames in this replay.
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Get the value of a state key at a specific frame.
    pub fn get(&self, frame: usize, key: &str) -> Option<f64> {
        self.frames.get(frame)?.state.get(key).copied()
    }

    /// Find the first frame where a condition is met.
    pub fn first_frame_where(&self, key: &str, predicate: impl Fn(f64) -> bool) -> Option<usize> {
        self.frames.iter().position(|f| {
            f.state.get(key).map_or(false, |v| predicate(*v))
        })
    }
}

/// Record a complete replay with per-frame state capture.
pub fn record_replay(
    name: &str,
    setup_fn: fn(&mut crate::engine::Engine),
    update_fn: fn(&mut crate::engine::Engine, f64),
    render_fn: fn(&mut crate::engine::Engine),
    action_dispatch: fn(&mut crate::engine::Engine, &ScheduledAction),
    actions: &[ScheduledAction],
    total_frames: u64,
    tracked_keys: &[&str],
) -> Replay {
    let mut runner = super::runner::HeadlessRunner::new(480, 720);
    let mut sorted_actions = actions.to_vec();
    sorted_actions.sort_by_key(|a| a.frame());

    setup_fn(&mut runner.engine);
    let dt = 1.0 / 60.0;
    let mut frames = Vec::with_capacity(total_frames as usize);

    for frame in 0..total_frames {
        runner.engine.tick(dt);

        for action in &sorted_actions {
            if action.frame() == frame {
                action_dispatch(&mut runner.engine, action);
            }
        }

        update_fn(&mut runner.engine, dt);
        render_fn(&mut runner.engine);

        let mut state = HashMap::new();
        for key in tracked_keys {
            if let Some(v) = runner.engine.global_state.get_f64(key) {
                state.insert(key.to_string(), v);
            }
        }

        frames.push(ReplayFrame {
            frame,
            state,
            fb_hash: super::framebuffer_hash(&runner.engine.framebuffer),
        });
    }

    Replay {
        name: name.to_string(),
        actions: actions.to_vec(),
        frames,
        final_fb_hash: super::framebuffer_hash(&runner.engine.framebuffer),
    }
}
