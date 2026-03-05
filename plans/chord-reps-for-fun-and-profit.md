# Music Theory for Fun and Profit

**Monetizing an Anki-style music theory / ear training app via affiliate links**

---

## The Core Idea

Show contextual affiliate product recommendations on "success" screens after users complete challenges. The insight/fun-fact content users already want to read becomes the vehicle for genuinely useful product recommendations.

**Why this could work:** Users are in a positive emotional state after succeeding, they're actively engaged with the content, and the recommendations can be hyper-specific to what they just practiced.

**Why you need to be realistic:** Affiliate revenue from a niche app is supplementary income, not a business model. The numbers are sobering (see Section 5). Design for the learning experience first; affiliate links are gravy.

---

## 1. How It Works: The Success Screen Model

```
+------------------------------------------+
|  CORRECT! You identified: Tritone         |
|                                           |
|  FUN FACT: This interval was called       |
|  "diabolus in musica" and banned in       |
|  medieval church music. It's now the      |
|  backbone of dominant 7th chords and      |
|  the entire blues tradition.              |
|                                           |
|  Go deeper:                               |
|  "The Jazz Harmony Book" - $24            |
|  (affiliate link)                         |
|                                           |
|  [Next Challenge]                         |
+------------------------------------------+
```

### Design Rules

1. **One recommendation max per success screen** -- more kills trust
2. **Show recommendations sparingly** -- maybe 1 in 4-5 successes, not every time
3. **The fun fact must stand alone** -- it should be interesting even without the product link
4. **Never block the core flow** -- the "Next Challenge" button is always prominent
5. **Stage-appropriate** -- don't recommend advanced textbooks to beginners
6. **FTC-compliant disclosure** -- "(affiliate link)" or similar, visible before the click
7. **Only recommend products you'd genuinely suggest to a student**

---

## 2. Challenge-to-Product Mapping

The specificity of the mapping is everything. Generic = spam. Specific = helpful.

### Tier 1: Highest-Converting Mappings (most natural)

| Challenge Type | Product Category | Example | Why It Works |
|---|---|---|---|
| **Rhythm exercises** | Metronome / practice pad | Korg TM-60, Vic Firth practice pad | 1:1 tool-to-skill match |
| **Timbre recognition** | Beginner instruments | Specific to identified instrument | Personalized by what they just heard |
| **Chord progressions** | Songwriting / harmony books | Hooktheory I & II, The Real Book | Direct creative next step |
| **Interval / chord ID** | MIDI keyboard controller | Akai MPK Mini, Arturia MiniLab | Bridges ear training to playing |

### Tier 2: Strong Mappings

| Challenge Type | Product Category | Example | Why It Works |
|---|---|---|---|
| **Scale identification** | Genre-specific courses / instruments | Jazz improv course (for Dorian), Blues guitar book (for pentatonic) | Scales map to genres and cultures |
| **Sight-reading** | Graded sight-reading books / sheet music | Paul Harris series, SheetMusicPlus | They're actively building this skill |
| **Chord identification** | Songbooks / theory books | Beatles Complete Chord Songbook | Apply what they're learning |

### Tier 3: Weaker Mappings (lean on fun facts, light on products)

| Challenge Type | Product Category | Notes |
|---|---|---|
| **Key signatures** | Theory workbooks, manuscript paper | Academic skill -- product links feel forced. Better to invest in great fun facts here and skip affiliate links most of the time. |

### Sample Fun Fact + Affiliate Pairings

**Interval: Perfect 5th**
> The perfect 5th is so fundamental that it's the basis of Pythagorean tuning -- one of the oldest tuning systems, derived from the ratios of vibrating strings. Pythagoras literally discovered music theory with a monochord.
> *Explore intervals hands-on: [MIDI Keyboard Controller] (affiliate link)*

**Chord progression: I-V-vi-IV**
> You just identified the most common progression in pop music. It's in *Let It Be*, *No Woman No Cry*, *With or Without You*, *Someone Like You*, and hundreds more. The Axis of Awesome famously performed 47 songs using only this progression.
> *See the patterns: [Hooktheory Book I] (affiliate link)*

**Rhythm: Polyrhythm**
> The 3-against-2 polyrhythm you just identified is foundational in West African drumming and is the rhythmic DNA of Afro-Cuban music. Dave Brubeck brought this into jazz. Bulgarian dancers handle 11/8 like it's nothing.
> *Feel the rhythm: [Practice Pad Kit] (affiliate link)*

**Timbre: Oboe identified**
> The oboe tunes the entire orchestra -- the principal oboist plays an A440 and everyone matches it. Its pitch is extremely stable and its tone cuts through the ensemble.
> *Curious about the oboe? [Beginner Oboe Guide] (affiliate link)*

---

## 3. Affiliate Programs Worth Pursuing

### Tier 1: Set Up Immediately

| Program | Commission | Cookie | Why |
|---|---|---|---|
| **Amazon Associates** | 4.5-7% by category | 24 hours | Covers everything (books, instruments, gear). Conversion rate compensates for short cookie. |
| **Coursera** | 15-45% | 30 days | Berklee music theory courses = near-perfect audience match |
| **Skillshare** | 40% of first payment | 30 days | Generous commission, broad music course catalog |
| **Sheet Music Plus** | 8-12% (tiered) | 30 days | Direct relevance, good commission. Hidden gem for this niche. |

### Tier 2: Set Up When Ready

| Program | Commission | Cookie | Why |
|---|---|---|---|
| **MasterClass** | 25% | 30 days | Herbie Hancock, deadmau5 etc. Strong brand. |
| **Sweetwater** | 5-8% | 14-30 days | Best instrument retailer affiliate. Requires direct outreach. |
| **Fender Play** | $5-12/signup | 7 days | Good if audience includes guitarists |

### Tier 3: Situational

| Program | Commission | Notes |
|---|---|---|
| **Guitar Center** | 4-8% | Fallback instrument retailer |
| **Splice** | $11.20/paid signup | Only if guiding users toward production |
| **Plugin Boutique** | Up to 15% | Only if recommending specific software |
| **Apple Performance Partners** | 7% | Catch-all for Apple ecosystem (Logic Pro, Apple Books) |

### Skip

- **Spotify** -- most users already subscribe
- **Ableton** -- no affiliate program exists
- **FL Studio** -- 3-7%, not worth the effort
- **Yousician** -- no public program

---

## 4. Implementation Plan

### Phase 1: Content First (before any affiliate links)

Build a curated database of ~50-80 fun facts mapped to challenge types and specific items (intervals, chords, scales, etc.). These should be genuinely interesting standalone content. Test them with users. This is the product -- the affiliate links are just decoration on top.

**Data structure:**
```
{
  challenge_type: "interval",
  specific_item: "tritone",
  user_level: "beginner",
  fun_fact: "This interval was called 'diabolus in musica'...",
  product: { name: "Jazz Harmony Book", price: "$24", affiliate_url: "...", program: "amazon" },
  show_frequency: 0.25  // show product 25% of the time
}
```

### Phase 2: Affiliate Setup

1. Sign up for Amazon Associates (covers books, instruments, everything)
2. Apply to Coursera affiliate program (best commission/relevance ratio)
3. Apply to Skillshare (generous 40%)
4. Apply to Sheet Music Plus (8-12%, very relevant)
5. Set up proper FTC disclosure language

### Phase 3: Integration

- Add affiliate content to success screens with frequency controls
- Track click-through rates per challenge type and product category
- A/B test: fun-fact-only vs. fun-fact-with-product
- Measure whether affiliate presence affects user retention (critical -- if users bounce more when they see affiliate links, pull back)

### Phase 4: Iterate

- Double down on what converts
- Kill what doesn't
- Expand fun fact database based on engagement data
- Add milestone/achievement affiliate placements (only after the core success-screen model is validated)

---

## 5. The Brutal Math

### Revenue at Different Traffic Levels

| MAU | See affiliate | Click (1.5% CTR) | Buy (2%) | Revenue/mo |
|---|---|---|---|---|
| 1,000 | 400 | 6 | 0.12 | ~$1.50 |
| 10,000 | 4,000 | 60 | 1.2 | ~$17 |
| 50,000 | 20,000 | 300 | 6 | ~$84 |
| 100,000 | 40,000 | 600 | 12 | ~$168 |

*Assumes 40% see a placement, 1.5% CTR, 2% purchase conversion, $14 avg commission.*

**Education platform commissions change the math significantly.** A Coursera signup commission ($20-50) or Skillshare first-payment commission (~$27) is worth far more per conversion than a $14 Amazon instrument commission. If even 2-3 users/month sign up for Coursera through your link at 50K MAU, that's $50-150/month from that program alone.

### Blended Estimate (50K MAU, mature implementation)

| Source | Est. Monthly |
|---|---|
| Amazon Associates (books, gear) | $30-60 |
| Course platforms (Coursera, Skillshare, MasterClass) | $50-200 |
| Sheet Music Plus | $10-30 |
| Instrument retailers (Sweetwater) | $20-50 |
| **Total** | **$110-340/month** |

### Honest Assessment

- **Under 10K MAU:** Essentially zero revenue. Not worth optimizing for.
- **10K-50K MAU:** Beer money. $20-100/month. Worth having but not worth compromising the UX for.
- **50K-100K MAU:** Meaningful supplement. $100-400/month. Worth investing in good content.
- **100K+ MAU:** Potentially $300-1000+/month. Now it's real money.

**Bottom line:** Affiliate revenue alone won't sustain the project until you have significant traffic. But the cost of implementing it is near-zero if you're already building fun-fact content (which you should be, because it makes the app better).

---

## 6. Alternative / Complementary Revenue Streams

Affiliate links should be **one layer** in a stack. Ranked by realistic return:

### 1. Freemium / Subscription ($5-15/month)
The proven model for music ed apps (Yousician, Simply Piano, Flowkey). Free tier with daily limits or basic content; paid tier unlocks everything. Even a 3-5% conversion rate with a few thousand users beats affiliate revenue.

