/// SM-2 based Spaced Repetition System for music theory cards.
///
/// Each unique challenge variant (concept_type:variant_id) is a "card".
/// Cards track easiness factor, interval, and review schedule.
///
/// Card ID format: "{concept_idx}:{variant}" where:
///   - 0:0..0:6  = ScaleDegree (7 cards, one per degree)
///   - 1:0..1:6  = RomanNumeral (7 cards, one per degree)
///   - 2:0..2:12 = IntervalRecognition (13 cards, 0-12 semitones)
///   - 3:0..3:3  = ChordQuality (4 cards)
///   - 4:0..4:3  = Cadence (4 cards)

use std::collections::BTreeMap;

// ─── CardState ──────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct CardState {
    pub easiness: f64,          // SM-2 EF, starts at 2.5, min 1.3
    pub interval_days: f64,     // days until next review
    pub repetitions: u32,       // consecutive correct count
    pub next_review_day: f64,   // absolute day number for next review
    pub last_reviewed_day: f64, // when last reviewed
}

impl CardState {
    pub fn new() -> Self {
        Self {
            easiness: 2.5,
            interval_days: 0.0,
            repetitions: 0,
            next_review_day: 0.0,
            last_reviewed_day: 0.0,
        }
    }

    /// Is this card due for review on the given day?
    pub fn is_due(&self, day: f64) -> bool {
        day >= self.next_review_day
    }

    /// How overdue is this card (positive = overdue, negative = not yet due).
    pub fn overdue_amount(&self, day: f64) -> f64 {
        day - self.next_review_day
    }
}

// ─── SrsState ───────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SrsState {
    pub cards: BTreeMap<String, CardState>,
    pub current_day: f64,
    pub new_cards_per_day: u32,
    pub new_cards_today: u32,
    pub reviews_today: u32,
    pub day_of_last_reset: f64,
}

/// All possible card IDs in the deck.
const CARD_DEFS: &[(u8, u8)] = &[
    // ScaleDegree: 7 cards
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6),
    // RomanNumeral: 7 cards
    (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
    // IntervalRecognition: 13 cards
    (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6),
    (2, 7), (2, 8), (2, 9), (2, 10), (2, 11), (2, 12),
    // ChordQuality: 4 cards
    (3, 0), (3, 1), (3, 2), (3, 3),
    // Cadence: 4 cards
    (4, 0), (4, 1), (4, 2), (4, 3),
];

fn card_id(concept: u8, variant: u8) -> String {
    format!("{}:{}", concept, variant)
}

impl SrsState {
    pub fn new() -> Self {
        Self {
            cards: BTreeMap::new(),
            current_day: 0.0,
            new_cards_per_day: 10,
            new_cards_today: 0,
            reviews_today: 0,
            day_of_last_reset: 0.0,
        }
    }

    /// Reset daily counters if the day has changed.
    pub fn check_day_reset(&mut self, day: f64) {
        self.current_day = day;
        if day > self.day_of_last_reset + 0.5 {
            self.new_cards_today = 0;
            self.reviews_today = 0;
            self.day_of_last_reset = day;
        }
    }

    /// Select the next card to review. Returns (concept_idx, variant).
    ///
    /// Priority:
    /// 1. Most overdue existing card
    /// 2. New card (if under daily new-card limit)
    /// 3. Least-recently-reviewed card (if nothing else available)
    pub fn select_next_card(&self, rng_seed: u64) -> Option<(u8, u8)> {
        let day = self.current_day;

        // Find the most overdue card among known cards
        let mut best_overdue: Option<(&str, f64)> = None;
        for (id, card) in &self.cards {
            if card.is_due(day) {
                let overdue = card.overdue_amount(day);
                if best_overdue.is_none() || overdue > best_overdue.unwrap().1 {
                    best_overdue = Some((id.as_str(), overdue));
                }
            }
        }

        if let Some((id, _)) = best_overdue {
            return Self::parse_card_id(id);
        }

        // Try to introduce a new card
        if self.new_cards_today < self.new_cards_per_day {
            // Find cards not yet in the deck
            let mut unseen: Vec<(u8, u8)> = Vec::new();
            for &(concept, variant) in CARD_DEFS {
                let id = card_id(concept, variant);
                if !self.cards.contains_key(&id) {
                    unseen.push((concept, variant));
                }
            }
            if !unseen.is_empty() {
                // Use rng_seed to pick semi-randomly from unseen
                let idx = (rng_seed as usize) % unseen.len();
                return Some(unseen[idx]);
            }
        }

        // Everything reviewed and nothing due — pick least recently reviewed
        let mut oldest: Option<(&str, f64)> = None;
        for (id, card) in &self.cards {
            if oldest.is_none() || card.last_reviewed_day < oldest.unwrap().1 {
                oldest = Some((id.as_str(), card.last_reviewed_day));
            }
        }
        if let Some((id, _)) = oldest {
            return Self::parse_card_id(id);
        }

        // Completely empty deck — start with first card
        Some(CARD_DEFS[0])
    }

