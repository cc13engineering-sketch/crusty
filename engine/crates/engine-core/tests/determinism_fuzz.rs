//! Determinism fuzz test — runs many seeds x 2 to verify identical state hashes.
//!
//! Each seed is run twice with the same inputs. If any pair produces different
//! state hashes, the simulation is non-deterministic.

use engine_core::demo_ball::DemoBall;
use engine_core::headless::{HeadlessRunner, RunConfig};
use engine_core::input_frame::InputFrame;

/// Number of seeds to test. 100 provides good coverage without being slow.
const FUZZ_SEEDS: u64 = 100;
/// Frames per run.
const FRAMES: u64 = 120;

fn run_once(seed: u64) -> (u64, Vec<u64>) {
    let mut runner = HeadlessRunner::new(480, 270);
    let mut game = DemoBall::new();
    let inputs: Vec<InputFrame> = vec![InputFrame::default(); FRAMES as usize];
    let config = RunConfig {
        turbo: true,
        capture_state_hashes: true,
    };
    let result = runner.run_sim_frames(&mut game, seed, &inputs, FRAMES, config);
    (result.state_hash, result.state_hashes)
}

#[test]
fn determinism_fuzz_null_input() {
    let mut failures = Vec::new();

    for seed in 0..FUZZ_SEEDS {
        let (hash1, hashes1) = run_once(seed);
        let (hash2, hashes2) = run_once(seed);

        if hash1 != hash2 {
            // Find first divergent frame
            let diverge_frame = hashes1.iter().zip(hashes2.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(hashes1.len());
            failures.push((seed, diverge_frame, hash1, hash2));
        }
    }

    assert!(
        failures.is_empty(),
        "Determinism failures for {} out of {} seeds:\n{}",
        failures.len(),
        FUZZ_SEEDS,
        failures.iter()
            .map(|(seed, frame, h1, h2)| format!(
                "  seed {}: diverged at frame {}, hash1={:#x}, hash2={:#x}",
                seed, frame, h1, h2
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Fuzz with random policy input to exercise more code paths.
#[test]
fn determinism_fuzz_random_policy() {
    use engine_core::policy::RandomPolicy;

    let mut failures = Vec::new();

    for seed in 0..FUZZ_SEEDS {
        let keys = vec!["Space".into(), "KeyA".into(), "KeyD".into()];
        let config = RunConfig {
            turbo: true,
            capture_state_hashes: true,
        };

        let mut runner1 = HeadlessRunner::new(480, 270);
        let mut game1 = DemoBall::new();
        let mut policy1 = RandomPolicy::new(seed.wrapping_mul(7919), keys.clone());
        let res1 = runner1.run_with_policy(&mut game1, &mut policy1, seed, FRAMES, config.clone());

        let mut runner2 = HeadlessRunner::new(480, 270);
        let mut game2 = DemoBall::new();
        let mut policy2 = RandomPolicy::new(seed.wrapping_mul(7919), keys);
        let res2 = runner2.run_with_policy(&mut game2, &mut policy2, seed, FRAMES, config);

        if res1.state_hash != res2.state_hash {
            let diverge_frame = res1.state_hashes.iter().zip(res2.state_hashes.iter())
                .position(|(a, b)| a != b)
                .unwrap_or(res1.state_hashes.len());
            failures.push((seed, diverge_frame, res1.state_hash, res2.state_hash));
        }
    }

    assert!(
        failures.is_empty(),
        "Determinism failures with random policy for {} out of {} seeds:\n{}",
        failures.len(),
        FUZZ_SEEDS,
        failures.iter()
            .map(|(seed, frame, h1, h2)| format!(
                "  seed {}: diverged at frame {}, hash1={:#x}, hash2={:#x}",
                seed, frame, h1, h2
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