### 2. One-Time "Tip Jar" / Unlock Fee ($3-10)
Lower ceiling but zero ongoing friction. Works well for indie apps. Musictheory.net's Tenuto app uses this model.

### 3. Premium Content Packs
Sell specialized challenge sets: jazz ear training pack, production ear training pack, advanced rhythm pack. Digital goods with zero marginal cost.

### 4. Classroom / Institutional Licensing ($50-200/year)
If the app is useful for music teachers, sell classroom licenses. Schools have budgets for educational software.

### 5. Affiliate Links (this plan)
Supplementary. Low effort to implement. Enhances the product when done right.

### 6. Direct Sponsorship
Once you have an audience, a single partnership deal with Sweetwater or Fender could generate more than all affiliate links combined. But you need the audience first.

---

## 7. Legal Requirements

### FTC Disclosure (Required)

- Disclosure must appear **where the recommendation appears**, not buried in a footer
- Must be visible **before** the user clicks
- Plain language: "(affiliate link)" or "We earn a commission if you purchase"
- Amazon specifically requires: "As an Amazon Associate I earn from qualifying purchases"

### Template

```
[Fun fact content]

Recommended: [Product Name] - $XX
(affiliate link -- we earn a small commission if you purchase)
```

### EU / GDPR (If Applicable)

Affiliate tracking cookies require consent under GDPR. If you have EU traffic, you need a cookie consent mechanism.

---

## 8. Feedback and Realism Check

### What's genuinely promising about this approach

- **The fun-fact wrapper is the real product.** Even if zero affiliate links convert, the educational content makes the app better. This means the implementation cost is aligned with product quality, not just monetization.
- **The mappings are specific and natural.** Recommending a metronome after a rhythm exercise or an instrument after timbre recognition doesn't feel like advertising -- it feels like mentoring.
- **Low implementation cost.** It's just a content database and a display component. No complex infrastructure.

### What you should be skeptical about

- **Revenue won't be meaningful at small scale.** Below 10K MAU, you'll earn less than minimum wage for the time spent curating content. The fun facts need to justify themselves as product features, not revenue generators.
- **Conversion rates may be lower than benchmarks.** These are web app users in a learning flow, not people browsing a review site with purchase intent. Your CTR might be 0.5%, not 1.5%.
- **Amazon's 24-hour cookie is brutal.** Users might think "cool, I should get a MIDI keyboard" but not buy until next week. You get nothing.
- **Affiliate programs change terms constantly.** Amazon has cut rates multiple times. Don't build revenue projections on current rates.
- **User trust is fragile.** One pushy recommendation can undo ten good ones. Err on the side of fewer, better recommendations.

### The real play

The affiliate links are a **trojan horse for building a content database** that makes the app genuinely more engaging. The revenue is a bonus. If you're choosing between spending an hour on affiliate optimization vs. an hour on making the app more fun, always choose the app.

If the app gets traction (50K+ MAU), the content database you've built for affiliate purposes becomes the foundation for:
- A premium content tier (the fun facts themselves become a selling point)
- Direct sponsorship deals ("our ear training app is sponsored by Sweetwater")
- A mailing list/community that has much higher conversion potential than in-app links

**Think of the affiliate implementation as planting seeds for multiple future revenue streams, not as a revenue stream itself.**

---

## 9. Implementation Architecture (Code-Level)

This section maps the monetization plan onto the actual codebase so that adding new challenges, fun facts, and affiliate links is a **data-only task** — no Rust code changes needed once the framework is in place.

### 9.1 Current Architecture (What We're Building On)

The music theory app lives in 4 files:

```
engine/crates/engine-core/src/music_theory/
  mod.rs      — MusicTheorySim (state, input, rendering)    ~1400 lines
  theory.rs   — Pure lookup functions (insights, intervals)  ~410 lines
  srs.rs      — SM-2 spaced repetition                      ~455 lines
  persist.rs  — localStorage command queue                   ~110 lines
```

**Current insight flow:**
1. User answers correctly → `on_correct()` (mod.rs:713)
2. Match on `self.challenge.concept` → call e.g. `degree_insight(deg)` (theory.rs)
3. Store result in `self.current_insight: String`
4. `render_feedback()` (mod.rs:1861) wraps and draws `current_insight` in the OPTIONS_Y..PIANO_Y zone

**Current challenge identification:**
- `concept_idx()` returns 0–4 (ScaleDegree, RomanNumeral, Interval, ChordQuality, Cadence)
- `self.challenge.answer` is the variant (degree 0–6, semitones 0–12, quality 0–3, cadence 0–3)
- `self.difficulty` is 1–10, advances every 5 correct answers

**Current insight functions** in theory.rs are `match` statements returning `&'static str`:
- `degree_insight(degree: u8)` — 7 variants
- `numeral_insight(degree: u8)` — 7 variants
- `interval_insight(semitones: u8)` — 13 variants
- `quality_insight(q: ChordQuality)` — 4 variants
- `cadence_insight(c: CadenceType)` — 4 variants

Total: **35 static insight strings** (one per SRS card).

### 9.2 Target Design: Content as Data

**Goal:** Adding a new fun fact or affiliate link = adding a struct literal to an array in theory.rs. No changes to mod.rs, persist.rs, or rendering code.

#### New Types (add to theory.rs)

```rust
/// A product recommendation shown alongside a fun fact.
pub struct ProductRec {
    pub name: &'static str,
    pub blurb: &'static str,      // e.g. "Explore intervals hands-on"
    pub url: &'static str,         // affiliate URL
    pub program: &'static str,     // "amazon", "coursera", etc.
}

/// A fun fact + optional product for a specific challenge variant.
pub struct ContentEntry {
    pub concept: u8,               // 0–4
    pub variant: Option<u8>,       // None = any variant of this concept
    pub min_difficulty: u8,         // 0 = show always, 5 = intermediate+
    pub fact: &'static str,        // standalone fun fact text
    pub product: Option<ProductRec>,
}
```

All `&'static str` — zero allocation, compiled into the binary. Same pattern as existing `DEGREE_NAMES`, `INTERVAL_NAMES`, etc.

#### Content Database (add to theory.rs)

```rust
/// All fun facts and product recommendations.
/// Adding content = adding entries here. Nothing else to change.
pub const CONTENT_DB: &[ContentEntry] = &[
    // ── Intervals ──────────────────────────────────────────
    ContentEntry {
        concept: 2, variant: Some(6), min_difficulty: 0,
        fact: "The tritone was called 'diabolus in musica' and banned in \
               medieval church music. It's now the backbone of dominant 7th \
               chords and the entire blues tradition.",
        product: Some(ProductRec {
            name: "The Jazz Harmony Book",
            blurb: "Go deeper into jazz harmony",
            url: "https://www.amazon.com/dp/XXXXXXXXXX?tag=yourtag-20",
            program: "amazon",
        }),
    },
    ContentEntry {
        concept: 2, variant: Some(7), min_difficulty: 0,
        fact: "The perfect 5th is so fundamental it's the basis of \
               Pythagorean tuning — one of the oldest tuning systems, \
               derived from the ratios of vibrating strings.",
        product: Some(ProductRec {
            name: "Akai MPK Mini MIDI Keyboard",
            blurb: "Explore intervals hands-on",
            url: "https://www.amazon.com/dp/XXXXXXXXXX?tag=yourtag-20",
            program: "amazon",
        }),
    },
    // ── Cadences ───────────────────────────────────────────
    ContentEntry {
        concept: 4, variant: Some(3), min_difficulty: 0,
        fact: "V to vi — the great harmonic bait-and-switch. Your ear \
               expects the tonic but lands on its relative minor. Beethoven \
               and Radiohead both use deceptive cadences to subvert expectations.",
        product: None, // Not every entry needs a product
    },
    // ... 50-100 more entries ...
];
```

**To add content later:** Just append a `ContentEntry` literal. The compiler checks types. No other files touched.

#### Lookup Function (add to theory.rs)

```rust
/// Select a content entry for the given challenge result.
/// Returns the best matching entry, preferring variant-specific over generic.
/// `seed` is used for deterministic random selection when multiple entries match.
pub fn select_content(
    concept: u8,
    variant: u8,
    difficulty: u8,
    seed: u64,
) -> Option<&'static ContentEntry> {
    // Collect all entries that match concept + variant + difficulty
    let mut specific: Vec<&ContentEntry> = Vec::new();
    let mut generic: Vec<&ContentEntry> = Vec::new();

    for entry in CONTENT_DB {
        if entry.concept != concept || entry.min_difficulty > difficulty {
            continue;
        }
        match entry.variant {
            Some(v) if v == variant => specific.push(entry),
            None => generic.push(entry),
            _ => {}
        }
    }

    // Prefer variant-specific; fall back to concept-generic
    let pool = if !specific.is_empty() { &specific } else { &generic };
    if pool.is_empty() {
        return None;
    }

    // Deterministic selection using xorshift
    let idx = (xorshift(seed) % pool.len() as u64) as usize;
    Some(pool[idx])
}
```

This uses the existing `xorshift()` already in theory.rs. Deterministic. No new dependencies.

### 9.3 Changes to mod.rs (One-Time Wiring)

These are the **only code changes** needed. Once done, all future content is data-only.

#### 1. Add fields to MusicTheorySim

```rust
// In MusicTheorySim struct:
current_content: Option<&'static ContentEntry>,  // selected content for this success
show_product: bool,                               // whether to show the product this time
```

#### 2. Modify on_correct() — content selection

After the existing `self.current_insight = ...` lines, add:

```rust
// Select enriched content (fun fact + optional product)
let seed = engine.rng.next_u64();
let content = select_content(concept_idx, variant, self.difficulty, seed);
if let Some(entry) = content {
    self.current_insight = entry.fact.to_string();
    // Show product ~25% of the time (configurable)
    self.show_product = entry.product.is_some()
        && engine.rng.next_u64() % 4 == 0;
    self.current_content = Some(entry);
} else {
    self.current_content = None;
    self.show_product = false;
    // Keep the existing insight from degree_insight/etc. as fallback
}
```