    /// Record a review result.
    /// quality: 1=fail, 2=hint-used, 4=good, 5=easy
    pub fn review_card(&mut self, concept: u8, variant: u8, quality: u8) {
        let id = card_id(concept, variant);
        let day = self.current_day;
        let is_new = !self.cards.contains_key(&id);

        let card = self.cards.entry(id).or_insert_with(CardState::new);
        card.last_reviewed_day = day;

        if is_new {
            self.new_cards_today += 1;
        }
        self.reviews_today += 1;

        // SM-2 algorithm
        let q = quality as f64;

        if quality < 3 {
            // Failed — reset repetitions, short interval
            card.repetitions = 0;
            card.interval_days = if quality == 2 { 0.5 } else { 0.1 }; // hint = half day, fail = retry soon
        } else {
            // Successful
            card.repetitions += 1;
            match card.repetitions {
                1 => card.interval_days = 1.0,
                2 => card.interval_days = 3.0,
                _ => card.interval_days = (card.interval_days * card.easiness).max(1.0),
            }
        }

        // Update easiness factor (SM-2 formula)
        let new_ef = card.easiness + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02));
        card.easiness = new_ef.max(1.3);

        // Schedule next review
        card.next_review_day = day + card.interval_days;
    }

    /// Count of cards currently due for review.
    pub fn due_count(&self) -> u32 {
        let day = self.current_day;
        self.cards.values().filter(|c| c.is_due(day)).count() as u32
    }

    /// Count of cards in "learning" phase (interval < 7 days).
    pub fn learning_count(&self) -> u32 {
        self.cards.values().filter(|c| c.interval_days < 7.0).count() as u32
    }

    /// Count of cards in "mature" phase (interval >= 7 days).
    pub fn mature_count(&self) -> u32 {
        self.cards.values().filter(|c| c.interval_days >= 7.0).count() as u32
    }

    /// Total cards seen (in the deck).
    pub fn total_seen(&self) -> u32 {
        self.cards.len() as u32
    }

    // ─── JSON Serialization ─────────────────────────────────

    pub fn to_json(&self) -> String {
        let mut json = String::from("{\"cards\":{");
        for (i, (id, card)) in self.cards.iter().enumerate() {
            if i > 0 { json.push(','); }
            json.push_str(&format!(
                "\"{}\":{{\"e\":{:.4},\"i\":{:.4},\"r\":{},\"n\":{:.4},\"l\":{:.4}}}",
                id, card.easiness, card.interval_days, card.repetitions,
                card.next_review_day, card.last_reviewed_day
            ));
        }
        json.push_str(&format!(
            "}},\"day\":{:.4},\"npd\":{},\"nct\":{},\"rt\":{},\"dlr\":{:.4}}}",
            self.current_day, self.new_cards_per_day, self.new_cards_today,
            self.reviews_today, self.day_of_last_reset
        ));
        json
    }

    pub fn from_json(json: &str) -> Option<Self> {
        // Minimal JSON parser for our specific format
        let val: serde_json::Value = serde_json::from_str(json).ok()?;
        let obj = val.as_object()?;

        let mut state = SrsState::new();
        state.current_day = obj.get("day")?.as_f64()?;
        state.new_cards_per_day = obj.get("npd").and_then(|v| v.as_u64()).unwrap_or(10) as u32;
        state.new_cards_today = obj.get("nct").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        state.reviews_today = obj.get("rt").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        state.day_of_last_reset = obj.get("dlr").and_then(|v| v.as_f64()).unwrap_or(0.0);

        if let Some(cards_obj) = obj.get("cards").and_then(|v| v.as_object()) {
            for (id, card_val) in cards_obj {
                if let Some(co) = card_val.as_object() {
                    let card = CardState {
                        easiness: co.get("e").and_then(|v| v.as_f64()).unwrap_or(2.5),
                        interval_days: co.get("i").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        repetitions: co.get("r").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                        next_review_day: co.get("n").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        last_reviewed_day: co.get("l").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    };
                    state.cards.insert(id.clone(), card);
                }
            }
        }

        Some(state)
    }

    fn parse_card_id(id: &str) -> Option<(u8, u8)> {
        let parts: Vec<&str> = id.split(':').collect();
        if parts.len() != 2 { return None; }
        let concept = parts[0].parse::<u8>().ok()?;
        let variant = parts[1].parse::<u8>().ok()?;
        Some((concept, variant))
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_srs_state_is_empty() {
        let srs = SrsState::new();
        assert_eq!(srs.cards.len(), 0);
        assert_eq!(srs.due_count(), 0);
    }

    #[test]
    fn select_first_card_from_empty_deck() {
        let srs = SrsState::new();
        let card = srs.select_next_card(42);
        assert!(card.is_some());
    }

    #[test]
    fn review_creates_card() {
        let mut srs = SrsState::new();
        srs.review_card(0, 3, 4); // good quality
        assert_eq!(srs.cards.len(), 1);
        let id = card_id(0, 3);
        let card = srs.cards.get(&id).unwrap();
        assert_eq!(card.repetitions, 1);
        assert!(card.interval_days >= 1.0);
    }

    #[test]
    fn fail_resets_repetitions() {
        let mut srs = SrsState::new();
        srs.review_card(0, 0, 4); // good
        srs.review_card(0, 0, 4); // good
        assert_eq!(srs.cards.get(&card_id(0, 0)).unwrap().repetitions, 2);
        srs.review_card(0, 0, 1); // fail
        assert_eq!(srs.cards.get(&card_id(0, 0)).unwrap().repetitions, 0);
    }

    #[test]
    fn hint_quality_gives_short_interval() {
        let mut srs = SrsState::new();
        srs.review_card(2, 7, 2); // hint used
        let card = srs.cards.get(&card_id(2, 7)).unwrap();
        assert_eq!(card.repetitions, 0);
        assert!((card.interval_days - 0.5).abs() < 0.01);
    }

    #[test]
    fn easiness_decreases_on_fail() {
        let mut srs = SrsState::new();
        srs.review_card(0, 0, 1);
        let ef = srs.cards.get(&card_id(0, 0)).unwrap().easiness;
        assert!(ef < 2.5);
        assert!(ef >= 1.3);
    }

    #[test]
    fn easiness_increases_on_easy() {
        let mut srs = SrsState::new();
        srs.review_card(0, 0, 5);
        let ef = srs.cards.get(&card_id(0, 0)).unwrap().easiness;
        assert!(ef > 2.5);
    }

    #[test]
    fn due_count_tracks_reviews() {
        let mut srs = SrsState::new();
        srs.current_day = 0.0;
        srs.review_card(0, 0, 4); // interval 1 day
        srs.review_card(0, 1, 4); // interval 1 day

        // On day 0, nothing due yet (just reviewed)
        assert_eq!(srs.due_count(), 0);

        // On day 2, both should be due
        srs.current_day = 2.0;
        assert_eq!(srs.due_count(), 2);
    }

    #[test]
    fn select_returns_overdue_card() {
        let mut srs = SrsState::new();
        srs.current_day = 0.0;
        srs.review_card(0, 0, 4);
        srs.current_day = 5.0; // well past interval
        let card = srs.select_next_card(42);
        assert_eq!(card, Some((0, 0)));
    }

    #[test]
    fn json_round_trip() {
        let mut srs = SrsState::new();
        srs.current_day = 3.5;
        srs.review_card(0, 0, 4);
        srs.review_card(2, 7, 5);
        srs.review_card(3, 1, 1);

        let json = srs.to_json();
        let restored = SrsState::from_json(&json).expect("should parse");
        assert_eq!(restored.cards.len(), 3);
        assert!((restored.current_day - 3.5).abs() < 0.01);

        let c = restored.cards.get(&card_id(0, 0)).unwrap();
        assert_eq!(c.repetitions, 1);

        let c2 = restored.cards.get(&card_id(3, 1)).unwrap();
        assert_eq!(c2.repetitions, 0); // failed
    }

    #[test]
    fn day_reset_clears_counters() {
        let mut srs = SrsState::new();
        srs.current_day = 0.0;
        srs.review_card(0, 0, 4);
        assert_eq!(srs.new_cards_today, 1);

        srs.check_day_reset(1.0);
        assert_eq!(srs.new_cards_today, 0);
        assert_eq!(srs.reviews_today, 0);
    }

    #[test]
    fn new_card_limit() {
        let mut srs = SrsState::new();
        srs.new_cards_per_day = 2;
        srs.new_cards_today = 2; // already at limit

        // Should still return a card (fallback to first card def)
        let card = srs.select_next_card(42);
        assert!(card.is_some());
    }

    #[test]
    fn learning_vs_mature_count() {
        let mut srs = SrsState::new();
        // Card with short interval = learning
        srs.cards.insert(card_id(0, 0), CardState {
            easiness: 2.5,
            interval_days: 1.0,
            repetitions: 1,
            next_review_day: 1.0,
            last_reviewed_day: 0.0,
        });
        // Card with long interval = mature
        srs.cards.insert(card_id(0, 1), CardState {
            easiness: 2.5,
            interval_days: 14.0,
            repetitions: 5,
            next_review_day: 14.0,
            last_reviewed_day: 0.0,
        });
        assert_eq!(srs.learning_count(), 1);
        assert_eq!(srs.mature_count(), 1);
    }

    #[test]
    fn parse_card_id_works() {
        assert_eq!(SrsState::parse_card_id("0:3"), Some((0, 3)));
        assert_eq!(SrsState::parse_card_id("2:12"), Some((2, 12)));
        assert_eq!(SrsState::parse_card_id("bad"), None);
    }

    #[test]
    fn all_card_defs_valid() {
        for &(concept, variant) in CARD_DEFS {
            assert!(concept <= 4);
            match concept {
                0 | 1 => assert!(variant <= 6),
                2 => assert!(variant <= 12),
                3 | 4 => assert!(variant <= 3),
                _ => panic!("unknown concept"),
            }
        }
        assert_eq!(CARD_DEFS.len(), 35);
    }
}
