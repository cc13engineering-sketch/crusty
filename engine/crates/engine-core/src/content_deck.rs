/// Content Deck — Experimental content database with difficulty gating.
///
/// **STATUS: EXPERIMENTAL** — This module's API may change significantly
/// as more games adopt it. The core concept (content entries with tags,
/// difficulty gates, and selection strategies) is stable, but the specific
/// API surface is subject to iteration.
///
/// Manages a pool of content entries that can be filtered by difficulty,
/// tags, and selection strategy. Separate from SRS — SRS schedules reviews,
/// ContentDeck manages the content pool.

use crate::rng::SeededRng;

// ---------------------------------------------------------------------------
// ContentEntry
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ContentEntry {
    pub id: String,
    /// Minimum difficulty level required to unlock this content (0.0 = always available)
    pub difficulty_gate: f64,
    /// Categorization tags (e.g., "chord", "interval", "advanced")
    pub tags: Vec<String>,
    /// Arbitrary data payload as JSON string. Games interpret this.
    pub data: String,
}

// ---------------------------------------------------------------------------
// SelectionStrategy
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum SelectionStrategy {
    /// Select the entry that is most overdue for review (needs SRS data externally)
    MostOverdue,
    /// Random selection weighted by inverse frequency of past selections
    RandomWeighted,
    /// Pure random from available pool
    Random,
    /// Sequential (in order of insertion)
    Sequential,
}

// ---------------------------------------------------------------------------
// ContentDeck
// ---------------------------------------------------------------------------

pub struct ContentDeck {
    entries: Vec<ContentEntry>,
    /// Track selection counts for weighted random
    selection_counts: Vec<u32>,
    /// Current index for Sequential strategy
    current_index: usize,
}