**Key detail:** The existing insight functions remain as fallbacks. If `CONTENT_DB` has no entry for a variant, the current `degree_insight()` / `interval_insight()` etc. still work. This means content can be added incrementally — no need to populate every variant before shipping.

#### 3. Modify render_feedback() — product display

After the existing insight text rendering block (mod.rs:1886–1896), add a product block:

```rust
// Affiliate recommendation (below insight, above [ Next ])
if self.show_product {
    if let Some(entry) = self.current_content {
        if let Some(product) = &entry.product {
            let prod_y = start_y + (lines.len() as i32) * line_h + 20;

            // Blurb line
            text::draw_text_centered(fb, cx, prod_y,
                product.blurb, ACCENT_GOLD, 1);

            // Product name (clickable region)
            let name_y = prod_y + 16;
            text::draw_text_centered(fb, cx, name_y,
                product.name, ACCENT_CYAN, 2);

            // FTC disclosure
            let disc_y = name_y + 20;
            text::draw_text_centered(fb, cx, disc_y,
                "(affiliate link)", DIM_TEXT, 1);

            // Store clickable region for hit-testing
            // (use the same pattern as option buttons)
        }
    }
}
```

#### 4. Add URL-opening via PersistCommand

The app has no mechanism to open URLs. Add one using the existing persist queue pattern:

**persist.rs** — add a variant:

```rust
pub enum PersistCommand {
    Store { key: String, value: String },
    OpenUrl { url: String },  // NEW
}
```

**persist.rs** — add JSON serialization:

```rust
PersistCommand::OpenUrl { url } => {
    format!("{{\"type\":\"OpenUrl\",\"url\":\"{}\"}}", escape_json(url))
}
```

**index.html** — handle in the drain loop:

```javascript
cmds.forEach(cmd => {
    if (cmd.type === "Store") {
        localStorage.setItem(cmd.key, cmd.value);
    } else if (cmd.type === "OpenUrl") {
        window.open(cmd.url, '_blank', 'noopener');
    }
});
```

Then in mod.rs, when the user clicks the product region:

```rust
if let Some(entry) = self.current_content {
    if let Some(product) = &entry.product {
        engine.persist_queue.push(PersistCommand::OpenUrl {
            url: product.url.to_string(),
        });
    }
}
```

### 9.4 Layout: Where Things Go

The success screen currently uses OPTIONS_Y (310) to PIANO_Y (540) — 230px in the reference 900px design. Current layout:

```
310  "Correct!" (scale 2)
340  Insight text (wrapped, scale 2, ~4-6 lines)
500  [ Next ]
540  ── piano ──
```

With a product recommendation (25% of successes):

```
310  "Correct!" (scale 2)
340  Insight text (wrapped, scale 2, ~3-4 lines)
430  "Go deeper into jazz harmony" (scale 1, gold)
446  "The Jazz Harmony Book" (scale 2, cyan, clickable)
466  "(affiliate link)" (scale 1, dim)
500  [ Next ]
540  ── piano ──
```

**Space budget:** The insight text can be shorter for entries with products (the fun fact stands alone at ~2-3 lines vs. the current ~4-5 line insights). The product block needs ~50px.

### 9.5 Adding New Challenge Types (Future-Proofing)

When you add a 6th challenge type (e.g. Rhythm, Timbre), the content system automatically supports it:

1. Add variant to `MusicConcept` enum — assign concept_idx 5
2. Add SRS cards for the new concept in srs.rs
3. Add `ContentEntry` items to `CONTENT_DB` with `concept: 5`
4. The existing `select_content()` picks them up — no changes needed

The `learning_resources()` function already uses the same concept_idx pattern and just needs a new `5 => &[...]` arm.

### 9.6 Content Authoring Workflow

Once the framework ships, adding content is a 3-step process:

**Step 1:** Research a fun fact for a specific challenge variant. E.g., "What's interesting about the minor 3rd?"

**Step 2:** Find a relevant product and get the affiliate URL. E.g., a beginner ukulele on Amazon.

**Step 3:** Add one struct literal to `CONTENT_DB` in theory.rs:

```rust
ContentEntry {
    concept: 2,          // IntervalRecognition
    variant: Some(3),    // minor 3rd (3 semitones)
    min_difficulty: 0,
    fact: "'Greensleeves' opens with a minor 3rd leap — the interval \
           that defines every minor chord and minor key. Just one \
           semitone lower than a major 3rd, but it changes everything.",
    product: Some(ProductRec {
        name: "Beginner Ukulele Kit",
        blurb: "Hear minor 3rds on a real instrument",
        url: "https://www.amazon.com/dp/XXXXXXXXXX?tag=yourtag-20",
        program: "amazon",
    }),
},
```

Compile. Done. No other files changed. The SRS system will surface it when users encounter that variant, and the frequency control decides whether the product shows.

### 9.7 Analytics (Optional, Phase 2)

Track which content resonates using the existing persist queue:

```rust
// On product click:
engine.persist_queue.push(PersistCommand::Store {
    key: "analytics".to_string(),
    value: format!("click:{}:{}:{}", concept, variant, product.program),
});
```

JS side can batch these to a simple endpoint or just accumulate in localStorage for manual export. Don't over-engineer this until the content database is populated and there's real traffic.

### 9.8 File Change Summary

| File | Change | One-time? |
|---|---|---|
| **theory.rs** | Add `ContentEntry`, `ProductRec` structs; add `CONTENT_DB` array; add `select_content()` | Yes — then only `CONTENT_DB` entries grow |
| **mod.rs** | Add `current_content`/`show_product` fields; ~10 lines in `on_correct()`; ~20 lines in `render_feedback()`; click hit-test | Yes |
| **persist.rs** | Add `OpenUrl` variant to `PersistCommand` | Yes |
| **index.html** | Handle `OpenUrl` in drain loop (3 lines) | Yes |

**Total one-time code changes:** ~60-80 lines across 4 files. After that, all content additions are data-only edits to the `CONTENT_DB` array in theory.rs.

---

## 10. Open Music Theory Curriculum Analysis

