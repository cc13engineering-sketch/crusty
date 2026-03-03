/// Deterministic seeded RNG (xorshift64).
///
/// This is the engine's canonical random number generator. All game logic
/// and engine systems that need randomness should use this RNG through
/// `Engine::rng`. Using any other RNG source in engine-core breaks determinism.
#[derive(Clone, Debug)]
pub struct SeededRng {
    pub(crate) state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }

    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }

    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        if max <= min { return min; }
        let range = (max as i64 - min as i64 + 1) as u64;
        (min as i64 + (self.next_u64() % range) as i64) as i32
    }

    pub fn chance(&mut self, probability: f64) -> bool {
        self.next_f64() < probability
    }

    /// Reseed the RNG.
    pub fn reseed(&mut self, seed: u64) {
        self.state = seed.max(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let mut a = SeededRng::new(42);
        let mut b = SeededRng::new(42);
        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn range_f64_in_bounds() {
        let mut rng = SeededRng::new(123);
        for _ in 0..100 {
            let v = rng.range_f64(0.0, 1.0);
            assert!(v >= 0.0 && v <= 1.0);
        }
    }

    #[test]
    fn range_i32_in_bounds() {
        let mut rng = SeededRng::new(456);
        for _ in 0..100 {
            let v = rng.range_i32(0, 10);
            assert!(v >= 0 && v <= 10);
        }
    }

    #[test]
    fn chance_roughly_correct() {
        let mut rng = SeededRng::new(789);
        let mut trues = 0;
        for _ in 0..1000 {
            if rng.chance(0.5) { trues += 1; }
        }
        assert!(trues > 300 && trues < 700);
    }

    #[test]
    fn reseed_resets() {
        let mut rng = SeededRng::new(42);
        let first = rng.next_u64();
        rng.reseed(42);
        assert_eq!(rng.next_u64(), first);
    }
}
