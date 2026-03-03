use crate::input_frame::InputFrame;
use crate::observation::Observation;

/// A policy decides what input to produce each frame based on observations.
///
/// Policies are used by headless runners to drive simulations programmatically.
/// They observe the engine state and produce an `InputFrame` for the next tick.
///
/// Important: policy-internal RNG must be separate from the engine's RNG.
/// When replaying a recorded policy's outputs, the engine must produce
/// identical results regardless of how the policy generated those outputs.
pub trait Policy {
    /// Produce the next input frame based on the current observation.
    fn next_input(&mut self, obs: &Observation) -> InputFrame;
}

/// A policy that produces empty input every frame.
pub struct NullPolicy;

impl Policy for NullPolicy {
    fn next_input(&mut self, _obs: &Observation) -> InputFrame {
        InputFrame::default()
    }
}

/// A policy that produces random input from a separate seeded RNG.
///
/// Uses its own LCG PRNG so that policy randomness does not affect
/// engine determinism when the resulting inputs are replayed.
pub struct RandomPolicy {
    state: u64,
    keys: Vec<String>,
}

impl RandomPolicy {
    /// Create a random policy with the given seed and available key set.
    pub fn new(seed: u64, keys: Vec<String>) -> Self {
        Self {
            state: seed.wrapping_add(1),
            keys,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

impl Policy for RandomPolicy {
    fn next_input(&mut self, _obs: &Observation) -> InputFrame {
        let mut frame = InputFrame::default();

        // 20% chance of a key press
        if self.next_f64() < 0.2 && !self.keys.is_empty() {
            let idx = (self.next_u64() as usize) % self.keys.len();
            frame.keys_pressed.push(self.keys[idx].clone());
        }

        // 15% chance of pointer down at random position
        if self.next_f64() < 0.15 {
            let x = self.next_f64() * 480.0;
            let y = self.next_f64() * 270.0;
            frame.pointer_down = Some((x, y));
        }

        // 10% chance of pointer up
        if self.next_f64() < 0.10 {
            let x = self.next_f64() * 480.0;
            let y = self.next_f64() * 270.0;
            frame.pointer_up = Some((x, y));
        }

        frame
    }
}

/// A policy that replays a fixed sequence of input frames.
pub struct ScriptedPolicy {
    inputs: Vec<InputFrame>,
    index: usize,
}

impl ScriptedPolicy {
    pub fn new(inputs: Vec<InputFrame>) -> Self {
        Self { inputs, index: 0 }
    }
}

impl Policy for ScriptedPolicy {
    fn next_input(&mut self, _obs: &Observation) -> InputFrame {
        let frame = self.inputs.get(self.index)
            .cloned()
            .unwrap_or_default();
        self.index += 1;
        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Engine;

    fn make_obs(engine: &Engine) -> Observation<'_> {
        engine.observe()
    }

    #[test]
    fn null_policy_produces_empty_input() {
        let mut policy = NullPolicy;
        let engine = Engine::new(100, 100);
        let obs = make_obs(&engine);
        let input = policy.next_input(&obs);
        assert!(input.is_empty());
    }

    #[test]
    fn random_policy_deterministic() {
        let engine = Engine::new(100, 100);
        let keys = vec!["Space".into(), "KeyA".into()];

        let mut p1 = RandomPolicy::new(42, keys.clone());
        let mut p2 = RandomPolicy::new(42, keys);

        for _ in 0..100 {
            let obs = make_obs(&engine);
            let i1 = p1.next_input(&obs);
            let i2 = p2.next_input(&obs);
            assert_eq!(i1.keys_pressed, i2.keys_pressed);
            assert_eq!(i1.pointer_down, i2.pointer_down);
            assert_eq!(i1.pointer_up, i2.pointer_up);
        }
    }

    #[test]
    fn scripted_policy_replays_sequence() {
        let inputs = vec![
            InputFrame { keys_pressed: vec!["A".into()], ..Default::default() },
            InputFrame { keys_pressed: vec!["B".into()], ..Default::default() },
        ];
        let mut policy = ScriptedPolicy::new(inputs);
        let engine = Engine::new(100, 100);
        let obs = make_obs(&engine);

        let f1 = policy.next_input(&obs);
        assert_eq!(f1.keys_pressed, vec!["A".to_string()]);

        let f2 = policy.next_input(&obs);
        assert_eq!(f2.keys_pressed, vec!["B".to_string()]);

        // Beyond sequence: empty
        let f3 = policy.next_input(&obs);
        assert!(f3.is_empty());
    }
}