impl ContentDeck {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            selection_counts: Vec::new(),
            current_index: 0,
        }
    }

    /// Add a content entry to the deck.
    pub fn add(&mut self, entry: ContentEntry) {
        self.entries.push(entry);
        self.selection_counts.push(0);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Find an entry by ID.
    pub fn get(&self, id: &str) -> Option<&ContentEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Return entries where `difficulty_gate <= difficulty`.
    pub fn available(&self, difficulty: f64) -> Vec<&ContentEntry> {
        self.entries
            .iter()
            .filter(|e| e.difficulty_gate <= difficulty)
            .collect()
    }

    /// Return entries where `difficulty_gate <= difficulty` AND the entry has the given tag.
    pub fn available_with_tag(&self, difficulty: f64, tag: &str) -> Vec<&ContentEntry> {
        self.entries
            .iter()
            .filter(|e| e.difficulty_gate <= difficulty && e.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Select one entry using the given strategy. Returns `None` if nothing is available.
    ///
    /// Uses `SeededRng` for deterministic random selection.
    pub fn select(
        &mut self,
        difficulty: f64,
        strategy: &SelectionStrategy,
        seed: u64,
    ) -> Option<&ContentEntry> {
        // Collect indices of available entries
        let available_indices: Vec<usize> = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.difficulty_gate <= difficulty)
            .map(|(i, _)| i)
            .collect();

        if available_indices.is_empty() {
            return None;
        }

        let chosen_idx = match strategy {
            SelectionStrategy::Random => {
                let mut rng = SeededRng::new(seed);
                let pick = rng.next_u64() % available_indices.len() as u64;
                available_indices[pick as usize]
            }

            SelectionStrategy::RandomWeighted => {
                // Weight = 1 / (1 + selection_count). Less-selected entries are more likely.
                let weights: Vec<f64> = available_indices
                    .iter()
                    .map(|&i| 1.0 / (1.0 + self.selection_counts[i] as f64))
                    .collect();
                let total: f64 = weights.iter().sum();

                let mut rng = SeededRng::new(seed);
                let threshold = rng.next_f64() * total;

                let mut cumulative = 0.0;
                let mut chosen = available_indices[0];
                for (wi, &idx) in available_indices.iter().enumerate() {
                    cumulative += weights[wi];
                    if cumulative >= threshold {
                        chosen = idx;
                        break;
                    }
                }
                chosen
            }

            SelectionStrategy::Sequential => {
                // Walk through available entries in insertion order, wrapping around.
                // Find the first available index >= current_index, or wrap.
                let picked = available_indices
                    .iter()
                    .find(|&&i| i >= self.current_index)
                    .copied()
                    .unwrap_or(available_indices[0]);
                self.current_index = picked + 1;
                picked
            }

            SelectionStrategy::MostOverdue => {
                // Without external SRS data, fall back to least-selected entry.
                let mut best_idx = available_indices[0];
                let mut best_count = self.selection_counts[best_idx];
                for &idx in &available_indices[1..] {
                    if self.selection_counts[idx] < best_count {
                        best_count = self.selection_counts[idx];
                        best_idx = idx;
                    }
                }
                best_idx
            }
        };

        Some(&self.entries[chosen_idx])
    }

    /// Increment the selection count for the entry with the given ID.
    pub fn record_selection(&mut self, id: &str) {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.selection_counts[pos] += 1;
        }
    }

    /// Reset everything — clear all entries, counts, and the sequential index.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.selection_counts.clear();
        self.current_index = 0;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry(id: &str, gate: f64, tags: &[&str], data: &str) -> ContentEntry {
        ContentEntry {
            id: id.to_string(),
            difficulty_gate: gate,
            tags: tags.iter().map(|t| t.to_string()).collect(),
            data: data.to_string(),
        }
    }

    #[test]
    fn empty_deck_returns_none_on_select() {
        let mut deck = ContentDeck::new();
        assert!(deck.is_empty());
        let result = deck.select(1.0, &SelectionStrategy::Random, 42);
        assert!(result.is_none());
    }

    #[test]
    fn add_entries_and_retrieve_by_id() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("a", 0.0, &["chord"], r#"{"note":"C"}"#));
        deck.add(sample_entry("b", 0.0, &["interval"], r#"{"interval":"P5"}"#));

        assert_eq!(deck.len(), 2);

        let a = deck.get("a");
        assert!(a.is_some());
        assert_eq!(a.map(|e| e.id.as_str()), Some("a"));

        let b = deck.get("b");
        assert!(b.is_some());
        assert_eq!(b.map(|e| e.id.as_str()), Some("b"));

        assert!(deck.get("nonexistent").is_none());
    }

    #[test]
    fn difficulty_gating_filters_correctly() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("easy", 0.0, &[], ""));
        deck.add(sample_entry("medium", 0.5, &[], ""));
        deck.add(sample_entry("hard", 1.0, &[], ""));

        let avail_0 = deck.available(0.0);
        assert_eq!(avail_0.len(), 1);
        assert_eq!(avail_0[0].id, "easy");

        let avail_half = deck.available(0.5);
        assert_eq!(avail_half.len(), 2);

        let avail_all = deck.available(1.0);
        assert_eq!(avail_all.len(), 3);
    }

    #[test]
    fn tag_filtering_works() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("c1", 0.0, &["chord"], ""));
        deck.add(sample_entry("i1", 0.0, &["interval"], ""));
        deck.add(sample_entry("c2", 0.5, &["chord", "advanced"], ""));

        let chords = deck.available_with_tag(1.0, "chord");
        assert_eq!(chords.len(), 2);
        assert!(chords.iter().all(|e| e.tags.contains(&"chord".to_string())));

        let intervals = deck.available_with_tag(1.0, "interval");
        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].id, "i1");

        // Tag + difficulty gate combined
        let chords_low = deck.available_with_tag(0.3, "chord");
        assert_eq!(chords_low.len(), 1);
        assert_eq!(chords_low[0].id, "c1");
    }

    #[test]
    fn random_strategy_returns_from_available_pool() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("a", 0.0, &[], ""));
        deck.add(sample_entry("b", 0.0, &[], ""));
        deck.add(sample_entry("c", 99.0, &[], "")); // gated out

        // Try several seeds — result should always be "a" or "b"
        for seed in 1..20 {
            let result = deck.select(0.5, &SelectionStrategy::Random, seed);
            assert!(result.is_some());
            let id = result.map(|e| e.id.as_str());
            assert!(id == Some("a") || id == Some("b"));
        }
    }

    #[test]
    fn sequential_strategy_advances_through_entries() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("first", 0.0, &[], ""));
        deck.add(sample_entry("second", 0.0, &[], ""));
        deck.add(sample_entry("third", 0.0, &[], ""));

        let r1 = deck.select(1.0, &SelectionStrategy::Sequential, 0);
        assert_eq!(r1.map(|e| e.id.as_str()), Some("first"));

        let r2 = deck.select(1.0, &SelectionStrategy::Sequential, 0);
        assert_eq!(r2.map(|e| e.id.as_str()), Some("second"));

        let r3 = deck.select(1.0, &SelectionStrategy::Sequential, 0);
        assert_eq!(r3.map(|e| e.id.as_str()), Some("third"));

        // Wraps around
        let r4 = deck.select(1.0, &SelectionStrategy::Sequential, 0);
        assert_eq!(r4.map(|e| e.id.as_str()), Some("first"));
    }

    #[test]
    fn record_selection_increments_counts() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("x", 0.0, &[], ""));
        deck.add(sample_entry("y", 0.0, &[], ""));

        deck.record_selection("x");
        deck.record_selection("x");
        deck.record_selection("y");

        // Internal counts: x=2, y=1. Verify indirectly via MostOverdue picking y.
        let result = deck.select(1.0, &SelectionStrategy::MostOverdue, 0);
        assert_eq!(result.map(|e| e.id.as_str()), Some("y"));
    }

    #[test]
    fn available_returns_all_when_difficulty_is_high() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("a", 0.0, &[], ""));
        deck.add(sample_entry("b", 0.3, &[], ""));
        deck.add(sample_entry("c", 0.7, &[], ""));
        deck.add(sample_entry("d", 1.0, &[], ""));

        let all = deck.available(1000.0);
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn clear_resets_everything() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("a", 0.0, &[], ""));
        deck.record_selection("a");
        assert_eq!(deck.len(), 1);

        deck.clear();
        assert!(deck.is_empty());
        assert_eq!(deck.len(), 0);
        assert!(deck.get("a").is_none());
    }

    #[test]
    fn random_weighted_favors_less_selected() {
        let mut deck = ContentDeck::new();
        deck.add(sample_entry("frequent", 0.0, &[], ""));
        deck.add(sample_entry("rare", 0.0, &[], ""));

        // Heavily select "frequent"
        for _ in 0..100 {
            deck.record_selection("frequent");
        }

        // Over many seeds, "rare" should be picked more often than "frequent".
        // Use large seed values to avoid xorshift64 low-seed bias.
        let mut rare_count = 0;
        let mut frequent_count = 0;
        for i in 1u64..200 {
            let seed = i.wrapping_mul(6364136223846793005);
            if let Some(entry) = deck.select(1.0, &SelectionStrategy::RandomWeighted, seed) {
                if entry.id == "rare" {
                    rare_count += 1;
                } else {
                    frequent_count += 1;
                }
            }
        }

        assert!(
            rare_count > frequent_count,
            "rare={rare_count} should be > frequent={frequent_count}"
        );
    }
}