**Source:** [Open Music Theory](https://openmusictheory.github.io/contents.html) (open source)

~45 lessons scraped and analyzed. This section maps the curriculum to new challenge types, fun facts, SRS modifications, and affiliate opportunities.

### 10.1 New Challenge Categories (Prioritized by Ear-Training Value)

The current app has 5 concepts (35 SRS cards). Open Music Theory reveals **8 high-value new challenge types** that are audible, quizzable, and map naturally to the existing architecture.

#### Priority 1: Ship These Next

**6. Meter Identification** (concept_idx: 5)
- Play a musical excerpt → identify the meter type
- **Variants (6):** Simple Duple (2/4), Simple Triple (3/4), Simple Quadruple (4/4), Compound Duple (6/8), Compound Triple (9/8), Compound Quadruple (12/8)
- **SRS cards:** 6
- **Ear training value:** Very high — users tap along and feel beats/subdivisions
- **Audio implementation:** Generate rhythmic patterns with accented beats using existing `SoundCommand::PlayTone`. Accent beat 1, lighter subdivisions.
- **Difficulty scaling:** Start with simple duple vs. simple triple. Add compound meters at difficulty 4+. Add 9/8 vs. 12/8 at difficulty 7+.
- **Source:** [Meter](https://openmusictheory.github.io/meter.html)

**7. Seventh Chord Quality** (concept_idx: 6)
- Play a four-note chord → identify its quality
- **Variants (5):** Major 7th, Dominant 7th, Minor 7th, Half-Diminished 7th, Diminished 7th
- **SRS cards:** 5
- **Ear training value:** Extremely high — each 7th chord has a distinctive color
- **Audio implementation:** Same as existing ChordQuality but add a 4th note. Use `quality_intervals()` pattern extended to 7ths.
- **Difficulty scaling:** Start with Dominant 7th vs. Major 7th (most distinct). Add Minor 7th at difficulty 3. Add dim/half-dim at difficulty 5+.
- **Source:** [Triads and Seventh Chords](https://openmusictheory.github.io/triads.html)

**8. Pop/Rock Progression Identification** (concept_idx: 7)
- Play a 4-chord loop → identify which named progression it is
- **Variants (7):** Doo-Wop (I-vi-IV-V), Singer/Songwriter (vi-IV-I-V), Blues (I-I-I-I-IV-IV-I-I-V-IV-I-I), Lament (i-VII-VI-V), Plagal (I-IV), Double-Plagal (bVII-IV-I), Circle-of-Fifths (i-iv-VII-III)
- **SRS cards:** 7
- **Ear training value:** Very high — these are the most recognizable patterns in popular music
- **Audio implementation:** Play 2-4 bars of root-position triads. Reuse chord-playing logic from Cadence challenges.
- **Difficulty scaling:** Start with Blues vs. Doo-Wop (most distinct). Add Singer/Songwriter at difficulty 3. Add Lament, Plagal, Circle-of-Fifths at difficulty 5+.
- **Source:** [Pop/Rock Harmony](https://openmusictheory.github.io/popRockHarmony.html) and sub-pages

**9. Cadence Refinement (PAC vs. IAC vs. HC)** (concept_idx: 8)
- Existing Cadence challenge expanded: distinguish PAC from IAC (melody ending on do vs. mi/sol)
- **Variants (4):** PAC (V-I, melody on do), IAC (V-I, melody on mi or sol), HC (phrase ends on V), Plagal (IV-I)
- **SRS cards:** 4
- **Ear training value:** Extremely high — degree of "finality" is directly perceivable
- **Audio implementation:** Add a melody note over the final chord. PAC = do on top, IAC = mi on top, HC = re/ti on top. Reuse existing cadence audio.
- **Source:** [Classical Cadence Types](https://openmusictheory.github.io/cadenceTypes.html)

#### Priority 2: Strong Additions

**10. Embellishing Tone Identification** (concept_idx: 9)
- Play a short melodic fragment with one embellishing tone → identify it
- **Variants (6):** Passing Tone, Neighbor Tone, Suspension (4-3, 7-6, 9-8), Appoggiatura, Anticipation, Escape Tone
- **SRS cards:** 6
- **Ear training value:** Extremely high — suspensions and appoggiaturas have unmistakable sounds
- **Audio implementation:** Two-voice texture. Chord tones + one non-chord tone with characteristic approach/resolution.
- **Difficulty scaling:** Start with Passing Tone vs. Suspension (most distinct). Add Neighbor, Appoggiatura at difficulty 4. Add Anticipation, Escape at difficulty 7.
- **Source:** [Embellishing Tones](https://openmusictheory.github.io/embellishingTones.html)

**11. Scale/Collection Identification** (concept_idx: 10)
- Play a scale or melodic fragment → identify the scale type
- **Variants (8):** Major (Ionian), Natural Minor (Aeolian), Harmonic Minor, Dorian, Mixolydian, Pentatonic, Whole-Tone, Blues Scale
- **SRS cards:** 8
- **Ear training value:** Very high — each scale has a distinctive character
- **Audio implementation:** Play ascending/descending scale from root, or a short melody using only that scale's notes.
- **Difficulty scaling:** Start with Major vs. Minor. Add Pentatonic and Harmonic Minor at difficulty 3. Add modes at difficulty 5. Add Whole-Tone at difficulty 7.
- **Source:** [Scales](https://openmusictheory.github.io/scales.html), [Collections and Scales](https://openmusictheory.github.io/scales2.html)

**12. Contrapuntal Motion** (concept_idx: 11)
- Play two voices moving together → identify the motion type
- **Variants (4):** Parallel, Similar, Contrary, Oblique
- **SRS cards:** 4
- **Ear training value:** Moderate-high — parallel sounds "locked," contrary sounds "opening"
- **Audio implementation:** Two simultaneous melodic lines, 3-4 notes each. One motion type per example.
- **Source:** [Types of Motion](https://openmusictheory.github.io/motionTypes.html)

**13. Musical Form Identification** (concept_idx: 12)
- Listen to a structural description or short excerpt → identify the form
- **Variants (4):** Sentence (presentation + continuation), Period (antecedent + consequent), Strophic (AAA), Verse-Chorus
- **SRS cards:** 4
- **Ear training value:** High — sentence vs. period is very audible, as is verse vs. chorus
- **Audio implementation:** Generate 8-bar phrases with characteristic structures. Sentence: repeated idea + fragmentation. Period: idea + weak cadence + idea + strong cadence.
- **Source:** [The Sentence](https://openmusictheory.github.io/sentence.html), [The Period](https://openmusictheory.github.io/period.html), [Pop/Rock Form](https://openmusictheory.github.io/popRockForm.html)

#### Summary: New SRS Card Count

| Concept | Cards | Total |
|---|---|---|
| Existing 5 concepts | 35 | 35 |
| Meter Identification | 6 | 41 |
| Seventh Chord Quality | 5 | 46 |
| Pop/Rock Progressions | 7 | 53 |
| Cadence Refinement | 4 | 57 |
| Embellishing Tones | 6 | 63 |
| Scale/Collection ID | 8 | 71 |
| Contrapuntal Motion | 4 | 75 |
| Musical Form | 4 | 79 |

From 35 cards to **79 cards** — more than doubling the content.

### 10.2 SRS / Anki Sequence Modifications

The current SRS system (`srs.rs`) assumes all 35 cards are available from the start. With 79+ cards, the system needs **gated unlocking** and **concept grouping**.

#### Unlock Gates

```
Level 1 (default):  ScaleDegree, Interval, ChordQuality
Level 2 (after 20 reviews): RomanNumeral, Cadence
Level 3 (after 50 reviews): Meter, SeventhChord, ScaleCollection
Level 4 (after 100 reviews): PopProgression, CadenceRefined, EmbellishingTone
Level 5 (after 200 reviews): Motion, Form
```

**Implementation:** Add `min_reviews: u32` field to each concept group in `SrsState`. The `select_next_card()` function already filters — add a `total_reviews >= concept.min_reviews` check.

#### Interleaving Strategy

The current SRS selects purely by due-date. With 79 cards across 13 concepts, add a **concept diversity bias**: if the last 3 challenges were the same concept, boost priority of other concepts. This prevents monotony.

```rust
// In select_next_card():
let diversity_penalty = if last_3_concepts.iter().all(|&c| c == card.concept) {
    0.5  // halve priority for same-concept streaks
} else {
    1.0
};
```

#### Daily New Card Limit

Current limit is 10 new cards/day. With 79 cards this remains fine — it takes ~8 days to introduce all cards, which is a good pacing for a casual user.

### 10.3 Fun Fact / Success Text Database

Organized by concept and variant. These are ready to drop into `CONTENT_DB` entries.

#### Intervals (concept 2, expanding existing 13 variants)

| Variant | Semitones | Fun Fact |
|---|---|---|
| P1 (0) | Unison | "Gregorian chant is entirely unison singing — hundreds of monks on the same note. The raw power of unity." |
| m2 (1) | Minor 2nd | "The Jaws theme is just two notes a half step apart. The most dissonant interval creates the most tension." |
| M2 (2) | Major 2nd | "'Happy Birthday' starts with a major 2nd — the basic building block of every scale." |
| m3 (3) | Minor 3rd | "'Greensleeves' opens with a minor 3rd. This one interval defines every minor chord and minor key." |
| M3 (4) | Major 3rd | "Do-mi is ALWAYS a major 3rd, no matter what key you're in. That's the power of movable-do solfege." |
| P4 (5) | Perfect 4th | "The perfect 4th is music's great paradox: consonant in a melody, but traditionally dissonant in harmony!" |
| TT (6) | Tritone | "There are only 13 possible chromatic intervals in an octave, but the tritone is the only one that is its own inversion." |
| P5 (7) | Perfect 5th | "Power chords in rock are just root and fifth — the perfect 5th is the foundation of the overtone series." |
| m6 (8) | Minor 6th | "The minor 6th is the inversion of the major 3rd. Flip M3 upside down and you get m6 — major becomes minor." |
| M6 (9) | Major 6th | "Jazz musicians love added 6th chords for their smooth, mellow quality. The major 6th gives warmth without tension." |
| m7 (10) | Minor 7th | "The minor 7th is the heart of dominant 7th chords. It adds soulful tension without the extreme pull of the major 7th." |
| M7 (11) | Major 7th | "Major 7th chords are the signature sound of jazz ballads and bossa nova — dreamy, floating dissonance." |
| P8 (12) | Octave | "The frequency ratio of an octave is exactly 2:1. Your brain perceives octave-displaced notes as 'the same' — nobody fully understands why." |

#### Chord Quality (concept 3, expanding existing 4 variants)

| Variant | Fun Fact |
|---|---|
| Major | "Every major key contains exactly 3 major triads (I, IV, V), 3 minor triads (ii, iii, vi), and 1 diminished (vii°). The same recipe, every time." |
| Minor | "Major and minor differ by exactly one semitone — the 3rd. That single half-step is the difference between joy and melancholy." |
| Diminished | "A diminished 7th chord divides the octave into four equal minor thirds. There are really only 3 unique dim7 chords — every other is a rearrangement." |
| Augmented | "An augmented triad divides the octave into three equal parts. It sounds mysterious because your ear can't find a 'bottom.'" |

#### Cadences (concept 4, expanding existing 4 variants)

| Variant | Fun Fact |
|---|---|
| Authentic | "The cadential 6/4 is one of the most analyzed moments in all of theory. Despite looking like a tonic chord, it functions as a decorated dominant." |
| Plagal | "The Picardy third — ending a minor piece with a major chord — has been used since the Renaissance. Minor-key pieces almost ALWAYS ended major in the 1500s." |
| Half | "A half cadence is a musical question mark. The phrase pauses on the dominant, demanding continuation." |
| Deceptive | "V to vi — when Radiohead uses a deceptive cadence, they're using the same trick Beethoven did 200 years earlier." |

#### New Concepts: Pop/Rock Progressions (concept 7)

| Variant | Progression | Fun Fact |
|---|---|---|
| Doo-Wop | I-vi-IV-V | "Coldplay's 'Viva la Vida' uses the same doo-wop progression from the 1950s — just starting on chord IV instead of I." |
| Singer/Songwriter | vi-IV-I-V | "The chords Am-F-C-G work as vi-IV-I-V in C major OR i-VI-III-VII in A minor. Songwriters exploit this ambiguity to shift between happy and sad." |
| Blues | I-IV-V (12-bar) | "The 12-bar blues is built from just three chords (I, IV, V) but its specific ordering created the foundation for rock and roll." |
| Lament | i-VII-VI-V | "Purcell's 'Dido's Lament' and Muse's 'Thoughts of a Dying Atheist' use the same descending bass — 400 years apart." |
| Plagal | I-IV | "The plagal cadence (IV-I) is called the 'Amen cadence' because it's sung on 'A-men' at the end of hymns." |
| Double-Plagal | bVII-IV-I | "The 'Hey Jude' coda is the most famous double-plagal progression in rock: bVII-IV-I." |
| Circle-of-Fifths | i-iv-VII-III | "Gloria Gaynor's 'I Will Survive' uses a complete circle-of-fifths: i-iv-VII-III-VI-ii-V. Every diatonic chord in order." |

#### New Concepts: Meter (concept 5)

| Variant | Fun Fact |
|---|---|
| Simple Duple (2/4) | "March time — simple duple meter drives everything from Sousa marches to punk rock." |
| Simple Triple (3/4) | "The waltz is in simple triple meter. That 'ONE-two-three' feel has made people dance for centuries." |
| Simple Quadruple (4/4) | "4/4 is so common in pop that it's literally called 'common time' and gets its own symbol: C." |
| Compound Duple (6/8) | "6/8 feels like TWO big beats, not six small ones. That's why Irish jigs bounce — each beat naturally divides into three." |
| Compound Triple (9/8) | "Radiohead's 'The Tourist' is in compound triple (9/8). Three big beats, each dividing into three." |
| Compound Quadruple (12/8) | "Beethoven's 'Moonlight' Sonata uses 12/8 — that's why the triplet arpeggios feel so hypnotic." |

#### New Concepts: Seventh Chords (concept 6)

| Variant | Fun Fact |
|---|---|
| Major 7th | "The major 7th chord sounds like a jazz club at midnight — dreamy and sophisticated. It's built on the I and IV of any major key." |
| Dominant 7th | "The V7 chord appears in both major AND minor keys. Its minor 7th interval creates tension that desperately wants to resolve." |
| Minor 7th | "Minor 7th chords are the warm, mellow sound of soul and R&B. The ii7 chord in jazz is almost always a minor 7th." |
| Half-Diminished 7th | "The half-diminished 7th is the wistful outsider — it naturally occurs on the 7th degree of major keys, perpetually unresolved." |
| Diminished 7th | "Only 3 unique diminished 7th chords exist — the other 9 are just inversions. Each one divides the octave into perfectly equal minor thirds." |

#### New Concepts: Embellishing Tones (concept 9)

| Variant | Fun Fact |
|---|---|
| Passing Tone | "Passing tones fill in the gaps between chord tones by step. Remove them and the melody becomes a skeleton of leaps." |
| Neighbor Tone | "A neighbor tone departs from home by step and immediately returns — the musical equivalent of peeking around a corner." |
| Suspension | "The 4-3 suspension has been making listeners sigh since the Renaissance. Hold a note past its welcome, then let it fall by step." |
| Appoggiatura | "'Appoggiatura' means 'leaning note' in Italian — it crashes in by leap on a strong beat and gently resolves by step." |
| Anticipation | "An anticipation is the musical equivalent of finishing someone's sentence — the melody arrives at the next chord tone before the harmony gets there." |
| Escape Tone | "The escape tone does exactly what its name says: it steps away from a chord tone, then LEAPS away instead of resolving." |

#### New Concepts: Scales/Collections (concept 10)

| Variant | Fun Fact |
|---|---|
| Major | "The major scale pattern W-W-H-W-W-W-H has been the foundation of Western music for centuries." |
| Natural Minor | "C major and A natural minor use the exact same notes — just starting from different places. They're musical twins." |
| Harmonic Minor | "Harmonic minor has a raised 7th, creating that exotic augmented 2nd gap. You can hear it in Bach, surf rock, AND Middle Eastern music." |
| Dorian | "Miles Davis's 'So What' is built entirely on Dorian mode. It has a minor feel but with a brighter raised 6th." |
| Mixolydian | "Mixolydian is the mode of blues-rock — a major scale with a flatted 7th. Think 'Sweet Home Alabama.'" |
| Pentatonic | "The black keys on a piano form a perfect pentatonic scale. Play only the black keys and everything sounds good together." |
| Whole-Tone | "Only 2 whole-tone scales exist in all of music. Debussy loved them for their dreamy, directionless quality." |
| Blues | "The blues scale adds one 'blue note' (the flatted 5th) to the minor pentatonic. That single note defines an entire genre." |

### 10.4 Affiliate Link Opportunities by New Challenge Type

| New Challenge | Top Affiliate Mapping | Why It's Natural |
|---|---|---|
| **Meter** | Metronome (Korg TM-60), Drumsticks, Practice Pad | Meter = rhythm = physical practice tools |
| **Seventh Chords** | Jazz theory books (Mark Levine's *The Jazz Theory Book*), MIDI keyboard | 7th chords = jazz gateway |
| **Pop Progressions** | Hooktheory Books I & II, songwriting courses on Skillshare | Progression ID → songwriting interest |
| **Cadence (refined)** | Coursera Berklee courses, classical piano collections | PAC/IAC/HC → deeper classical study |
| **Embellishing Tones** | Counterpoint textbooks (Fux's *Gradus*, Salzer's *Structural Hearing*) | Ornaments → voice-leading study |
| **Scale/Collection** | Guitar instruction books (genre-specific), Ableton trial (for modes) | Scale → genre → instrument/DAW |
| **Motion Types** | Bach chorales book, part-writing workbooks | Counterpoint → choral/composition study |
| **Form** | Songwriting books (Pat Pattison), DAW subscriptions (Splice, GarageBand) | Form awareness → production/composition |

**Highest-value new affiliate mappings:**
1. **Pop Progressions → Hooktheory** (~$5-10/sale via Amazon, very high relevance)
2. **Meter → Metronome** (Amazon ~7%, near-perfect tool match)
3. **Seventh Chords → Mark Levine's Jazz Theory Book** (Amazon ~4.5%, classic recommendation)
4. **Scales → Genre-specific guitar books** (Amazon ~4.5%, scale → genre → instrument pipeline)

### 10.5 Song Reference Database

Songs mentioned across Open Music Theory that can be used for fun facts and cultural context:

| Song | Artist | Concept | Use Case |
|---|---|---|---|
| "Moonlight" Sonata | Beethoven | Meter (12/8) | Compound quadruple example |
| "With or Without You" | U2 | Meter (4/4), Pop form | Simple quadruple; simple verse-chorus |
| "The Tourist" | Radiohead | Meter (9/8) | Compound triple example |
| "Shiver" | Radiohead | Meter (6/8) | Compound duple example |
| "Viva la Vida" | Coldplay | Progression | Rotated doo-wop |
| "Friday" | Rebecca Black | Progression | Doo-wop in modern pop |
| "I Will Survive" | Gloria Gaynor | Progression | Complete circle-of-fifths |
| "Hey Jude" | The Beatles | Progression | Double-plagal coda |
| "Hey Joe" | Jimi Hendrix | Progression | Extended plagal |
| "Hound Dog" | Elvis | Progression | 12-bar blues |
| "House of the Rising Sun" | The Animals | Progression | Puff (minor) |
| "So What" | Miles Davis | Scale | Dorian mode |
| "Sweet Home Alabama" | Lynyrd Skynyrd | Scale | Mixolydian |
| "Dido's Lament" | Purcell | Progression | Lament bass |
| "I Want to Hold Your Hand" | The Beatles | Form | AABA |
| "Livin' on a Prayer" | Bon Jovi | Form | Verse-chorus with prechorus |
| "Blue Suede Shoes" | Carl Perkins | Form | Strophic |
| "I Wanna Be Sedated" | The Ramones | Modulation | Step-up modulation |
| "Maria" (West Side Story) | Bernstein | Interval | Tritone |
| "Here Comes the Bride" | Wagner | Interval | Perfect 4th |
| "Star Wars" | John Williams | Interval | Perfect 5th |
| "Somewhere Over the Rainbow" | Garland | Interval | Octave |

### 10.6 Hints System Enhancement

The current app has a hint system (`hint_used`, `eliminated_options`). Open Music Theory content suggests richer hint types per concept:

| Challenge Type | Current Hint | Enhanced Hint (from OMT) |
|---|---|---|
| **Interval** | Eliminate 2 options | + "Song reference" hint: show the song that starts with this interval |
| **Chord Quality** | Eliminate 2 options | + "Interval content" hint: show the 3rd type (major or minor) |
| **Cadence** | Eliminate 2 options | + "Bass motion" hint: show whether bass moves up, down, or stays |
| **Scale Degree** | Eliminate 2 options | + "Solfege" hint: show the solfege syllable (do, re, mi...) |
| **Pop Progression** | (new) | "Bass pattern" hint: show descending, ascending, or static |
| **Meter** | (new) | "Count along" hint: show beat groupings (1-2, 1-2-3, 1-2-3-4) |
| **Seventh Chord** | (new) | "Triad base" hint: show the underlying triad quality (major, minor, dim) |
| **Embellishing Tone** | (new) | "Direction" hint: show approach direction (step/leap) and resolution direction |

**Implementation:** Add a `hint_text: Option<&'static str>` field to `ContentEntry`. On hint press, show the text instead of eliminating options. More educational, less game-y.

### 10.7 Galant Schemata (Advanced Feature — Future)

Open Music Theory's Galant schemata content (Meyer, Prinner, Fonte, Monte, Ponte) represents an **advanced challenge tier** that no other ear-training app offers. This could be a differentiator.

**Concept: Schema Identification** (concept_idx: 13, future)
- Play a 4-8 bar Classical-style phrase → identify the schema
- **Variants:** Meyer (tonic prolongation opener), Prinner (fa-mi-re-do closing), Fonte (descending sequence), Monte (ascending sequence), Ponte (dominant pedal)
- **Ear training value:** Very high for intermediate/advanced users
- **Affiliate angle:** Gjerdingen's *Music in the Galant Style* (the definitive book, ~$40 on Amazon)

**Fun facts:**
- "18th-century composers didn't write from scratch — they assembled music from 'schemata,' stock phrases internalized since childhood. It's like musical Lego."
- "The Prinner (bass: fa-mi-re-do with melody a tenth above) is one of the most common phrases in all of Classical music. Once you hear it, you'll recognize it everywhere."
- "Fonte, Monte, Ponte — Fountain, Mountain, Bridge. These Italian names from the 1750s perfectly describe how each pattern moves: down, up, or nowhere."

### 10.8 Advanced Harmony Concepts (Content-Only — No New Challenge Types Needed)

These topics from Open Music Theory are too complex for new challenge types but provide excellent fun-fact content for existing challenges:

**Applied Chords / Tonicization** — Fun facts for existing chord/cadence challenges:
- "An applied chord is like a brief vacation to another key — you borrow a chord to make one of your own feel momentarily like a tonic."
- "The most common applied chord is V/V — the 'dominant of the dominant.' It creates a brief flash of a new key."

**Modal Mixture** — Fun facts for existing chord quality challenges:
- "When Nirvana uses a bVII chord in a major-key song, that's modal mixture — borrowing from the parallel minor for a darker sound."
- "Modal mixture is painting with colors borrowed from a parallel universe — the parallel key."

**Augmented Sixth Chords** — Fun facts for existing interval challenges:
- "The 'Swiss' augmented sixth chord gets its name because it 'sounds German but is spelled French' — a bilingual musical pun."
- "All four augmented-sixth chords share the same defining interval: the augmented 6th between b6 and #4, which expands outward to an octave when it resolves."

**Tendency Tones** — Fun facts for existing scale degree challenges:
- "The leading tone (ti) is music's most impatient note — it 'wants' to resolve up to do so strongly that when it doesn't, theorists call it 'frustrated.'"
- "Le (b6) is the 'dark' tendency tone — it pulls downward to sol with melancholy gravity, while ti pushes upward to do with bright urgency."

**Modulation** — Fun facts for existing cadence challenges:
- "The 'truck-driver modulation' — named by scholar Walter Everett — sounds like shifting gears: drop to the dominant of the new key, then rev up to the new tonic."
- "That key change near the end of a pop song that lifts everything up? It's called a 'pump-up modulation.'"

**Counterpoint** — Fun facts for existing interval/chord challenges:
- "Parallel fifths are the cardinal sin of counterpoint. Two voices in parallel perfect consonances lose their independence — they start sounding like one voice."
- "Contrary motion — voices moving in opposite directions — is the gold standard for voice independence. It's why Bach sounds so richly layered."
- "Music exploits a survival mechanism: your brain uses sound irregularities to detect threats. That's why a surprise chord change gives you chills."

### 10.9 Difficulty Ladder (Full Curriculum)

```
Difficulty 1-2:  Major/minor scales, basic intervals (P5, P4, P8, M3, m3)
                 Major/minor triads, simple meter (3/4 vs. 4/4)

Difficulty 3-4:  All intervals, diminished/augmented triads
                 Compound meter (6/8), I-IV-V-I cadences
                 Pentatonic scale, doo-wop progression

Difficulty 5-6:  Seventh chords (Maj7, Dom7, min7)
                 PAC vs. IAC vs. HC cadences
                 Dorian/Mixolydian modes, blues progression
                 Singer/songwriter + lament progressions
                 Passing tones, neighbor tones

Difficulty 7-8:  Half-dim/dim seventh chords
                 Harmonic minor, whole-tone scale
                 Suspensions, appoggiaturas
                 Contrapuntal motion types
                 Circle-of-fifths + double-plagal progressions
                 Sentence vs. period form

Difficulty 9-10: All embellishing tones
                 All scale collections (octatonic, chromatic)
                 Pop form identification
                 Galant schemata (advanced)
                 Modulation detection
```

---

## 11. Corrections to Fun Facts

Research turned up several claims that need fixing before they ship.

| Claim | Issue | Correction |
|---|---|---|
| "Beethoven's Moonlight Sonata uses 12/8 meter" | **Incorrect.** The first movement is in 4/4 (common time) with triplet eighth notes, not 12/8. | Rewrite: "Beethoven wrote the Moonlight Sonata's first movement in 4/4, but the persistent triplet figuration creates an effect so close to 12/8 that many listeners assume it is." |
| "The tritone was called 'diabolus in musica' in medieval times" | **Misleading.** No medieval source uses this exact phrase. Earliest documented use: Werckmeister, *Harmonologia Musica* (1702). | Rewrite: "The tritone was called 'diabolus in musica' — though the phrase is younger than you'd think. The earliest citation is from 1702, not the Middle Ages." |
| "The Pachelbel progression (I-V-vi-IV-I-IV-V)" | **Inaccurate simplification.** The actual Canon in D progression is I-V-vi-iii-IV-I-IV-V. Pop's "four-chord song" (I-V-vi-IV) is a subset. | Use the correct full progression or clarify that pop uses a simplified version. |
| "Bobby McFerrin demonstrated pentatonic universality" | **Overstated.** His 2009 World Science Festival demo was compelling but not a controlled experiment (Western audience, self-selected). | Frame as "demonstration" not "proof." |
| "Almost always ended with Picardy third in the 1500s" | **Slightly overstated** for the 1500s. More accurate for the 1600s-1700s. | Soften to "by the 1600s, the Picardy third was standard practice." |

---

## 12. Bibliography

All fun facts in the content database should carry a reference code pointing to this bibliography. Format: `[REF-XX]` in the `ContentEntry` comment.

### Primary Sources

- **[REF-01]** Fux, Johann Joseph. *Gradus ad Parnassum*. Vienna: Johann Peter van Ghelen, 1725. English trans.: *The Study of Counterpoint from Johann Joseph Fux's Gradus ad Parnassum*. Trans. and ed. Alfred Mann. New York: W. W. Norton, 1965.
- **[REF-02]** Guido d'Arezzo. *Micrologus*. Ca. 1025-1026. Modern edition: Smits van Waesberghe, Joseph, ed. *Guidonis Aretini Micrologus*. Rome: American Institute of Musicology, 1955. (Corpus Scriptorum de Musica, vol. 4.)
- **[REF-03]** Schoenberg, Arnold. "Composition with Twelve Tones (1)." In *Style and Idea: Selected Writings of Arnold Schoenberg*, edited by Leonard Stein, translated by Leo Black, 214-245. Berkeley: University of California Press, 1975.
- **[REF-04]** Messiaen, Olivier. *Technique de mon langage musical*. 2 vols. Paris: Alphonse Leduc, 1944. English trans.: John Satterfield, 1956.
- **[REF-05]** Schoenberg, Arnold. *Harmonielehre* [Theory of Harmony]. Vienna: Universal Edition, 1911. English trans.: Roy E. Carter. Berkeley: University of California Press, 1978.
- **[REF-06]** Rousseau, Jean-Jacques. *Dictionnaire de musique*. Paris: Veuve Duchesne, 1768. (First use of "tierce de Picardie.")
- **[REF-07]** Mattheson, Johann. *Das neu-eröffnete Orchestre*. Hamburg: B. Schiller's Widow, 1713. (Key-affect associations.)
- **[REF-08]** Schubart, Christian Friedrich Daniel. *Ideen zu einer Aesthetik der Tonkunst*. Vienna: J. V. Degen, 1806. (Key-character descriptions.)
- **[REF-09]** Diletsky (Dyletsky), Nikolai. *Idea grammatiki musikiyskoy*. Moscow, 1679. (Earliest known circle of fifths.)
- **[REF-10]** Werckmeister, Andreas. *Harmonologia Musica*. Quedlinburg, 1702. (Earliest documented use of "diabolus in musica.")

### Scholarly Monographs

- **[REF-11]** Huron, David. *Sweet Anticipation: Music and the Psychology of Expectation*. Cambridge, MA: MIT Press, 2006. (Melodic tendencies, ITPRA theory, musical expectation.)
- **[REF-12]** Huron, David. *Voice Leading: The Science behind a Musical Art*. Cambridge, MA: MIT Press, 2016. (Psychoacoustic basis for voice-leading rules, tonal fusion principle.)
- **[REF-13]** Meyer, Leonard B. *Style and Music: Theory, History, and Ideology*. Philadelphia: University of Pennsylvania Press, 1989. (Laws/rules/strategies framework.)
- **[REF-14]** Gjerdingen, Robert O. *Music in the Galant Style*. New York: Oxford University Press, 2007. (Galant schemata: Prinner, Meyer, Romanesca, Fonte, Monte, Ponte.)
- **[REF-15]** Caplin, William E. *Classical Form: A Theory of Formal Functions for the Instrumental Music of Haydn, Mozart, and Beethoven*. New York: Oxford University Press, 1998. (Sentence, period, formal functions.)
- **[REF-16]** Caplin, William E. *Analyzing Classical Form: An Approach for the Classroom*. New York: Oxford University Press, 2013.
- **[REF-17]** Doll, Christopher. *Hearing Harmony: Toward a Tonal Theory for the Rock Era*. Ann Arbor: University of Michigan Press, 2017. (Pop/rock harmonic analysis.)
- **[REF-18]** Temperley, David. *The Musical Language of Rock*. New York: Oxford University Press, 2018. (Syncopation, speech-rhythm connections.)
- **[REF-19]** Lehman, Frank. *Hollywood Harmony: Musical Wonder and the Sound of Cinema*. New York: Oxford University Press, 2018. (Film score analysis, modal usage.)
- **[REF-20]** Walser, Robert. *Running with the Devil: Power, Gender, and Madness in Heavy Metal Music*. Hanover, NH: Wesleyan University Press, 1993. (Power chord definition.)
- **[REF-21]** Levine, Mark. *The Jazz Theory Book*. Petaluma, CA: Sher Music Co., 1995. (Jazz harmony, Dorian mode, seventh chords.)
- **[REF-22]** Bregman, Albert S. *Auditory Scene Analysis: The Perceptual Organization of Sound*. Cambridge, MA: MIT Press, 1990. (Psychoacoustics of voice separation.)
- **[REF-23]** Everett, Walter. *The Beatles as Musicians: Revolver through the Anthology*. New York: Oxford University Press, 1999. (Double-plagal progression, Beatles harmonic analysis.)
- **[REF-24]** Audissino, Emilio. *John Williams's Film Music*. Madison: University of Wisconsin Press, 2014. (Jaws theme, Star Wars analysis.)
- **[REF-25]** Jarman, Douglas. *The Music of Alban Berg*. Berkeley: University of California Press, 1979. (Berg's tonal twelve-tone practice.)
- **[REF-26]** Parks, Richard S. *The Music of Claude Debussy*. New Haven: Yale University Press, 1989. (Whole-tone scale in "Voiles".)
- **[REF-27]** Simeone, Nigel. *Leonard Bernstein: West Side Story*. Farnham: Ashgate, 2009. (Tritone motif in "Maria".)
- **[REF-28]** Forte, Allen. *The American Popular Ballad of the Golden Era, 1924-1950*. Princeton: Princeton University Press, 1995. ("Somewhere Over the Rainbow" analysis.)
- **[REF-29]** Hiley, David. *Western Plainchant: A Handbook*. Oxford: Clarendon Press, 1993. (Gregorian chant as monophonic/unison.)
- **[REF-30]** Jensen, Claudia R. *Musical Cultures in Seventeenth-Century Russia*. Bloomington: Indiana University Press, 2009. (Diletsky's circle of fifths in context.)
- **[REF-31]** Stoia, Nicholas. *Sweet Thing: The History and Musical Structure of a Shared American Vernacular Form*. New York: Oxford University Press, 2021. (12-bar blues structure.)
- **[REF-32]** Pople, Anthony. *Berg: Violin Concerto*. Cambridge: Cambridge University Press, 1991. (Berg's tonal rows.)

### Textbooks (Standard Harmony/Theory)

- **[REF-33]** Kostka, Stefan, Dorothy Payne, and Byron Almen. *Tonal Harmony*. 8th ed. New York: McGraw-Hill, 2018. (Diatonic triads, diminished 7th symmetry, augmented sixths.)
- **[REF-34]** Aldwell, Edward, Carl Schachter, and Allen Cadwallader. *Harmony and Voice Leading*. 4th ed. Boston: Schirmer/Cengage, 2011.
- **[REF-35]** Laitz, Steven G. *The Complete Musician*. 4th ed. New York: Oxford University Press, 2016.
- **[REF-36]** Straus, Joseph N. *Introduction to Post-Tonal Theory*. 4th ed. New York: W. W. Norton, 2016. (Whole-tone/octatonic collections, twelve-tone theory, set theory.)
- **[REF-37]** Benward, Bruce, and Marilyn Saker. *Music in Theory and Practice*. 9th ed. New York: McGraw-Hill, 2015. (Song-interval associations, key signature mnemonics.)
- **[REF-38]** Karpinski, Gary S. *Aural Skills Acquisition*. New York: Oxford University Press, 2000. (Ear-training pedagogy, interval recognition.)
- **[REF-39]** Jeppesen, Knud. *Counterpoint: The Polyphonic Vocal Style of the Sixteenth Century*. Trans. Glen Haydon. New York: Prentice-Hall, 1939. Reprint: Dover, 1992.

### Journal Articles

- **[REF-40]** De Clercq, Trevor, and David Temperley. "A Corpus Analysis of Rock Harmony." *Popular Music* 30, no. 1 (2011): 47-70. (I-V-vi-IV as common pop progression.)
- **[REF-41]** Everett, Walter. "Making Sense of Rock's Tonal Systems." *Music Theory Online* 10, no. 4 (December 2004). (Six tonal systems, "truck-driver modulation.")
- **[REF-42]** Rosand, Ellen. "The Descending Tetrachord: An Emblem of Lament." *The Musical Quarterly* 65, no. 3 (1979): 346-359. (Lament bass topos, Purcell's Dido.)
- **[REF-43]** Huron, David. "Tone and Voice: A Derivation of the Rules of Voice-Leading from Perceptual Principles." *Music Perception* 19, no. 1 (2001): 1-64. (Parallel fifths and perceptual fusion.)
- **[REF-44]** Schneller, Tom. "Modal Interchange and Semantic Resonance in Themes by John Williams." *Journal of Film Music* 6, no. 1 (2015): 49-74. (Lydian mode in film scores.)
- **[REF-45]** Tan, Ivan, Ethan Lustig, and David Temperley. "Anticipatory Syncopation in Rock: A Corpus Study." *Music Perception* 36, no. 4 (2019): 353-370. (Syncopation matching speech patterns.)
- **[REF-46]** Biamonte, Nicole. "Formal Functions of Metric Dissonance in Rock Music." *Music Theory Online* 20, no. 2 (2014). (3+3+2 "tresillo" pattern.)
- **[REF-47]** Juslin, Patrik N., and Daniel Vastfjall. "Emotional Responses to Music: The Need to Consider Underlying Mechanisms." *Behavioral and Brain Sciences* 31, no. 5 (2008): 559-575. (Brainstem reflexes, survival mechanisms.)
- **[REF-48]** Panksepp, Jaak. "The Emotional Sources of 'Chills' Induced by Music." *Music Perception* 13, no. 2 (1995): 171-207.
- **[REF-49]** Manuel, Peter. "The Anticipated Bass in Cuban Popular Music." *Latin American Music Review* 6, no. 2 (1985): 249-261. (Tresillo / 3+3+2 Afro-Cuban origins.)

### Ethnomusicology & Universals

- **[REF-50]** Bernstein, Leonard. *The Unanswered Question: Six Talks at Harvard*. Cambridge, MA: Harvard University Press, 1976. (Pentatonic universality argument.)
- **[REF-51]** Nettl, Bruno. "An Ethnomusicologist Contemplates Universals in Musical Sound and Musical Culture." In *The Origins of Music*, edited by Nils L. Wallin, Bjorn Merker, and Steven Brown, 463-472. Cambridge, MA: MIT Press, 2000.
- **[REF-52]** Sachs, Curt. *The Rise of Music in the Ancient World, East and West*. New York: W. W. Norton, 1943. (Pentatonic cross-cultural distribution.)

### Reference Works

- **[REF-53]** Randel, Don Michael, ed. *The Harvard Dictionary of Music*. 4th ed. Cambridge, MA: Belknap Press, 2003. (Picardy third, appoggiatura etymology.)
- **[REF-54]** Kernfeld, Barry, ed. *The New Grove Dictionary of Jazz*. 2nd ed. New York: Oxford University Press, 2002. ("So What" entry.)

### Dissertations

- **[REF-55]** Terry, Jason. "A History of the Plagal-Amen Cadence." D.M.A. diss., University of South Carolina, 2016. (IV-I "Amen" origin traced to Tallis.)

### Open Educational Resources

- **[REF-56]** Shaffer, Kris, et al. *Open Music Theory*. https://openmusictheory.github.io/. (CC-BY-SA. Source for curriculum structure, "fake triplet" terminology, pop progression schemata.)

### Musical Scores & Recordings Cited

- Beethoven, Ludwig van. *Piano Sonata No. 14 in C-sharp minor, Op. 27, No. 2* ("Moonlight"). Vienna: Giovanni Cappi, 1802.
- Bernstein, Leonard. *West Side Story*. Vocal score. New York: G. Schirmer / Boosey & Hawkes, 1957.
- Davis, Miles. *Kind of Blue*. Columbia CL 1355, 1959. (Liner notes by Bill Evans.)
- Hill, Mildred J., and Patty Smith Hill. "Good Morning to All." *Song Stories for the Kindergarten*. Chicago: Clayton F. Summy, 1893.
- Purcell, Henry. *Dido and Aeneas*, Z. 626. Premiered ca. 1689.
- Wagner, Richard. *Lohengrin*, WWV 75. Premiered August 28, 1850, Weimar. ("Bridal Chorus," Act III.)
- Williams, John. *Jaws: Original Motion Picture Soundtrack*. MCA Records, 1975.
- Williams, John. *Star Wars: Original Motion Picture Soundtrack*. 20th Century Records, 1977.
- McFerrin, Bobby. "Notes & Neurons: In Search of the Common Chorus." Panel presentation, World Science Festival, New York, June 12, 2009.

---

## 13. Next Steps

1. **Build the fun-fact content database** — Start with 30-50 high-quality entries across all challenge types. Section 10.3 has ~80 ready-to-use entries. Apply corrections from Section 11 first.
2. **Sign up for Amazon Associates** — Covers the widest range of products. Do this first.
3. **Apply to Coursera and Skillshare** — Highest commissions for the most relevant products.
4. **Implement the one-time framework** — ~60-80 lines across 4 files (see Section 9.8).
5. **Add Meter and Seventh Chord challenges** — Highest ear-training value, lowest implementation effort (reuse existing audio patterns).
6. **Add Pop/Rock Progression identification** — Unique differentiator, massive fun-fact potential, strong affiliate mapping to Hooktheory.
7. **Implement tracking** — CTR per challenge type, per product, and (critically) whether showing affiliate content affects user retention.
8. **Iterate based on data** — Kill what doesn't work. Expand what does.

---

## 14. Coursera Affiliate Link Candidates — Ranked Analysis

*Research date: March 2026. Data sourced from Coursera course pages and web research.*

### Commission Structure Recap

| Product Type | Commission | Cookie |
|---|---|---|
| Individual courses | 20% | 30 days |
| Specializations & Professional Certificates | **45%** | 30 days |
| Coursera Plus subscription | 10% | 30 days |

**Key insight:** Specializations pay 2.25x more commission than individual courses. Prioritize linking to specializations wherever the audience fit is strong.

### Tier 1: Near-Perfect Audience Match — Link These First

These courses teach *exactly* what Chord Reps users are practicing. The overlap is near-total.

| # | Course | Provider | Enrollments | Rating | Commission | URL Slug |
|---|---|---|---|---|---|---|
| 1 | **Fundamentals of Music Theory** | U. of Edinburgh | **399,308** | 4.52 (1,853 reviews) | 20% | `/learn/edinburgh-music-theory` |
| 2 | **Developing Your Musicianship** (Specialization) | Berklee | **112,800+** | 4.4 (1,400 reviews) | **45%** | `/specializations/musicianship-specialization` |
| 3 | **Getting Started With Music Theory** | Michigan State | **120,291** | 4.46 (758 reviews) | 20% | `/learn/music-theory` |

**Why these are the top 3:**

1. **Edinburgh (399K enrolled)** — The single most enrolled music theory course on Coursera. Covers intervals, scales, chords, cadences, harmony — *the exact same skills* Chord Reps drills. Beginner-friendly. 399K enrollments proves massive demand. Show this after ANY challenge type.

2. **Berklee Musicianship Specialization (112K, 45% commission)** — This is the money play. A specialization earns 45% vs. 20% for individual courses. If a Coursera Plus subscription costs ~$59/month, a 45% commission on a ~$49/month specialization enrollment is significant. Covers ear training + theory + chord progressions — literally the same learning objectives as the app. Three courses + capstone project gives users a structured path.

3. **Michigan State (120K enrolled)** — Strong beginner on-ramp. Covers triads, seventh chords, Roman numeral analysis. Good alternative to Edinburgh for users who want a different teaching style.

**Challenge-to-course mapping:**

| Challenge Type | Best Tier 1 Link | Why |
|---|---|---|
| **Any (beginner, difficulty 1-4)** | Edinburgh or Michigan State | Broadest match, beginner-friendly |
| **Chord Quality / Intervals** | Berklee Musicianship Specialization | Ear training focus, 45% commission |
| **Scale Degrees / Roman Numerals** | Michigan State | Explicit Roman numeral coverage |
| **Cadences** | Edinburgh | Cadences + harmony in depth |

### Tier 2: Strong Match for Specific Challenge Types

These courses match well when surfaced *after specific challenge types* at intermediate+ difficulty.

| # | Course | Provider | Enrollments | Rating | Commission | URL Slug |
|---|---|---|---|---|---|---|
| 4 | **Musicianship: Tensions, Harmonic Function, and Modal Interchange** | Berklee | 29,242 | **4.87** (403 reviews) | 20% | `/learn/musicianship-harmony` |
| 5 | **Jazz Improvisation** | Berklee (Gary Burton) | 74,218 | **4.81** (789 reviews) | 20% | `/learn/jazz-improvisation` |
| 6 | **Songwriting: Writing, Arranging, and Producing Music** (Specialization) | Berklee | — | — | **45%** | `/specializations/songwriting` |

**Why these work:**

4. **Berklee Tensions/Modal Interchange (4.87 rating!)** — Highest-rated music course on Coursera. Covers II-V-I, modal interchange, diminished chords, tensions — advanced chord knowledge. Surface after difficulty 5+ on chord progression or cadence challenges. The 4.87 rating is a trust signal.

5. **Jazz Improvisation (Gary Burton)** — Celebrity instructor (legendary vibraphonist). Users identifying Dorian mode, ii-V-I progressions, or jazz-related content get this. 74K enrollments proves demand. "Learn to improvise over the chords you just identified" is a natural pitch.

6. **Songwriting Specialization (45% commission)** — When users identify pop progressions (I-V-vi-IV, etc.), the natural next step is "write your own songs using these progressions." Specialization = 45% commission. Maps perfectly to the Pop/Rock Progression challenge type from Section 10.

**Challenge-to-course mapping:**

| Challenge Type | Best Tier 2 Link | Why |
|---|---|---|
| **Chord Progressions (difficulty 5+)** | Berklee Tensions/Modal Interchange | Deepens harmonic understanding |
| **Scale ID (Dorian, Mixolydian)** | Jazz Improvisation | Modal jazz = natural gateway |
| **Pop Progressions** | Songwriting Specialization | "Write with these chords" (45% commission) |
| **Seventh Chords** | Berklee Tensions/Modal Interchange | Tensions are 7th chord extensions |

### Tier 3: Production Pipeline — "Now Create With What You Hear"

Users who master ear training often want to produce. These courses monetize that transition.

| # | Course | Provider | Enrollments | Rating | Commission | URL Slug |
|---|---|---|---|---|---|---|
| 7 | **The Art of Music Production** | Berklee | **120,595** | 4.78 (2,102 reviews) | 20% | `/learn/producing-music` |
| 8 | **Introduction to Ableton Live** | Berklee | **90,052** | 4.78 (1,118 reviews) | 20% | `/learn/ableton-live` |
| 9 | **The Technology of Music Production** | Berklee | 77,047 | 4.81 (1,082 reviews) | 20% | `/learn/technology-of-music-production` |
| 10 | **Music Production** (Specialization) | Berklee | — | **4.8** (1,878 reviews) | **45%** | `/specializations/music-production` |
| 11 | **Electronic Music Production** (Specialization) | Berklee | — | — | **45%** | `/specializations/electronic-music-production` |

**When to surface these:** After users reach difficulty 7+ or have completed 100+ reviews. The pitch: "You can identify chords by ear — now learn to produce music."

**Best bets in this tier:**
- **Music Production Specialization (#10)** — 45% commission, 4.8 rating, 1,878 reviews. The highest-commission option for production-curious users.
- **Art of Music Production (#7)** — 120K enrolled, emphasizes ear training for production. Natural bridge from Chord Reps.
- **Ableton Live (#8)** — 90K enrolled. Concrete tool recommendation. Users searching for "how to make music" land here.

### Ranking: Top 5 Affiliate Link Candidates (Final)

Ranked by **(audience fit × enrollment volume × commission rate)**:

| Rank | Course | Key Metric | Commission | Verdict |
|---|---|---|---|---|
| **#1** | **Berklee Musicianship Specialization** | 112K enrolled, exact skill match | **45%** | Best overall. Highest commission on the most relevant content. |
| **#2** | **Fundamentals of Music Theory (Edinburgh)** | **399K enrolled**, broadest match | 20% | Highest volume. Universal recommendation for any challenge type. |
| **#3** | **Songwriting Specialization (Berklee)** | Natural "next step" for pop progressions | **45%** | High commission, great for pop progression challenges. |
| **#4** | **Music Production Specialization (Berklee)** | 4.8 rating, 1,878 reviews | **45%** | Production pipeline. Best for advanced users. |
| **#5** | **Jazz Improvisation (Berklee/Gary Burton)** | 74K enrolled, 4.81 rating, celebrity instructor | 20% | Strongest for Dorian/jazz content. Gary Burton's name converts. |

### Revenue Estimates: Coursera Affiliate Specifically

Using the commission structure (20% individual, 45% specialization) and Coursera's pricing (~$49/month for specialization access):

| Scenario (MAU) | Users see Coursera link | Click (1.5% CTR) | Sign up (3%) | Revenue/mo |
|---|---|---|---|---|
| 10,000 | 2,000 | 30 | 0.9 | ~$20 |
| 50,000 | 10,000 | 150 | 4.5 | ~$100 |
| 100,000 | 20,000 | 300 | 9 | ~$200 |

*Assumes: 20% of users see a Coursera-specific placement, 1.5% CTR, 3% signup conversion, blended $22 avg commission (mix of 20% and 45% products at ~$49 price point).*

**Coursera's 30-day cookie is a huge advantage over Amazon's 24-hour cookie.** A user who clicks today and signs up 2 weeks later still counts. This makes Coursera potentially the highest-value single affiliate program for Chord Reps.

### Implementation: Content Database Entries

Ready-to-use `ProductRec` entries for the top candidates:

```rust
// #1 — Berklee Musicianship Specialization (45% commission)
// Surface on: any chord/interval/ear-training challenge, difficulty 1-6
ProductRec {
    name: "Berklee Musicianship Course",
    blurb: "Master ear training with Berklee professors",
    url: "https://www.coursera.org/specializations/musicianship-specialization",
    program: "coursera",
}

// #2 — Edinburgh Fundamentals of Music Theory (20% commission)
// Surface on: any challenge type, difficulty 1-4
ProductRec {
    name: "Music Theory Fundamentals (Edinburgh)",
    blurb: "The most popular theory course online — 400K students",
    url: "https://www.coursera.org/learn/edinburgh-music-theory",
    program: "coursera",
}

// #3 — Songwriting Specialization (45% commission)
// Surface on: pop progressions, cadences, form identification
ProductRec {
    name: "Berklee Songwriting Course",
    blurb: "Write songs with the progressions you just identified",
    url: "https://www.coursera.org/specializations/songwriting",
    program: "coursera",
}

// #4 — Music Production Specialization (45% commission)
// Surface on: any challenge, difficulty 7+
ProductRec {
    name: "Berklee Music Production Course",
    blurb: "You can hear it — now learn to produce it",
    url: "https://www.coursera.org/specializations/music-production",
    program: "coursera",
}

// #5 — Jazz Improvisation (20% commission)
// Surface on: Dorian/Mixolydian scale ID, ii-V-I progressions, jazz-related chords
ProductRec {
    name: "Jazz Improvisation with Gary Burton",
    blurb: "Improvise over the chords you just identified",
    url: "https://www.coursera.org/learn/jazz-improvisation",
    program: "coursera",
}

// #6 — Berklee Tensions & Modal Interchange (20% commission)
// Surface on: seventh chord, chord progression at difficulty 5+
ProductRec {
    name: "Advanced Harmony: Tensions & Modal Interchange",
    blurb: "Go deeper — the highest-rated music course on Coursera",
    url: "https://www.coursera.org/learn/musicianship-harmony",
    program: "coursera",
}
```

**Note:** These URLs will need `?utm_source=...` or Impact tracking parameters appended once accepted into the Coursera affiliate program. The Impact dashboard generates the tracking links.

### Why Coursera Should Be Affiliate Program #1 (Not Amazon)

| Factor | Coursera | Amazon |
|---|---|---|
| Commission on best product | **45%** (specializations) | 4.5-7% |
| Cookie duration | **30 days** | 24 hours |
| Avg. ticket price | ~$49/month | ~$25 (book) |
| Avg. commission per conversion | **~$10-22** | ~$1-1.75 |
| Audience fit | Near-perfect | Good but broader |
| Recurring? | Monthly subscription = potential recurring | One-time |

**A single Coursera specialization signup is worth 6-15x a typical Amazon book sale.** The 30-day cookie means users who click today and sign up in two weeks still count. And if they stay subscribed, the recurring nature of the subscription means your one click keeps earning.

**Recommendation: Sign up for the Coursera affiliate program (via Impact) before Amazon Associates.** Amazon is still worth having for books and instruments, but Coursera is the higher-value program for this specific audience.
