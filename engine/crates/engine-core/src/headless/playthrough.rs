//! PlaythroughFile — serializable replay format for deterministic verification.
//!
//! A playthrough captures everything needed to reproduce a simulation run:
//! seed, inputs, and expected hashes. It can be written to JSON, loaded back,
//! and verified by replaying with the same seed+inputs and comparing hashes.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::input_frame::InputFrame;

/// A complete recorded simulation run, serializable to/from JSON.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaythroughFile {
    /// Engine version string for compatibility checking.
    pub engine_version: String,
    /// RNG seed used for this run.
    pub seed: u64,
    /// Per-frame input sequence.
    pub inputs: Vec<InputFrame>,
    /// Total number of frames simulated.
    pub frame_count: u64,
    /// Simulation state hash at the final frame.
    pub final_state_hash: u64,
    /// Framebuffer hash at the final frame.
    pub final_fb_hash: u64,
    /// Optional per-frame state hashes for fine-grained verification.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub state_hashes: Vec<u64>,
    /// Arbitrary metadata (game name, test scenario, etc.).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

impl PlaythroughFile {
    /// Serialize to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to a pretty-printed JSON string.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Record a playthrough from a simulation run.
    pub fn record<S: crate::simulation::Simulation>(
        sim: &mut S,
        seed: u64,
        inputs: &[InputFrame],
        frames: u64,
        capture_per_frame: bool,
    ) -> Self {
        let config = super::RunConfig {
            turbo: false,
            capture_state_hashes: capture_per_frame,
        };
        let mut runner = super::HeadlessRunner::new(480, 270);
        let result = runner.run_sim_frames(sim, seed, inputs, frames, config);

        Self {
            engine_version: env!("CARGO_PKG_VERSION").to_string(),
            seed,
            inputs: inputs.to_vec(),
            frame_count: frames,
            final_state_hash: result.state_hash,
            final_fb_hash: result.framebuffer_hash,
            state_hashes: result.state_hashes,
            metadata: HashMap::new(),
        }
    }

    /// Verify this playthrough by replaying and comparing hashes.
    ///
    /// Returns `Ok(())` if the replay matches, or `Err(detail)` on mismatch.
    pub fn verify<S: crate::simulation::Simulation>(
        &self,
        sim: &mut S,
    ) -> Result<(), String> {
        let config = super::RunConfig {
            turbo: true,
            capture_state_hashes: !self.state_hashes.is_empty(),
        };
        let mut runner = super::HeadlessRunner::new(480, 270);
        let result = runner.run_sim_frames(
            sim, self.seed, &self.inputs, self.frame_count, config,
        );

        if result.state_hash != self.final_state_hash {
            return Err(format!(
                "state hash mismatch: expected {:#x}, got {:#x}",
                self.final_state_hash, result.state_hash
            ));
        }

        // Check per-frame hashes if available
        if !self.state_hashes.is_empty() {
            for (i, (expected, actual)) in self.state_hashes.iter()
                .zip(result.state_hashes.iter()).enumerate()
            {
                if expected != actual {
                    return Err(format!(
                        "state hash mismatch at frame {}: expected {:#x}, got {:#x}",
                        i, expected, actual
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo_ball::DemoBall;

    #[test]
    fn record_and_verify_roundtrip() {
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); 30];
        let mut game = DemoBall::new();
        let playthrough = PlaythroughFile::record(&mut game, 42, &inputs, 30, false);

        let mut game2 = DemoBall::new();
        assert!(playthrough.verify(&mut game2).is_ok());
    }

    #[test]
    fn record_with_per_frame_hashes() {
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); 10];
        let mut game = DemoBall::new();
        let pt = PlaythroughFile::record(&mut game, 42, &inputs, 10, true);

        assert_eq!(pt.state_hashes.len(), 10);
        let mut game2 = DemoBall::new();
        assert!(pt.verify(&mut game2).is_ok());
    }

    #[test]
    fn serde_roundtrip() {
        let inputs = vec![
            InputFrame { keys_pressed: vec!["Space".into()], ..Default::default() },
            InputFrame::default(),
        ];
        let mut game = DemoBall::new();
        let pt = PlaythroughFile::record(&mut game, 99, &inputs, 2, false);

        let json = pt.to_json().unwrap();
        let loaded = PlaythroughFile::from_json(&json).unwrap();

        assert_eq!(loaded.seed, 99);
        assert_eq!(loaded.frame_count, 2);
        assert_eq!(loaded.final_state_hash, pt.final_state_hash);
    }

    #[test]
    fn verify_detects_wrong_seed() {
        let inputs: Vec<InputFrame> = vec![InputFrame::default(); 10];
        let mut game = DemoBall::new();
        let mut pt = PlaythroughFile::record(&mut game, 42, &inputs, 10, false);

        // Corrupt the seed
        pt.seed = 999;
        let mut game2 = DemoBall::new();
        assert!(pt.verify(&mut game2).is_err());
    }
}
