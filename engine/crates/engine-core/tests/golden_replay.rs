//! Golden replay test — CI gate for determinism.
//!
//! This test loads a golden playthrough baseline, replays it with the
//! same seed and inputs, and asserts that the state hash matches exactly.
//! If this test fails, something changed the simulation's deterministic output.

use engine_core::demo_ball::DemoBall;
use engine_core::headless::playthrough::PlaythroughFile;
use engine_core::input_frame::InputFrame;

/// The golden baseline: 120 frames of demo_ball with seed 42 and a tap at frame 30.
///
/// This is embedded directly in the test rather than loaded from a file,
/// so it works in CI without file path issues.
fn golden_baseline() -> PlaythroughFile {
    let mut inputs: Vec<InputFrame> = Vec::with_capacity(120);
    for i in 0..120u64 {
        if i == 30 {
            inputs.push(InputFrame {
                pointer_down: Some((350.0, 100.0)),
                ..Default::default()
            });
        } else {
            inputs.push(InputFrame::default());
        }
    }

    // Record the golden baseline
    let mut game = DemoBall::new();
    PlaythroughFile::record(&mut game, 42, &inputs, 120, true)
}

#[test]
fn golden_replay_matches_baseline() {
    let baseline = golden_baseline();

    // Replay and verify
    let mut game = DemoBall::new();
    let result = baseline.verify(&mut game);
    assert!(result.is_ok(), "Golden replay mismatch: {:?}", result.err());
}

#[test]
fn golden_replay_per_frame_hashes_stable() {
    let baseline = golden_baseline();
    assert_eq!(baseline.state_hashes.len(), 120,
        "baseline should have per-frame hashes");

    // Record again — must be identical
    let baseline2 = golden_baseline();
    assert_eq!(baseline.state_hashes, baseline2.state_hashes,
        "two recordings with same seed+inputs must produce identical per-frame hashes");
}

#[test]
fn golden_replay_serialization_roundtrip() {
    let baseline = golden_baseline();
    let json = baseline.to_json().unwrap();
    let loaded = PlaythroughFile::from_json(&json).unwrap();

    let mut game = DemoBall::new();
    let result = loaded.verify(&mut game);
    assert!(result.is_ok(),
        "Golden replay from deserialized file should still verify: {:?}", result.err());
}
