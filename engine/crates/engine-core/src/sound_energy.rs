//! Sound-reactive visual effects energy tracker.
//!
//! Tracks "sound energy" levels that visual systems can sample to drive
//! beat-reactive or frequency-reactive effects.  Energy is spiked by game
//! events (collisions, note hits, etc.) and decays smoothly each tick.

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A frequency range in hertz, with an influence weight.
#[derive(Clone, Debug)]
pub struct FrequencyRange {
    pub min_hz: f64,
    pub max_hz: f64,
    pub weight: f64,
}

impl FrequencyRange {
    /// Returns `true` when `freq` falls within `[min_hz, max_hz]`.
    pub fn contains(&self, freq: f64) -> bool {
        freq >= self.min_hz && freq <= self.max_hz
    }
}

/// A single frequency band with its own energy level and decay rate.
#[derive(Clone, Debug)]
pub struct EnergyBand {
    pub range: FrequencyRange,
    pub level: f64,
    pub decay_rate: f64,
}

/// Tracks overall and per-band sound energy that visuals can respond to.
///
/// Energy levels normally sit in the `0.0..=1.0` range but are allowed to
/// exceed `1.0` on spikes so downstream effects can detect transients.
#[derive(Clone, Debug)]
pub struct SoundEnergy {
    /// Overall energy level (0.0 to 1.0+, can exceed 1.0 on spikes).
    level: f64,
    /// How fast overall energy decays per second.
    decay_rate: f64,
    /// Per-band energy levels for frequency-specific reactivity.
    bands: Vec<EnergyBand>,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl SoundEnergy {
    /// Create a new `SoundEnergy` with the given overall decay rate and no
    /// frequency bands.
    pub fn new(decay_rate: f64) -> Self {
        Self {
            level: 0.0,
            decay_rate,
            bands: Vec::new(),
        }
    }

    /// Add a frequency band that will be tracked independently.
    pub fn add_band(&mut self, range: FrequencyRange, decay_rate: f64) {
        self.bands.push(EnergyBand {
            range,
            level: 0.0,
            decay_rate,
        });
    }

    /// Spike the overall energy by `amount`.
    pub fn spike(&mut self, amount: f64) {
        self.level += amount;
    }

    /// Spike a specific band by `amount`.  Out-of-range indices are silently
    /// ignored.
    pub fn spike_band(&mut self, band_index: usize, amount: f64) {
        if let Some(band) = self.bands.get_mut(band_index) {
            band.level += amount;
        }
    }

    /// Spike every band whose frequency range contains `freq_hz`, scaled by
    /// the band's weight.
    pub fn spike_frequency(&mut self, freq_hz: f64, amount: f64) {
        for band in &mut self.bands {
            if band.range.contains(freq_hz) {
                band.level += amount * band.range.weight;
            }
        }
    }

    /// Decay all energy levels by their respective rates over `dt` seconds.
    /// Levels are clamped so they never go below zero.
    pub fn tick(&mut self, dt: f64) {
        self.level = (self.level - self.decay_rate * dt).max(0.0);
        for band in &mut self.bands {
            band.level = (band.level - band.decay_rate * dt).max(0.0);
        }
    }

    /// Current overall energy level.
    pub fn level(&self) -> f64 {
        self.level
    }

    /// Energy level for a specific band.  Returns `0.0` if `index` is out of
    /// range.
    pub fn band_level(&self, index: usize) -> f64 {
        self.bands.get(index).map_or(0.0, |b| b.level)
    }

    /// Number of registered frequency bands.
    pub fn band_count(&self) -> usize {
        self.bands.len()
    }

    /// Reset all energy levels (overall and per-band) to zero.
    pub fn clear(&mut self) {
        self.level = 0.0;
        for band in &mut self.bands {
            band.level = 0.0;
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn bass_range() -> FrequencyRange {
        FrequencyRange {
            min_hz: 20.0,
            max_hz: 250.0,
            weight: 1.0,
        }
    }

    fn mid_range() -> FrequencyRange {
        FrequencyRange {
            min_hz: 250.0,
            max_hz: 4000.0,
            weight: 0.5,
        }
    }

    fn high_range() -> FrequencyRange {
        FrequencyRange {
            min_hz: 4000.0,
            max_hz: 20000.0,
            weight: 0.25,
        }
    }

    #[test]
    fn new_energy_starts_at_zero() {
        let energy = SoundEnergy::new(1.0);
        assert_eq!(energy.level(), 0.0);
        assert_eq!(energy.band_count(), 0);
    }

    #[test]
    fn spike_increases_level() {
        let mut energy = SoundEnergy::new(1.0);
        energy.spike(0.8);
        assert!((energy.level() - 0.8).abs() < f64::EPSILON);
        energy.spike(0.5);
        assert!((energy.level() - 1.3).abs() < f64::EPSILON);
    }

    #[test]
    fn tick_decays_level() {
        let mut energy = SoundEnergy::new(2.0);
        energy.spike(1.0);
        energy.tick(0.25);
        // 1.0 - 2.0 * 0.25 = 0.5
        assert!((energy.level() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn level_cannot_go_below_zero() {
        let mut energy = SoundEnergy::new(10.0);
        energy.spike(0.1);
        energy.tick(1.0);
        assert_eq!(energy.level(), 0.0);
    }

    #[test]
    fn band_specific_spikes() {
        let mut energy = SoundEnergy::new(1.0);
        energy.add_band(bass_range(), 2.0);
        energy.add_band(mid_range(), 1.5);

        energy.spike_band(0, 0.9);
        assert!((energy.band_level(0) - 0.9).abs() < f64::EPSILON);
        assert_eq!(energy.band_level(1), 0.0);

        // Out-of-range index is harmless.
        energy.spike_band(99, 1.0);
        assert_eq!(energy.band_count(), 2);
    }

    #[test]
    fn spike_frequency_hits_correct_bands() {
        let mut energy = SoundEnergy::new(1.0);
        energy.add_band(bass_range(), 2.0);   // 20–250 Hz, weight 1.0
        energy.add_band(mid_range(), 1.5);    // 250–4000 Hz, weight 0.5
        energy.add_band(high_range(), 1.0);   // 4000–20000 Hz, weight 0.25

        // 100 Hz falls only in bass.
        energy.spike_frequency(100.0, 1.0);
        assert!((energy.band_level(0) - 1.0).abs() < f64::EPSILON);
        assert_eq!(energy.band_level(1), 0.0);
        assert_eq!(energy.band_level(2), 0.0);

        // 250 Hz sits on the boundary of bass AND mid.
        energy.clear();
        energy.spike_frequency(250.0, 1.0);
        assert!((energy.band_level(0) - 1.0).abs() < f64::EPSILON);   // weight 1.0
        assert!((energy.band_level(1) - 0.5).abs() < f64::EPSILON);   // weight 0.5

        // 5000 Hz hits only high.
        energy.clear();
        energy.spike_frequency(5000.0, 1.0);
        assert_eq!(energy.band_level(0), 0.0);
        assert_eq!(energy.band_level(1), 0.0);
        assert!((energy.band_level(2) - 0.25).abs() < f64::EPSILON);  // weight 0.25
    }

    #[test]
    fn tick_decays_bands_independently() {
        let mut energy = SoundEnergy::new(1.0);
        energy.add_band(bass_range(), 4.0);
        energy.add_band(mid_range(), 2.0);

        energy.spike_band(0, 1.0);
        energy.spike_band(1, 1.0);
        energy.tick(0.25);

        // band 0: 1.0 - 4.0 * 0.25 = 0.0
        assert_eq!(energy.band_level(0), 0.0);
        // band 1: 1.0 - 2.0 * 0.25 = 0.5
        assert!((energy.band_level(1) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn clear_resets_everything() {
        let mut energy = SoundEnergy::new(1.0);
        energy.add_band(bass_range(), 2.0);
        energy.add_band(mid_range(), 1.5);

        energy.spike(0.7);
        energy.spike_band(0, 0.9);
        energy.spike_band(1, 0.4);

        energy.clear();

        assert_eq!(energy.level(), 0.0);
        assert_eq!(energy.band_level(0), 0.0);
        assert_eq!(energy.band_level(1), 0.0);
    }

    #[test]
    fn band_level_returns_zero_for_out_of_range() {
        let energy = SoundEnergy::new(1.0);
        assert_eq!(energy.band_level(0), 0.0);
        assert_eq!(energy.band_level(999), 0.0);
    }
}
